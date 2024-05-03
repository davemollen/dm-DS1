use std::f32::consts::TAU;
mod bilinear_transform;
use bilinear_transform::BilinearTransform;
mod filter;
use filter::Filter;

const GAIN: f32 = 63.095734;

pub struct TransistorBooster {
  bilinear_transform: BilinearTransform,
  filter: Filter,
}

impl TransistorBooster {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      bilinear_transform: BilinearTransform::new(sample_rate),
      filter: Filter::new(),
    }
  }

  pub fn process(&mut self, input: f32) -> f32 {
    let s_domain_coefficients = self.get_s_domain_coefficients();
    let z_domain_coefficients = self.bilinear_transform.process(s_domain_coefficients);
    self.filter.process(input, z_domain_coefficients) * GAIN
  }

  fn get_s_domain_coefficients(&self) -> (f32, [f32; 3]) {
    let freq1 = 3.3862753849339;
    let freq2 = 673.449;
    let w1 = freq1 * TAU;
    let w2 = freq2 * TAU;

    (1., [1., w1 + w2, w1 * w2])
  }
}
