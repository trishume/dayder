extern crate byteorder;
extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate persistent;
extern crate url;
extern crate memmem;
extern crate lazysort;
#[macro_use]
extern crate lazy_static;
extern crate hprof;
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
use url::percent_encoding::lossy_utf8_percent_decode;
use memmem::{Searcher, TwoWaySearcher};
use lazysort::SortedPartial;

const PER_PAGE : usize = 100;

#[derive(Copy, Clone)]
pub struct CorrCache;
impl Key for CorrCache { type Value = CorrelationCache; }

lazy_static! {
    static ref DATA_SETS: Vec<lib::btsf::BinaryTimeSeries> = {
        let mut all_data = Vec::new();
        lib::btsf::read_btsf_file(&mut File::open("./btsf/mortality.btsf").unwrap(), &mut all_data).unwrap();
        lib::btsf::read_btsf_file(&mut File::open("./btsf/canada_gdp.btsf").unwrap(), &mut all_data).unwrap();
        if Path::new("./btsf/fred-small.btsf").exists() {
            lib::btsf::read_btsf_file(&mut BufReader::new(File::open("./btsf/fred-small.btsf").unwrap()), &mut all_data).unwrap();
        }
        all_data
    };
    static ref SET_NAMES: Vec<String> = {
        DATA_SETS.iter().map(|s| s.name.to_ascii_lowercase()).collect()
    };
}

fn filter_text(req: &Request) -> String {
    let filter_query = req.url.query.as_ref().map(|x| &**x).unwrap_or("");
    lossy_utf8_percent_decode(filter_query.as_ref()).to_ascii_lowercase()
}

fn main() {
    fn find_handler(req: &mut Request) -> IronResult<Response>{
        let p = hprof::Profiler::new("find handler");
        p.start_frame();
        p.enter_noguard("request");

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
            let _g = p.enter("correlating");
            let mutex = req.get::<persistent::Write<CorrCache>>().unwrap();
            let mut cache = mutex.lock().unwrap();
            cache.correlate(&input_charts[0], &DATA_SETS[..])
        };

        // TODO: fuzzy matching
        let filter : String = filter_text(req);
        p.enter_noguard("filtering");
        let filtered: Vec<lib::btsf::CorrelatedTimeSeries> = if filter != "" {
            let search = TwoWaySearcher::new(filter.as_bytes());
            SET_NAMES.iter()
                     .enumerate()
                     .filter(|&(_,s)| search.search_in(s.as_bytes()).is_some())
                     .map(|(i,_)| lib::btsf::CorrelatedTimeSeries { series: &DATA_SETS[i], correlation: result[i]})
                     .filter(|corr_series| corr_series.correlation != 0.0)
                     .sorted_partial(false)
                     .take(PER_PAGE)
                     .collect()
        } else {
            result.iter()
                  .enumerate()
                  .map(|(i,c)| lib::btsf::CorrelatedTimeSeries { series: &DATA_SETS[i], correlation: c.clone()})
                  .filter(|corr_series| corr_series.correlation != 0.0)
                  .sorted_partial(false)
                  .take(PER_PAGE)
                  .collect()
        };
        p.leave();

        let mut response_data: Vec<u8> = Vec::new();
        if let Err(e) = lib::btsf::write_correlated_btsf_file(&filtered[..], &mut response_data) {
            return Err(IronError::new(e, status::InternalServerError));
        }

        p.leave();
        p.end_frame();
        println!("Find handler, filter: '{}', corr: '{}'", filter, input_charts[0].name);
        p.print_timing();

        let content_type: Mime = "application/octet-stream".parse().unwrap();
        return Ok(Response::with((status::Ok, content_type, response_data)));
    };

    fn raw_handler(req: &mut Request) -> IronResult<Response>{
        let p = hprof::Profiler::new("raw handler");
        p.start_frame();
        p.enter_noguard("request");

        let filter : String = filter_text(req);
        p.enter_noguard("filtering");
        let result: Vec<&lib::btsf::BinaryTimeSeries> = if filter != "" {
            let search = TwoWaySearcher::new(filter.as_bytes());
            SET_NAMES.iter().enumerate().filter(|&(_,s)| search.search_in(s.as_bytes()).is_some()).take(PER_PAGE).map(|(i,_)| &DATA_SETS[i]).collect()
        } else {
            DATA_SETS.iter().take(PER_PAGE).collect()
        };
        p.leave();

        let mut response_data: Vec<u8> = Vec::new();
        if let Err(e) = lib::btsf::write_btsf_file(&result[..], &mut response_data) {
            return Err(IronError::new(e, status::InternalServerError));
        }

        p.leave();
        p.end_frame();
        println!("Raw handler, query: '{}'", filter);
        p.print_timing();

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
    let total_size : usize = SET_NAMES.iter().map(|x| x.len()).fold(0, |a,b| a+b);
    println!("Filtering data has size {}", total_size);

    Iron::new(chain).http("localhost:8080").unwrap();
}
