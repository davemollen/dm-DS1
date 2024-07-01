const R: f32 = 2200.;
const C: f32 = 1e-8;
const IS: f32 = 2.52e-9;
const VT: f32 = 0.0453;

pub struct Clipper {
  fs: f32,
  x: f32,
  yn1: f32,
}

impl Clipper {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      fs: sample_rate,
      x: 0.,
      yn1: 0.,
    }
  }

  pub fn process(&mut self, input: f32) -> f32 {
    let mut yc: f32 = self.yn1; // Current value of the output (being solved for)
    let mut res: f32 = 10.; // set residual above limit of while loop

    for _ in 0..100 {
      if res < 1e-12 {
        break;
      }

      // Calculate the function value
      let f = (input - yc) / R - ((2. * C * self.fs) * yc) + self.x - 2. * IS * (yc / VT).sinh();

      // Calculate it's derivative
      let j = -1. / R - (2. * C * self.fs) - (2. * IS / VT) * (yc / VT).cosh();

      // Newton step is defined by the function divided by the derivative.
      let step = -f / j;

      // Only magnitude is usefuly for defining a residual
      res = step.abs();

      // Take a Newton step towards the solution (potentially)
      yc = yc + step;
    }

    // Update last solution
    self.yn1 = yc;

    // Calculate state
    self.x = (4. * C * self.fs) * yc - self.x;

    yc * 0.5
  }
}
