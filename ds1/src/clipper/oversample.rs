mod coefficients;
use std::simd::{f32x4, num::SimdFloat};
use coefficients::{QUARTER_BAND};
mod fir_buffer;
use fir_buffer::FirBuffer;

pub struct Oversample<const N: usize> {
  upsample_buffer: FirBuffer,
  downsample_buffer: FirBuffer,
  coefficients: [f32x4; 8]
}

impl<const N: usize> Oversample<N> {
  pub fn new() -> Self {
    Self {
      upsample_buffer: FirBuffer::new(32 / N),
      downsample_buffer: FirBuffer::new(32),
      coefficients: match N {
        // 2 => HALF_BAND,
        4 => QUARTER_BAND,
        // 8 => EIGHTH_BAND,
        // 16 => SIXTEENTH_BAND,
        _ => panic!("Oversampling ratio must be either 2, 4, 8 or 16.")
      }  
    }
  }

  pub fn process<F>(&mut self, input: f32, callback: F) -> f32
    where F: Fn(f32x4) -> f32x4 {
      let upsampled = self.upsample(input);
      let processed = self.run_upsampled_process(upsampled, callback);
      self.downsample(processed)
  }

  pub fn upsample(&mut self, input: f32) -> f32x4 {
    self.upsample_buffer.write(f32x4::splat(input * N as f32));

    self.coefficients
      .iter()
      .enumerate()
      .map(|(i, coeff)| {
        self.upsample_buffer.read(i) * coeff
      })
      .sum()
  }

  pub fn run_upsampled_process<F>(&mut self, input: f32x4, callback: F) -> f32x4
    where F: Fn(f32x4) -> f32x4 {
      callback(input)
  }

  pub fn downsample(&mut self, input: f32x4) -> f32 {
    self.downsample_buffer.write(input);
    
    self.coefficients
      .iter()
      .enumerate()
      .map(|(i, coeff)| {
        self.downsample_buffer.read(i) * coeff
      })
      .sum::<f32x4>()
      .reduce_sum()
  }
}