mod ramp_smooth;
use ramp_smooth::RampSmooth;

pub struct SmoothParameters {
  smooth_tone: RampSmooth,
  smooth_level: RampSmooth,
  smooth_dist: RampSmooth,
}

impl SmoothParameters {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      smooth_tone: RampSmooth::new(sample_rate, 20.),
      smooth_level: RampSmooth::new(sample_rate, 20.),
      smooth_dist: RampSmooth::new(sample_rate, 20.),
    }
  }

  pub fn initialize(&mut self, tone: f32, level: f32, dist: f32) {
    self.smooth_tone.initialize(tone);
    self.smooth_level.initialize(level);
    self.smooth_dist.initialize(dist);
  }

  pub fn process(&mut self, tone: f32, level: f32, dist: f32) -> (f32, f32, f32) {
    (
      self.smooth_tone.process(tone),
      self.smooth_level.process(level),
      self.smooth_dist.process(dist),
    )
  }
}
