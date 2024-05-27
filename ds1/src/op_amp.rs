pub struct OpAmp {
  z1: f32,
  z2: f32,
  double_sr: f32,
  squared_double_sr: f32,
}

impl OpAmp {
  const R1: f32 = 100000.;
  const R2: f32 = 4700.;
  const C1: f32 = 2.5e-10;
  const C2: f32 = 1e-6;

  pub fn new(sample_rate: f32) -> Self {
    let double_sr = sample_rate * 2.;

    Self {
      z1: 0.,
      z2: 0.,
      double_sr,
      squared_double_sr: double_sr * double_sr,
    }
  }

  pub fn process(&mut self, input: f32, dist: f32) -> f32 {
    let dist = dist.max(0.00001);

    let rt = dist * Self::R1;
    let rb = ((1. - dist) * Self::R1) + Self::R2;

    let a = (rt * rb * Self::C2 * Self::C1).recip();
    let c = (rt * Self::C1).recip() + (rb * Self::C2).recip();
    let b = c + (rb * Self::C1).recip();

    let a0 = a + c * self.double_sr + self.squared_double_sr;
    let b0 = (a + b * self.double_sr + self.squared_double_sr) / a0;
    let b1 = (2. * a - 2. * self.squared_double_sr) / a0;
    let b2 = (a - b * self.double_sr + self.squared_double_sr) / a0;
    let a1 = b1;
    let a2 = (a - c * self.double_sr + self.squared_double_sr) / a0;

    let y = input * b0 + self.z1;
    self.z1 = input * b1 + self.z2 - a1 * y;
    self.z2 = input * b2 - a2 * y;

    y
  }
}
