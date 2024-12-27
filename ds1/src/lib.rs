#![feature(portable_simd)]
mod clipper;
mod op_amp;
mod tone;
mod transistor_booster;
mod shared {
  pub mod float_ext;
}
mod params;
pub use params::Params;
use {
  clipper::Clipper, op_amp::OpAmp, params::Smoother, tone::Tone,
  transistor_booster::TransistorBooster,
};

pub struct DS1 {
  transistor_booster: TransistorBooster,
  op_amp: OpAmp,
  clipper: Clipper,
  tone: Tone,
}

impl DS1 {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      transistor_booster: TransistorBooster::new(sample_rate),
      op_amp: OpAmp::new(sample_rate),
      clipper: Clipper::new(sample_rate),
      tone: Tone::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32, params: &mut Params) -> f32 {
    let tone = params.tone.next();
    let level = params.level.next();
    let dist = params.dist.next();

    let booster_output = self.transistor_booster.process(input);
    let op_amp_output = self.op_amp.process(booster_output, dist);
    let clip_output = self.clipper.process(op_amp_output);
    let tone_output = self.tone.process(clip_output, tone);

    tone_output * level
  }
}
