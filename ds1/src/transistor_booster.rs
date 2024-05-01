use std::f32::consts::TAU;

pub struct TransistorBooster {
  z1: f32,
  z2: f32,
  double_sr: f32,
  sample_period: f32,
}

impl TransistorBooster {
  pub fn new(sample_rate: f32) -> Self {
    let double_sr = sample_rate * 2.;
    Self {
      z1: 0.,
      z2: 0.,
      double_sr,
      sample_period: double_sr.recip(),
    }
  }

  pub fn process(&mut self, input: f32) -> f32 {
    let freq1 = 3.077643;
    let freq2 = 703.162476;
    let gain = 63.095734;

    let radians1 = (freq1 * TAU * self.sample_period).tan() * self.double_sr;
    let radians2 = (freq2 * TAU * self.sample_period).tan() * self.double_sr;

    let a = radians1 + self.double_sr;
    let b = radians1 - self.double_sr;
    let c = radians2 + self.double_sr;
    let d = radians2 - self.double_sr;

    let a0 = a * c;
    let a1 = (a * d + b * c) / a0;
    let a2 = b * d / a0;
    let b0 = self.double_sr * self.double_sr / a0 * gain;
    let b1 = b0 * -2.;
    let b2 = b0;

    let y = input * b0 + self.z1;
    self.z1 = input * b1 + self.z2 - a1 * y;
    self.z2 = input * b2 - a2 * y;

    y
  }
}
