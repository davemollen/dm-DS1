mod coefficients;
use coefficients::{HALF_BAND, QUARTER_BAND, EIGHTH_BAND, SIXTEENTH_BAND};
mod fir_buffer;
use fir_buffer::FirBuffer;

pub struct Oversample<const N: usize> {
  upsample_buffer: FirBuffer,
  downsample_buffer: FirBuffer,
  coefficients: [f32; 32]
}

impl<const N: usize> Oversample<N> {
  pub fn new() -> Self {
    Self {
      upsample_buffer: FirBuffer::new(32 / N),
      downsample_buffer: FirBuffer::new(32),
      coefficients: match N {
        2 => HALF_BAND,
        4 => QUARTER_BAND,
        8 => EIGHTH_BAND,
        16 => SIXTEENTH_BAND,
        _ => panic!("Oversampling ratio must be either 2, 4, 8 or 16.")
      }  
    }
  }

  pub fn process<F>(&mut self, input: f32, callback: F) -> f32
    where F: Fn(f32) -> f32 {
      let upsampled = self.upsample(input);
      let processed = self.run_upsampled_process(upsampled, callback);
      self.downsample(processed)
  }

  pub fn upsample(&mut self, input: f32) -> [f32; N] {
    self.upsample_buffer.write(input * N as f32);

    self.coefficients
      .iter()
      .enumerate()
      .fold([0.; N], |mut acc, (i, coeff)| {
        acc[i % N] += self.upsample_buffer.read(i / N) * coeff;
        acc
      })
  }

  pub fn run_upsampled_process<F>(&mut self, input: [f32; N], callback: F) -> [f32; N]
    where F: Fn(f32) -> f32 {
      input.map(|sample| {
        callback(sample)
      })
  }

  pub fn downsample(&mut self, input: [f32; N]) -> f32 {
    input.iter().for_each(|sample| {
      self.downsample_buffer.write(*sample);
    });

    self.coefficients
      .iter()
      .enumerate()
      .map(|(i, coeff)| {
        self.downsample_buffer.read(i) * coeff
      })
      .sum()
  }
}