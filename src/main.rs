extern crate byteorder;
extern crate stats;
extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate persistent;
mod lib;

use std::fs::File;
use std::io::*;
use std::iter::FromIterator;
use iron::{Iron, Request, Response, IronResult, Plugin};
use iron::typemap::Key;
use iron::status;
use iron::headers;
use iron::mime::Mime;
use mount::Mount;
use staticfile::Static;
use std::path::Path;

#[derive(Copy, Clone)]
pub struct DataSets;
impl Key for DataSets { type Value = Vec<lib::btsf::BinaryTimeSeries>; }

fn main() {
    fn request_handler(req: &mut Request) -> IronResult<Response>{
        let mut buffer: Vec<u8> = Vec::new();
        req.body.read_to_end(&mut buffer);
        let input_charts = lib::btsf::read_btsf_file(&mut Cursor::new(&mut buffer));
        if input_charts.len() != 1 {
            return Ok(Response::with((status::BadRequest, "Please send a BTSF file with precisely one chart in it")))
        }

        let data_sets = req.get::<persistent::Read<DataSets>>().unwrap();
        let result = lib::correlate::correlate(&input_charts[0], &data_sets[..]);

        let mut response_data: Vec<u8> = Vec::new();
        lib::btsf::write_correlated_btsf_file(&result[..], &mut response_data);

        let contentType: Mime = "application/octet-stream".parse().unwrap();
        return Ok(Response::with((status::Ok, contentType, response_data)));
    };

    let mut mount = Mount::new();
    mount
        .mount("/", Static::new(Path::new("./public")))
        .mount("/find", request_handler);

    let data_sets = lib::btsf::read_btsf_file(&mut File::open("./btsf/mortality.btsf").unwrap());
    let mut chain = iron::Chain::new(mount);
    chain.link_before(persistent::Read::<DataSets>::one(data_sets));

    Iron::new(chain).http("localhost:8080").unwrap();
}
