mod coefficients;
use coefficients::Coefficients;
use std::{iter::Sum, ops::Mul, simd::num::SimdFloat};
mod fir_buffer;
use fir_buffer::{FirBuffer, SimdFir};
mod simd_type;
use simd_type::SimdType;

pub struct Oversample<T> {
  upsample_buffer: FirBuffer<T>,
  downsample_buffer: FirBuffer<T>,
  coefficients: Vec<T>,
  oversample_factor: usize,
}

impl<T: SimdType + Mul<T> + Sum<<T as Mul>::Output> + Copy + SimdFloat<Scalar = f32>> Oversample<T>
where
  Vec<T>: Coefficients,
{
  pub fn new() -> Self {
    let oversample_factor = T::oversample_factor();

    Self {
      upsample_buffer: FirBuffer::new(32 / oversample_factor),
      downsample_buffer: FirBuffer::new(32),
      coefficients: Coefficients::new(),
      oversample_factor,
    }
  }

  pub fn process<F>(&mut self, input: f32, callback: F) -> f32
  where
    F: Fn(T) -> T,
  {
    let upsampled = self.upsample(input);
    let processed = self.run_upsampled_process(upsampled, callback);
    self.downsample(processed)
  }

  fn upsample(&mut self, input: f32) -> T {
    self
      .upsample_buffer
      .write(SimdType::splat(input * self.oversample_factor as f32));

    self
      .coefficients
      .iter()
      .enumerate()
      .map(|(i, coeff)| self.upsample_buffer.read(i) * *coeff)
      .sum()
  }

  fn run_upsampled_process<F>(&mut self, input: T, callback: F) -> T
  where
    F: Fn(T) -> T,
  {
    callback(input)
  }

  fn downsample(&mut self, input: T) -> f32 {
    self.downsample_buffer.write(input);

    self
      .coefficients
      .iter()
      .enumerate()
      .map(|(i, coeff)| self.downsample_buffer.read(i) * *coeff)
      .sum::<T>()
      .reduce_sum()
  }
}
