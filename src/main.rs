extern crate byteorder;
extern crate stats;
extern crate iron;
#[macro_use]
extern crate router;
extern crate staticfile;
mod lib;

use std::fs::File;
use std::io::*;
use std::iter::FromIterator;
use iron::{Iron, Request, Response, IronResult};
use iron::status;
use iron::headers;
use iron::mime::Mime;
use router::{Router};

fn main() {
    fn request_handler(req: &mut Request) -> IronResult<Response>{
        let mut buffer: Vec<u8> = Vec::new();
        req.body.read_to_end(&mut buffer);
        let input_charts = lib::btsf::read_btsf_file(&mut Cursor::new(&mut buffer));
        if input_charts.len() != 1 {
            return Ok(Response::with((status::BadRequest, "Please send a BTSF file with precisely one chart in it")))
        }
        let result = lib::correlate::correlate(&input_charts[0]);
        
        let mut response_data: Vec<u8> = Vec::new();
        lib::btsf::write_btsf_file(&result, &mut response_data);

        let contentType: Mime = "application/octet-stream".parse().unwrap();
        return Ok(Response::with((status::Ok, contentType, response_data)));
    };

    let router = router!(
        post "/find" => request_handler
            );


    let mut possibilities = lib::btsf::read_btsf_file(&mut File::open("./btsf/mortality.btsf").unwrap());
    for i in 0..possibilities.len() - 1 {
        possibilities.pop();
    }
    lib::btsf::write_btsf_file(&possibilities, &mut File::create("./test.btsf").unwrap());

    Iron::new(router).http("localhost:8080").unwrap();
}
