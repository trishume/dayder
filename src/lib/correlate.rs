extern crate rayon;
use self::rayon::prelude::*;
use lib::btsf::*;
use lib::stats::*;

const QUERY_SERIES_SIZE_THRESH: usize = 256;

pub fn correlate(data: &BinaryTimeSeries, possibilities: &[BinaryTimeSeries]) -> Vec<f32> {
    let query_series: BinaryTimeSeries = if data.data.len() < QUERY_SERIES_SIZE_THRESH {
        data.clone()
    } else {
        let skip: usize = data.data.len() / QUERY_SERIES_SIZE_THRESH;
        let mut downsampled = Vec::with_capacity(data.data.len() / skip);
        for i in (0..(data.data.len())).filter(|x| x % skip == 0) {
            downsampled.push(data.data[i].clone());
        }
        BinaryTimeSeries {
            name: data.name.clone(),
            data: downsampled,
        }
    };

    let mut correlations: Vec<f32> = Vec::new();
    possibilities
        .par_iter()
        .map(|poss| {
            if let Some((xs, ys)) = pairinate(&query_series, poss) {
                pearson_correlation_coefficient(&xs, &ys) as f32
            } else {
                0.0
            }
        })
        .collect_into(&mut correlations);

    println!("Found correlations for '{}'", data.name);

    return correlations;
}
