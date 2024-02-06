use super::simd_type::SimdType;

pub trait SimdFir {
  type Float;

  fn new(length: usize) -> Self;
  fn write(&mut self, input: Self::Float);
  fn read(&self, i: usize) -> Self::Float;
  fn wrap(&self, i: usize) -> usize;
}

pub struct FirBuffer<T> {
  index: usize,
  buffer: Vec<T>,
}

impl<T: SimdType + Clone + Copy> SimdFir for FirBuffer<T> {
  type Float = T;

  fn new(length: usize) -> Self {
    Self {
      index: 0,
      buffer: vec![T::splat(0.); length],
    }
  }

  fn write(&mut self, input: Self::Float) {
    self.index = self.wrap(self.index + 1);
    self.buffer[self.index] = input;
  }

  fn read(&self, i: usize) -> Self::Float {
    let index = self.index - i + self.buffer.len();
    self.buffer[self.wrap(index)]
  }

  fn wrap(&self, index: usize) -> usize {
    let buffer_len = self.buffer.len();
    if index >= buffer_len {
      index - buffer_len
    } else {
      index
    }
  }
}
