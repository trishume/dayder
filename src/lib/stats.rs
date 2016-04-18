use lib::btsf::*;

const MIN_OVERLAP : usize = 5;

pub fn pearson_correlation_coefficient(xs: &Vec<f32>, ys: &Vec<f32>) -> f64{
    let n = xs.len();

    let mut sum_xy: f64 = 0.0;
    let mut sum_xx: f64 = 0.0;
    let mut sum_yy: f64 = 0.0;
    let mut sum_x: f64 = 0.0;
    let mut sum_y: f64 = 0.0;
    for i in 0..n {
        sum_xy += (xs[i] as f64) * (ys[i] as f64);
        sum_xx += (xs[i] as f64) * (xs[i] as f64);
        sum_yy += (ys[i] as f64) * (ys[i] as f64);
        sum_x += xs[i] as f64;
        sum_y += ys[i] as f64;
    }
    let x_avg = sum_x / (n as f64);
    let y_avg = sum_y / (n as f64);

    let denom = (sum_xx - (n as f64) * (x_avg*x_avg)).sqrt() * (sum_yy - (n as f64) * (y_avg*y_avg)).sqrt();

    // this is to guard against series that are flat lines and thus might correlate with anything, added when needed with a previous formula
    // TODO this may not fire due to floating point error, it may not even be necessary anymore
    if denom == 0.0 {
        return 0.0;
    }

    let numerator = sum_xy - (sum_x * y_avg);
    let final_val = numerator / denom;

    if final_val.is_nan() {
        return 0.0;
    }

    return final_val;
}

pub fn pairinate(base: &BinaryTimeSeries, other: &BinaryTimeSeries)
                 -> Option<(Vec<f32>, Vec<f32>)>{
    if base.data.len() < MIN_OVERLAP || other.data.len() < MIN_OVERLAP { return None; }
    let (bs, os) = interpolate(&base.data[..], &other.data[..]);
    if bs.len() < MIN_OVERLAP { return None; }
    return Some((bs, os));
}

// interpolate a point from 'other' for as many points as possible in 'base'
fn interpolate(base: &[Point], other: &[Point]) -> (Vec<f32>, Vec<f32>) {
    assert!(other.len() > 0);
    let mut base_data = Vec::<f32>::new();
    let mut other_data = Vec::<f32>::new();

    // TODO: early exit on cases where there is bound to be nothing productive
    let mut search_index = 0;
    for data_point in base {
        // advance search_index as far as possible without putting it past the base data point
        // condition: not already past it or at it, not the last point, and the next point isn't past it
        while other[search_index].t < data_point.t && search_index < (other.len() - 1) && other[search_index+1].t <= data_point.t {
            search_index += 1;
        }

        if other[search_index].t == data_point.t{
            other_data.push(other[search_index].val);
            base_data.push(data_point.val);
        } else if search_index < (other.len() - 1) && other[search_index].t < data_point.t {
            assert!(other[search_index+1].t >= data_point.t);
            // We have to interpolate data
            let time_offset = data_point.t - other[search_index].t;
            let point_offset = other[search_index+1].t - other[search_index].t;
            let interpolation_ratio: f32= (time_offset as f32) / (point_offset as f32);

            let interpolated_data_point = (other[search_index].val * (1.0 - interpolation_ratio)) +
                (other[search_index+1].val * interpolation_ratio);

            other_data.push(interpolated_data_point);
            base_data.push(data_point.val);
        }
    }

    return (base_data, other_data);
}
