use std::f32::consts::TAU;

pub struct LowpassFilter {
  sample_rate: f32,
  z: f32,
}

impl LowpassFilter {
  pub fn new(sample_rate: f32) -> Self {
    Self { sample_rate, z: 0. }
  }

  pub fn process(&mut self, input: f32, frequency: f32) -> f32 {
    let double_sample_rate = 2. * self.sample_rate;
    let sample_period = double_sample_rate.recip();
    let radians = (frequency * TAU * sample_period).tan() * double_sample_rate;

    let a0 = radians + double_sample_rate;
    let a1 = (radians - double_sample_rate) / a0;
    let b0 = radians / a0;
    let b1 = radians / a0;

    let y = input * b0 + self.z;
    self.z = input * b1 - a1 * y;

    y
  }
}
