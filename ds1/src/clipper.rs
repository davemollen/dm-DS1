use std::simd::f32x16;
mod oversample;
use crate::shared::lowpass_filter::LowpassFilter;
use oversample::Oversample;

pub struct Clipper {
  lowpass_filter: LowpassFilter,
  oversample: Oversample<f32x16>,
}

impl Clipper {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      lowpass_filter: LowpassFilter::new(sample_rate),
      oversample: Oversample::new(),
    }
  }

  pub fn process(&mut self, input: f32) -> f32 {
    let filtered = self.lowpass_filter.process(input, 7230.);

    self.oversample.process(filtered, |x| {
      let n = 6.;
      let mapped = f32x16::to_array(x).map(|x| x / (1. + x.abs()).powf(n).powf(1. / n));
      f32x16::from_array(mapped)
    }) * 0.558838
  }
}
