use crate::shared::one_pole_filter::OnePoleFilter;

const LOW_PASS_FREQ: f32 = 234.05138689985;
const HIGH_PASS_FREQ: f32 = 1063.8699404538;

pub struct Tone {
  lowpass: OnePoleFilter,
  highpass: OnePoleFilter,
}

impl Tone {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      lowpass: OnePoleFilter::new(sample_rate),
      highpass: OnePoleFilter::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32, tone: f32) -> f32 {
    let lowpass_output =
      self.lowpass.process(input, LOW_PASS_FREQ) * ((1. - tone) * 0.595235 + 0.202379);
    let highpass_output =
      input - self.highpass.process(input, HIGH_PASS_FREQ) * (tone * 0.694642 + 0.002896);

    lowpass_output + highpass_output
  }
}
