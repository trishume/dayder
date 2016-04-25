extern crate fst;
#[macro_use]
extern crate lazy_static;
extern crate dayder;

use std::fs::File;
use std::io::{BufWriter, BufReader};
use std::path::Path;
use std::ascii::AsciiExt;
use fst::{IntoStreamer, Streamer, Map, MapBuilder, Result};

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

fn build_index() -> Result<()> {
    // This is where we'll write our map to.
    let mut wtr = BufWriter::new(try!(File::create("btsf/index.fst")));

    // Create a builder that can be used to insert new key-value pairs.
    let mut build = try!(MapBuilder::new(wtr));
    build.insert("bruce", 1).unwrap();
    build.insert("clarence", 2).unwrap();
    build.insert("stevie", 3).unwrap();

    // Finish construction of the map and flush its contents to disk.
    try!(build.finish());

    return Ok(());
}

fn main() {
    build_index().unwrap();
}
