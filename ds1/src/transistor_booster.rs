use std::f32::consts::TAU;
mod bilinear_transform;
use crate::shared::biquad_filter::BiquadFilter;
use bilinear_transform::BilinearTransform;

const FREQ_1: f32 = 3.3862753849339;
const FREQ_2: f32 = 673.449;
const GAIN: f32 = 63.095734;

pub struct TransistorBooster {
  bilinear_transform: BilinearTransform,
  filter: BiquadFilter,
}

impl TransistorBooster {
  const W1: f32 = FREQ_1 * TAU;
  const W2: f32 = FREQ_2 * TAU;
  const A1: f32 = Self::W1 + Self::W2;
  const A2: f32 = Self::W1 * Self::W2;

  pub fn new(sample_rate: f32) -> Self {
    Self {
      bilinear_transform: BilinearTransform::new(sample_rate),
      filter: BiquadFilter::new(),
    }
  }

  pub fn reset(&mut self) {
    self.filter.reset();
  }

  pub fn process(&mut self, input: f32) -> f32 {
    let s_domain_coefficients = self.get_s_domain_coefficients();
    let z_domain_coefficients = self.bilinear_transform.process(s_domain_coefficients);
    self.filter.process(input, z_domain_coefficients) * GAIN
  }

  fn get_s_domain_coefficients(&self) -> (f32, [f32; 3]) {
    (1., [1., Self::A1, Self::A2])
  }
}
