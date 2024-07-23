#![feature(portable_simd)]
mod transistor_booster;
use transistor_booster::TransistorBooster;
mod op_amp;
use op_amp::OpAmp;
mod clipper;
use clipper::Clipper;
mod tone;
use tone::Tone;
mod shared {
  pub mod float_ext;
}
mod smooth_parameters;
use smooth_parameters::SmoothParameters;

pub struct DS1 {
  transistor_booster: TransistorBooster,
  op_amp: OpAmp,
  clipper: Clipper,
  tone: Tone,
  smooth_parameters: SmoothParameters,
}

impl DS1 {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      transistor_booster: TransistorBooster::new(sample_rate),
      op_amp: OpAmp::new(sample_rate),
      clipper: Clipper::new(sample_rate),
      tone: Tone::new(sample_rate),
      smooth_parameters: SmoothParameters::new(sample_rate),
    }
  }

  pub fn initialize_params(&mut self, tone: f32, level: f32, dist: f32) {
    self.smooth_parameters.initialize(tone, level, dist);
  }

  pub fn process(&mut self, input: f32, tone: f32, level: f32, dist: f32) -> f32 {
    let (tone, level, dist) = self.smooth_parameters.process(tone, level, dist);
    let booster_output = self.transistor_booster.process(input);
    let op_amp_output = self.op_amp.process(booster_output, dist);
    let clip_output = self.clipper.process(op_amp_output);
    let tone_output = self.tone.process(clip_output, tone);

    tone_output * level
  }
}
