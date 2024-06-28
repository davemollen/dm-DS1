mod oversample;
use crate::shared::one_pole_filter::OnePoleFilter;
use oversample::Oversample;
use std::simd::{f32x8, StdFloat};

pub struct Clipper {
  lowpass_filter: OnePoleFilter,
  oversample: Oversample,
}

impl Clipper {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      lowpass_filter: OnePoleFilter::new(sample_rate, 7230.),
      oversample: Oversample::new(),
    }
  }

  pub fn process(&mut self, input: f32) -> f32 {
    let filtered = self.lowpass_filter.process(input);

    self.oversample.process(filtered, |x| {
      let x2 = x * x;
      let x3 = x2 * x;
      let x5 = x3 * x2;
      let a = x + f32x8::splat(0.16489087) * x3 + f32x8::splat(0.00985468) * x5;
      a / (f32x8::splat(1.0) + a * a).sqrt()
    }) * 0.305496 // 0.610992 * 0.5
  }
}
