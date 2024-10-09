mod fir_filter;
mod one_pole_filter;
use {
  fir_filter::FirFilter,
  one_pole_filter::OnePoleFilter,
  std::simd::{f32x8, num::SimdFloat, StdFloat},
};

pub struct Clipper {
  lowpass_filter: OnePoleFilter,
  upsample_fir: FirFilter,
  downsample_fir: FirFilter,
}

impl Clipper {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      lowpass_filter: OnePoleFilter::new(sample_rate, 7230.),
      upsample_fir: FirFilter::new(),
      downsample_fir: FirFilter::new(),
    }
  }

  pub fn process(&mut self, input: f32) -> f32 {
    let filtered = self.lowpass_filter.process(input);
    let upsampled = self.upsample_fir.upsample(filtered * 2.05);
    let clipped = Self::clip(upsampled);
    self.downsample_fir.downsample(clipped) * 0.302344 // 0.604688 * 0.5
  }

  fn clip(x: f32x8) -> f32x8 {
    let x_abs = x.abs();
    let x2 = x_abs * x_abs;
    let x4 = x2 * x2;
    let a = f32x8::splat(1.) + x4;

    x / a.sqrt().sqrt()
  }
}
