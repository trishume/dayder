extern crate fst;
#[macro_use]
extern crate lazy_static;
extern crate dayder;

use std::fs::File;
use std::io::{BufWriter, BufReader};
use std::path::Path;
use std::ascii::AsciiExt;
use fst::{MapBuilder, Result};

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

    // Sort the data in preparation of adding it to the map
    let mut data : Vec<(usize, String)> = DATA_SETS.iter().map(|x| x.name.to_ascii_lowercase()).enumerate().collect();
    data.sort_by(|a,b| a.1.cmp(&b.1));
    println!("Sorted data, creating map file...");

    // Create a builder that can be used to insert new key-value pairs.
    let mut build = try!(MapBuilder::new(wtr));

    let mut last_s : String = String::from("");
    for (i,s) in data.into_iter() {
        // println!("inserting {:?} at {} {:?}", s, i, last_s);
        if s != last_s {
            build.insert(&s, i as u64).unwrap();
        }
        last_s = s;
    }

    // Finish construction of the map and flush its contents to disk.
    try!(build.finish());

    return Ok(());
}

fn main() {
    build_index().unwrap();
}
