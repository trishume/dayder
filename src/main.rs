extern crate byteorder;
mod lib;

use std::fs::File;

fn main() {
    let datas = lib::btsf::read_btsf_file(&mut File::open("./btsf/mortality.btsf").unwrap());

    let mut outdata = File::create("output.btsf").unwrap();
    lib::btsf::write_btsf_file(&datas, &mut outdata);
}
