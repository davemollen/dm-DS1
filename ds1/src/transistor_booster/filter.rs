pub struct Filter {
  z1: f32,
  z2: f32,
}

impl Filter {
  pub fn new() -> Self {
    Self { z1: 0., z2: 0. }
  }

  pub fn process(&mut self, input: f32, (b, a): ([f32; 3], [f32; 3])) -> f32 {
    let y = input * b[0] + self.z1;
    self.z1 = input * b[1] + self.z2 - a[1] * y;
    self.z2 = input * b[2] - a[2] * y;

    y
  }
}
