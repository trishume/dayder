use lib::btsf::*;
use stats;

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

    return rval * lval;
}

pub fn pairinate(a: &BinaryTimeSeries, b: &BinaryTimeSeries)
                 -> (Vec<f32>, Vec<f32>){
    if a.data.len() != b.data.len(){
        panic!("AHHHH! Everything is broken. Those datas don't match.");
    }

    return (a.data.iter().map(|p| p.val).collect(), b.data.iter().map(|p| p.val).collect());
}
