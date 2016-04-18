use lib::btsf::*;
use lib::stats::*;

const QUERY_SERIES_SIZE_THRESH : usize = 256;

pub fn correlate(data: &BinaryTimeSeries, possibilities: &'static [BinaryTimeSeries]) -> Vec<CorrelatedTimeSeries<'static>>{
    // let mut possibilities = read_btsf_file(&mut File::open("./btsf/mortality.btsf").unwrap());
    let mut correlations = Vec::new();

    let query_series : BinaryTimeSeries = if data.data.len() < QUERY_SERIES_SIZE_THRESH {
        data.clone()
    } else {
        let skip : usize = data.data.len() / QUERY_SERIES_SIZE_THRESH;
        let mut downsampled = Vec::with_capacity(data.data.len() / skip);
        for i in (0..(data.data.len())).filter(|x| x % skip == 0) {
            downsampled.push(data.data[i].clone());
        }
        BinaryTimeSeries {
            name: data.name.clone(),
            data: downsampled
        }
    };

    for poss in possibilities {
        if let Some((xs, ys)) = pairinate(&query_series, poss) {
            correlations.push(CorrelatedTimeSeries{
                series: poss,
                correlation: pearson_correlation_coefficient(&xs, &ys) as f32
            });
        }
    }

    correlations.sort_by(|btsa, btsb| btsb.correlation.partial_cmp(&btsa.correlation).unwrap());

    println!("{}", correlations[0].correlation);

    return correlations;
}
