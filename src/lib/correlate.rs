use lib::btsf::*;
use lib::stats::*;
use std::fs::File;

pub fn correlate(data: &BinaryTimeSeries) -> Vec<BinaryTimeSeries>{
    let mut possibilities = read_btsf_file(&mut File::open("./btsf/mortality.btsf").unwrap());

    for i in 0..possibilities.len() {
        let (xs, ys) = pairinate(&possibilities[i], &data);
        possibilities[i].correlation = pearson_correlation_coefficient(&xs, &ys) as f32;
    }

    possibilities.sort_by(|btsa, btsb| btsa.correlation.partial_cmp(&btsa.correlation).unwrap());

    println!("{}", possibilities[0].correlation);

    return possibilities;
}
