use std::fs::File;
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

pub fn read_btsf_file<T: Read + Seek>(f: &mut T) -> Vec<BinaryTimeSeries> {
    f.seek(SeekFrom::Start(0)).unwrap();
    let mut series = Vec::<BinaryTimeSeries>::new();
    let version = f.read_u32::<LittleEndian>().unwrap();
    let file_header_len = f.read_u32::<LittleEndian>().unwrap();
    let rec_header_len = f.read_u32::<LittleEndian>().unwrap();
    let num_records = f.read_u32::<LittleEndian>().unwrap();

    f.seek(SeekFrom::Start(file_header_len as u64)).unwrap();

    println!("Version Number: {}", version);
    println!("File Header Len: {}", file_header_len);
    println!("Rec Header Len: {}", rec_header_len);
    println!("Num Records: {}", num_records);

    for i in 0..num_records {
        let n = f.read_u32::<LittleEndian>().unwrap();
        let name_length = f.read_u32::<LittleEndian>().unwrap();
        f.seek(SeekFrom::Current((rec_header_len - 8) as i64)).unwrap(); // Skip padding bytes;

        let mut buffer = [0; 1024];
        f.take(name_length as u64).read(&mut buffer).unwrap();

        let name = str::from_utf8(&buffer[0..(name_length as usize)]).unwrap();

        let mut data = Vec::<Point>::new();

        for j in 0..n {
            let t = f.read_i32::<LittleEndian>().unwrap();
            let d = f.read_f32::<LittleEndian>().unwrap();
            data.push(Point{t: t, val: d});
        }
        series.push(BinaryTimeSeries{
            name: String::from(name),
            data: data
        })
    }

    return series;
}

pub fn write_correlated_btsf_file<T: Write>(data: &[CorrelatedTimeSeries], output: &mut T){
    // Version Header, File Header Len, Rec Header Len
    output.write_u32::<LittleEndian>(2).unwrap();
    output.write_u32::<LittleEndian>(16).unwrap();
    output.write_u32::<LittleEndian>(12).unwrap();

    output.write_u32::<LittleEndian>(data.len() as u32);

    for corr_record in data {
        let record = corr_record.series;
        output.write_u32::<LittleEndian>(record.data.len() as u32);
        output.write_u32::<LittleEndian>(record.name.len() as u32);
        output.write_f32::<LittleEndian>(corr_record.correlation as f32);
        output.write(&record.name.as_bytes());
        for i in 0..record.data.len() {
            output.write_i32::<LittleEndian>(record.data[i].t);
            output.write_f32::<LittleEndian>(record.data[i].val);
        }
    }
}
