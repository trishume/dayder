use lib::btsf::*;
use lib::stats::*;

pub fn correlate(data: &BinaryTimeSeries, possibilities: &'static [BinaryTimeSeries]) -> Vec<CorrelatedTimeSeries<'static>>{
    // let mut possibilities = read_btsf_file(&mut File::open("./btsf/mortality.btsf").unwrap());
    let mut correlations = Vec::new();

    for poss in possibilities {
        let (xs, ys) = pairinate(poss, &data);
        correlations.push(CorrelatedTimeSeries{
          series: poss,
          correlation: pearson_correlation_coefficient(&xs, &ys) as f32
        });
    }

    correlations.sort_by(|btsa, btsb| btsb.correlation.partial_cmp(&btsa.correlation).unwrap());

    println!("{}", correlations[0].correlation);

    return correlations;
}
