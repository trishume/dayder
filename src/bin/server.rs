extern crate byteorder;
extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate persistent;
extern crate url;
extern crate fst;
#[macro_use]
extern crate lazy_static;
extern crate dayder;

use std::fs::File;
use std::io::*;
use iron::{Iron, Request, Response, IronResult, IronError, Plugin};
use iron::typemap::Key;
use iron::status;
use iron::mime::Mime;
use mount::Mount;
use staticfile::Static;
use std::path::Path;
use dayder::caching::CorrelationCache;
use std::ascii::AsciiExt;
use url::percent_encoding::lossy_utf8_percent_decode;

const PER_PAGE : usize = 100;

#[derive(Copy, Clone)]
pub struct CorrCache;
impl Key for CorrCache { type Value = CorrelationCache; }

lazy_static! {
    static ref DATA_SETS: Vec<dayder::btsf::BinaryTimeSeries> = {
        let mut all_data = Vec::new();
        dayder::btsf::read_btsf_file(&mut File::open("./btsf/mortality.btsf").unwrap(), &mut all_data).unwrap();
        dayder::btsf::read_btsf_file(&mut File::open("./btsf/canada_gdp.btsf").unwrap(), &mut all_data).unwrap();
        if Path::new("./btsf/fred-small.btsf").exists() {
            dayder::btsf::read_btsf_file(&mut BufReader::new(File::open("./btsf/fred-small.btsf").unwrap()), &mut all_data).unwrap();
        }
        all_data
    };
}

fn filter_text(req: &Request) -> String {
    let filter_query = req.url.query.as_ref().map(|x| &**x).unwrap_or("");
    lossy_utf8_percent_decode(filter_query.as_ref()).to_ascii_lowercase()
}

fn main() {
    fn find_handler(req: &mut Request) -> IronResult<Response>{
        let mut buffer: Vec<u8> = Vec::new();
        req.body.read_to_end(&mut buffer).unwrap();
        let mut input_charts = Vec::with_capacity(1);
        if let Err(e) = dayder::btsf::read_btsf_file(&mut Cursor::new(&mut buffer), &mut input_charts) {
            return Err(IronError::new(e, status::BadRequest))
        };
        if input_charts.len() != 1 {
            return Ok(Response::with((status::BadRequest, "Please send a BTSF file with precisely one chart in it")))
        }


        let result = {
            let mutex = req.get::<persistent::Write<CorrCache>>().unwrap();
            let mut cache = mutex.lock().unwrap();
            cache.correlate(&input_charts[0], &DATA_SETS[..])
        };

        // TODO: Faster filtering algorithm
        // TODO: fuzzy matching
        let filter : String = filter_text(req);
        let filtered : Vec<dayder::btsf::CorrelatedTimeSeries> = result.into_iter().filter(|s| s.series.name.to_ascii_lowercase().contains(&filter)).take(PER_PAGE).collect();

        let mut response_data: Vec<u8> = Vec::new();
        if let Err(e) = dayder::btsf::write_correlated_btsf_file(&filtered[..], &mut response_data) {
            return Err(IronError::new(e, status::InternalServerError));
        }

        let content_type: Mime = "application/octet-stream".parse().unwrap();
        return Ok(Response::with((status::Ok, content_type, response_data)));
    };

    fn raw_handler(req: &mut Request) -> IronResult<Response>{
        let filter : String = filter_text(req);
        let result: Vec<&dayder::btsf::BinaryTimeSeries> = DATA_SETS.iter().filter(|s| s.name.to_ascii_lowercase().contains(&filter)).take(PER_PAGE).collect();

        let mut response_data: Vec<u8> = Vec::new();
        if let Err(e) = dayder::btsf::write_btsf_file(&result[..], &mut response_data) {
            return Err(IronError::new(e, status::InternalServerError));
        }

        let content_type: Mime = "application/octet-stream".parse().unwrap();
        return Ok(Response::with((status::Ok, content_type, response_data)));
    };

    let mut mount = Mount::new();
    mount
        .mount("/", Static::new(Path::new("./public")))
        .mount("/raw", raw_handler)
        .mount("/find", find_handler);

    let corr_cache = CorrelationCache::new();
    let mut chain = iron::Chain::new(mount);
    chain.link_before(persistent::Write::<CorrCache>::one(corr_cache));

    // This print statement is partially just to invoke the lazy static
    println!("Serving up {} data sets!", DATA_SETS.len());
    Iron::new(chain).http("localhost:8080").unwrap();
}
