use std::f32::consts::TAU;
mod bilinear_transform;
use crate::shared::biquad_filter::BiquadFilter;
use bilinear_transform::BilinearTransform;

const FREQ_1: f32 = 3.3862753849339;
const FREQ_2: f32 = 673.449;
const GAIN: f32 = 63.095734;

pub struct TransistorBooster {
  z_domain_coefficients: ([f32; 3], [f32; 3]),
  filter: BiquadFilter,
}

impl TransistorBooster {
  pub fn new(sample_rate: f32) -> Self {
    let s_domain_coefficients = Self::get_s_domain_coefficients();
    let bilinear_transform = BilinearTransform::new(sample_rate);
    let z_domain_coefficients = bilinear_transform.process(s_domain_coefficients);

    Self {
      z_domain_coefficients,
      filter: BiquadFilter::new(),
    }
  }

  pub fn reset(&mut self) {
    self.filter.reset();
  }

  pub fn process(&mut self, input: f32) -> f32 {
    self.filter.process(input, self.z_domain_coefficients) * GAIN
  }

  fn get_s_domain_coefficients() -> (f32, [f32; 3]) {
    let w1 = FREQ_1 * TAU;
    let w2 = FREQ_2 * TAU;
    let a1 = w1 + w2;
    let a2 = w1 * w2;
    (1., [1., a1, a2])
  }
}
