pub struct FirBuffer {
  index: usize,
  buffer: Vec<f32>
}

impl FirBuffer {
  pub fn new(length: usize) -> Self {
    Self {
      index: 0,
      buffer: vec![0.; length]
    }
  }

  pub fn write(&mut self, input: f32) {
    self.index = self.wrap(self.index + 1);
    self.buffer[self.index] = input;
  }

  pub fn read(&self, i: usize) -> f32 {
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