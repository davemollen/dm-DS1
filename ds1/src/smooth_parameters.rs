mod ramp_smooth;
use ramp_smooth::RampSmooth;

const RAMP_TIME: f32 = 50.;

pub struct SmoothParameters {
  filters: [RampSmooth; 3],
}

impl SmoothParameters {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      filters: [RampSmooth::new(sample_rate); 3],
    }
  }

  pub fn process(&mut self, tone: f32, level: f32, dist: f32) -> (f32, f32, f32) {
    (
      self.filters[0].process(tone, RAMP_TIME),
      self.filters[1].process(level, RAMP_TIME),
      self.filters[2].process(dist, RAMP_TIME),
    )
  }
}
