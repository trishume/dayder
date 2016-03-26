use std::iter;
use lib::btsf::*;

pub fn pairinate(a: &BinaryTimeSeries, b: &BinaryTimeSeries)
                 -> (Vec<f32>, Vec<f32>){
    if a.data.len() != b.data.len(){
        panic!("AHHHH! Everything is broken. Those datas don't match.");
    }

    return (a.data.iter().map(|p| p.val).collect(), b.data.iter().map(|p| p.val).collect());
}
