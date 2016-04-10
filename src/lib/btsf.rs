use std::io::*;
use std::str;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub struct Point {
    pub t: i32,
    pub val: f32
}

pub struct BinaryTimeSeries {
    pub name: String,
    pub data: Vec<Point>
}

pub struct CorrelatedTimeSeries<'a> {
    pub series: &'a BinaryTimeSeries,
    pub correlation: f32
}

pub fn read_btsf_file<T: Read + Seek>(f: &mut T) -> Result<Vec<BinaryTimeSeries>> {
    try!(f.seek(SeekFrom::Start(0)));
    let mut series = Vec::<BinaryTimeSeries>::new();
    let version = try!(f.read_u32::<LittleEndian>());
    let file_header_len = try!(f.read_u32::<LittleEndian>());
    let rec_header_len = try!(f.read_u32::<LittleEndian>());
    let num_records = try!(f.read_u32::<LittleEndian>());

    try!(f.seek(SeekFrom::Start(file_header_len as u64)));

    println!("Version Number: {}", version);
    println!("File Header Len: {}", file_header_len);
    println!("Rec Header Len: {}", rec_header_len);
    println!("Num Records: {}", num_records);

    for _ in 0..num_records {
        let n = try!(f.read_u32::<LittleEndian>());
        let name_length = try!(f.read_u32::<LittleEndian>());
        try!(f.seek(SeekFrom::Current((rec_header_len - 8) as i64))); // Skip padding bytes;

        let mut buffer = [0; 1024];
        try!(f.take(name_length as u64).read(&mut buffer));

        let name = match str::from_utf8(&buffer[0..(name_length as usize)]) {
            Ok(s) => s,
            Err(_) => return Err(Error::new(ErrorKind::InvalidInput, "Invalid input UTF-8")),
        };

        let mut data = Vec::<Point>::new();

        for _ in 0..n {
            let t = try!(f.read_i32::<LittleEndian>());
            let d = try!(f.read_f32::<LittleEndian>());
            data.push(Point{t: t, val: d});
        }
        series.push(BinaryTimeSeries{
            name: String::from(name),
            data: data
        })
    }

    return Ok(series);
}

pub fn write_correlated_btsf_file<T: Write>(data: &[CorrelatedTimeSeries], output: &mut T) -> Result<()> {
    // Version Header, File Header Len, Rec Header Len
    try!(output.write_u32::<LittleEndian>(2));
    try!(output.write_u32::<LittleEndian>(16));
    try!(output.write_u32::<LittleEndian>(12));

    try!(output.write_u32::<LittleEndian>(data.len() as u32));

    for corr_record in data {
        let record = corr_record.series;
        try!(output.write_u32::<LittleEndian>(record.data.len() as u32));
        try!(output.write_u32::<LittleEndian>(record.name.len() as u32));
        try!(output.write_f32::<LittleEndian>(corr_record.correlation as f32));
        try!(output.write(&record.name.as_bytes()));
        for i in 0..record.data.len() {
            try!(output.write_i32::<LittleEndian>(record.data[i].t));
            try!(output.write_f32::<LittleEndian>(record.data[i].val));
        }
    }
    Ok(())
}
