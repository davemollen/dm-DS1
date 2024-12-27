mod smooth;
use smooth::LinearSmooth;
pub use smooth::Smoother;

pub struct Params {
  pub tone: LinearSmooth,
  pub level: LinearSmooth,
  pub dist: LinearSmooth,
  is_initialized: bool,
}

impl Params {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      tone: LinearSmooth::new(sample_rate, 20.),
      level: LinearSmooth::new(sample_rate, 20.),
      dist: LinearSmooth::new(sample_rate, 20.),
      is_initialized: false,
    }
  }

  pub fn set(&mut self, tone: f32, level: f32, dist: f32) {
    if self.is_initialized {
      self.tone.set_target(tone);
      self.level.set_target(level);
      self.dist.set_target(dist);
    } else {
      self.tone.reset(tone);
      self.level.reset(level);
      self.dist.reset(dist);
      self.is_initialized = true;
    }
  }
}
