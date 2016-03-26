extern crate byteorder;
extern crate stats;
extern crate iron;
#[macro_use]
extern crate router;
extern crate staticfile;
mod lib;

use std::fs::File;

use iron::{Iron, Request, Response, IronResult};
use iron::status;
use router::{Router};

fn main() {
    fn request_handler(req: &mut Request) -> IronResult<Response>{
        
    }
}
