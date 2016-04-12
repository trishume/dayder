use lib::btsf::*;
use stats;

macro_rules! try_opt {
    ($expr:expr) => (match $expr {
        ::std::option::Option::Some(val) => val,
        ::std::option::Option::None => return None
    })
}

pub fn pearson_correlation_coefficient(xs: &Vec<f32>, ys: &Vec<f32>) -> f64{
    let x_avg = stats::mean(xs.iter().map(|x| x.clone()));
    let x_dev = stats::stddev(xs.iter().map(|x| x.clone()));
    let y_avg = stats::mean(ys.iter().map(|y| y.clone()));
    let y_dev = stats::stddev(ys.iter().map(|y| y.clone()));
    let n = xs.len();

    let mut rval: f64 = 0.0;
    for i in 0..n{
        rval += (((xs[i] as f64) - x_avg) / x_dev) * (((ys[i] as f64) - y_avg) / y_dev);
    }

    let lval: f64 = 1.0 / ((n as f64) - 1.0);

    let final_val = rval * lval;

    if final_val.is_nan() {
        return 0.0;
    }

    return final_val;
}

pub fn pairinate(a: &BinaryTimeSeries, b: &BinaryTimeSeries)
                 -> Option<(Vec<f32>, Vec<f32>)>{

    let a_start = try_opt!(get_start_index(a, b));
    let b_start = try_opt!(get_start_index(b, a));
    let a_end = try_opt!(get_end_index(a, b));
    let b_end = try_opt!(get_end_index(b, a));

    if (a_end - a_start) >= (b_end - b_start) {
        // Only one point of overlap, cannot correlate
        if a_end - a_start == 0 {return None};
        return Some(interpolate(&a.data[a_start .. a_end], &b.data[..]));
    }else{
        return Some(interpolate(&b.data[b_start .. b_end], &a.data[..]));
    }
}

fn get_start_index(series: &BinaryTimeSeries, target: &BinaryTimeSeries) -> Option<usize>{
    let mut start_index = 0;
    while series.data[start_index].t < target.data[0].t{
        start_index += 1;
        if start_index >= series.data.len() {
            return None;
        }
    }
    return Some(start_index);
}

fn get_end_index(series: &BinaryTimeSeries, target: &BinaryTimeSeries) -> Option<usize>{
    let mut end_index = series.data.len() - 1;

    while series.data[end_index].t < target.data[target.data.len() - 1].t{
        if end_index == 0{
            return None;
        }
        end_index -= 1;
    }
    return Some(end_index)
}

fn interpolate(base: &[Point], other: &[Point]) -> (Vec<f32>, Vec<f32>){
    let mut search_index = 0;

    let mut base_data = Vec::<f32>::new();
    let mut other_data = Vec::<f32>::new();

    for data_point in base {
        base_data.push(data_point.val);
        while other[search_index].t < data_point.t{
            search_index += 1;
        }
        if other[search_index].t == data_point.t{
            other_data.push(other[search_index].val);
        }else{
            // We have to interpolate data
            let time_offset = data_point.t - other[search_index - 1].t;
            let point_offset = other[search_index - 1].t - other[search_index].t;
            let interpolation_ratio: f32= (time_offset as f32) / (point_offset as f32);

            let interpolated_data_point = (other[search_index - 1].val * (1.0 - interpolation_ratio)) +
                (other[search_index].val * interpolation_ratio);

            other_data.push(interpolated_data_point);
        }
    }

    return (base_data, other_data);
}
