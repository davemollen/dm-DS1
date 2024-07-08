mod one_pole_filter;
use one_pole_filter::OnePoleFilter;

pub struct Tone {
  lowpass: OnePoleFilter,
  highpass: OnePoleFilter,
}

impl Tone {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      lowpass: OnePoleFilter::new(sample_rate, 234.05138689985),
      highpass: OnePoleFilter::new(sample_rate, 1063.8699404538),
    }
  }

  pub fn process(&mut self, input: f32, tone: f32) -> f32 {
    let lowpass_output = self.lowpass.process(input) * ((1. - tone) * 0.595235 + 0.202379);
    let highpass_output = input - self.highpass.process(input) * (tone * 0.694642 + 0.002896);

    lowpass_output + highpass_output
  }
}
