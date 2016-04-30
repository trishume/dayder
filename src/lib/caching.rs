extern crate lru_time_cache;
use lib::btsf::*;
use lib::correlate::*;

pub struct CorrelationCache {
  lru: lru_time_cache::LruCache<BinaryTimeSeries, Vec<f32>>
}

impl<'a> CorrelationCache {
  pub fn new() -> CorrelationCache {
    CorrelationCache {lru: lru_time_cache::LruCache::with_capacity(5) }
  }
  pub fn correlate(&'a mut self, data: &BinaryTimeSeries, possibilities: &'static [BinaryTimeSeries]) -> Vec<f32> {
    if let Some(correlations) = self.lru.get(data) {
      return correlations.clone();
    }

    let correlations = correlate(data, possibilities);
    self.lru.insert(data.clone(), correlations);
    let corr_ref = self.lru.get(data).unwrap();
    corr_ref.clone()
  }
}
