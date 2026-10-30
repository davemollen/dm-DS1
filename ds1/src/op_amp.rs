use crate::shared::biquad_filter::BiquadFilter;

const R1: f32 = 100000.;
const R2: f32 = 4700.;
const C1: f32 = 2.5e-10;
const C2: f32 = 1e-6;

pub struct OpAmp {
  z: [f32; 2],
  double_sr: f32,
  squared_double_sr: f32,
  filter: BiquadFilter,
  prev_dist: Option<f32>,
  z_domain_coefficients: ([f32; 3], [f32; 3]),
}

impl OpAmp {
  pub fn new(sample_rate: f32) -> Self {
    let double_sr = sample_rate * 2.;

    Self {
      z: [0.; 2],
      double_sr,
      squared_double_sr: double_sr * double_sr,
      filter: BiquadFilter::new(),
      prev_dist: None,
      z_domain_coefficients: ([0.; 3], [0.; 3]),
    }
  }

  pub fn reset(&mut self) {
    self.z = [0.; 2];
  }

  pub fn process(&mut self, input: f32, dist: f32) -> f32 {
    if self.prev_dist.map_or(true, |value| value != dist) {
      self.set_z_domain_coefficients(dist);
      self.prev_dist = Some(dist);
    }
    self.filter.process(input, self.z_domain_coefficients)
  }

  fn set_z_domain_coefficients(&mut self, dist: f32) {
    let rt = dist * R1;
    let rb = ((1. - dist) * R1) + R2;

    let a = (rt * rb * C2 * C1).recip();
    let c = (rt * C1).recip() + (rb * C2).recip();
    let b = c + (rb * C1).recip();

    let a0 = a + c * self.double_sr + self.squared_double_sr;
    let b0 = (a + b * self.double_sr + self.squared_double_sr) / a0;
    let b1 = (2. * a - 2. * self.squared_double_sr) / a0;
    let b2 = (a - b * self.double_sr + self.squared_double_sr) / a0;
    let a1 = b1;
    let a2 = (a - c * self.double_sr + self.squared_double_sr) / a0;

    self.z_domain_coefficients = ([b0, b1, b2], [a0, a1, a2]);
  }
}
