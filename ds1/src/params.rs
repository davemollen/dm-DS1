mod smooth;
use smooth::LinearSmooth;
pub use smooth::Smoother;

pub struct Params {
  pub tone: LinearSmooth,
  pub level: LinearSmooth,
  pub dist: LinearSmooth,
}

impl Params {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      tone: LinearSmooth::new(20., sample_rate),
      level: LinearSmooth::new(20., sample_rate),
      dist: LinearSmooth::new(20., sample_rate),
    }
  }

  pub fn set(&mut self, tone: f32, level: f32, dist: f32) {
    self.tone.set_target(tone);
    self.level.set_target(level);
    self.dist.set_target(dist);
  }
}
