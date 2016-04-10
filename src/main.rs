extern crate byteorder;
extern crate stats;
extern crate iron;
#[macro_use]
extern crate router;
extern crate staticfile;
extern crate mount;
mod lib;

use std::fs::File;
use std::io::*;
use std::iter::FromIterator;
use iron::{Iron, Request, Response, IronResult};
use iron::status;
use iron::headers;
use iron::mime::Mime;
use router::{Router};
use mount::Mount;
use staticfile::Static;
use std::path::Path;

fn main() {
    fn request_handler(req: &mut Request) -> IronResult<Response>{
        let mut buffer: Vec<u8> = Vec::new();
        req.body.read_to_end(&mut buffer);
        let input_charts = lib::btsf::read_btsf_file(&mut Cursor::new(&mut buffer));
        if input_charts.len() != 1 {
            return Ok(Response::with((status::BadRequest, "Please send a BTSF file with precisely one chart in it")))
        }

        let data_sets = lib::btsf::read_btsf_file(&mut File::open("./btsf/mortality.btsf").unwrap());
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

    Iron::new(mount).http("localhost:8080").unwrap();
}
