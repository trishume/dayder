extern crate byteorder;
extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate persistent;
#[macro_use]
extern crate lazy_static;
mod lib;

use std::fs::File;
use std::io::*;
use iron::{Iron, Request, Response, IronResult, IronError, Plugin};
use iron::typemap::Key;
use iron::status;
use iron::mime::Mime;
use mount::Mount;
use staticfile::Static;
use std::path::Path;
use lib::caching::CorrelationCache;
use std::ascii::AsciiExt;

const PER_PAGE : usize = 100;

#[derive(Copy, Clone)]
pub struct CorrCache;
impl Key for CorrCache { type Value = CorrelationCache; }

lazy_static! {
    static ref DATA_SETS: Vec<lib::btsf::BinaryTimeSeries> = {
        let mut all_data = Vec::new();
        lib::btsf::read_btsf_file(&mut File::open("./btsf/mortality.btsf").unwrap(), &mut all_data).unwrap();
        lib::btsf::read_btsf_file(&mut File::open("./btsf/canada_gdp.btsf").unwrap(), &mut all_data).unwrap();
        if Path::new("./btsf/fred.btsf").exists() {
            lib::btsf::read_btsf_file(&mut BufReader::new(File::open("./btsf/fred.btsf").unwrap()), &mut all_data).unwrap();
        }
        all_data
    };
}

fn main() {
    fn find_handler(req: &mut Request) -> IronResult<Response>{
        let mut buffer: Vec<u8> = Vec::new();
        req.body.read_to_end(&mut buffer).unwrap();
        let mut input_charts = Vec::with_capacity(1);
        if let Err(e) = lib::btsf::read_btsf_file(&mut Cursor::new(&mut buffer), &mut input_charts) {
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

        // TODO: URL decode query so you can search for spaces and it doesn't try to search for '%20'
        // TODO: Faster filtering algorithm
        // TODO: fuzzy matching
        let filter : String = req.url.query.as_ref().map(|x| &**x).unwrap_or("").to_ascii_lowercase();
        let filtered : Vec<lib::btsf::CorrelatedTimeSeries> = result.into_iter().filter(|s| s.series.name.to_ascii_lowercase().contains(&filter)).take(PER_PAGE).collect();

        let mut response_data: Vec<u8> = Vec::new();
        if let Err(e) = lib::btsf::write_correlated_btsf_file(&filtered[..], &mut response_data) {
            return Err(IronError::new(e, status::InternalServerError));
        }

        let content_type: Mime = "application/octet-stream".parse().unwrap();
        return Ok(Response::with((status::Ok, content_type, response_data)));
    };

    fn raw_handler(req: &mut Request) -> IronResult<Response>{
        let filter : String = req.url.query.as_ref().map(|x| &**x).unwrap_or("").to_ascii_lowercase();
        let result: Vec<&lib::btsf::BinaryTimeSeries> = DATA_SETS.iter().filter(|s| s.name.to_ascii_lowercase().contains(&filter)).take(PER_PAGE).collect();

        let mut response_data: Vec<u8> = Vec::new();
        if let Err(e) = lib::btsf::write_btsf_file(&result[..], &mut response_data) {
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
