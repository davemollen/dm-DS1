use std::simd::{f32x4, num::SimdFloat};
mod oversample;
use oversample::Oversample;
use crate::shared::lowpass_filter::LowpassFilter;

pub struct Clipper {
  lowpass_filter: LowpassFilter,
  oversample: Oversample<4>
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

    self.oversample.process(
      filtered, 
      |x| {
        // let squared_x = x * x;
        // let numerator = x * (f32x4::splat(135135.) + squared_x * (f32x4::splat(17325.) + squared_x * (f32x4::splat(378.) + squared_x)));
        // let denominator = f32x4::splat(135135.) + squared_x * (f32x4::splat(62370.) + squared_x * (f32x4::splat(3150.) + squared_x * f32x4::splat(28.)));
        // numerator / denominator
        let n = 6.;
        let mapped = f32x4::to_array(x).map(|x| {
          x / (1. + x.abs()).powf(n).powf(1. / n)
        });
        f32x4::from_array(
          mapped
        )
      }
    ) * 0.558838
  }
}