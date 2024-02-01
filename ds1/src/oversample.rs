mod fir_filter;
use fir_filter::FirFilter;

pub struct Oversample<const N: usize> {
  fir_filter: FirFilter<N>,
  buffer: [f32; N]
}

impl<const N: usize> Oversample<N> {
  pub fn new() -> Self {
    Self {
      fir_filter: FirFilter::default(),
      buffer: [0.; N]
    }
  }

  pub fn upsample(&mut self, input: f32) {
    // zero-stuff the input
    self.buffer.fill(0.);
    self.buffer[0] = (N as f32) * input;
    let fir = &mut self.fir_filter;

    self.buffer.iter_mut().for_each(|sample| {
      *sample = fir.process(*sample);
    })
  }

  pub fn process<F>(&mut self, callback: F)
    where F: Fn(f32) -> f32 {
      self.buffer.iter_mut().for_each(|x| {
        *x = callback(*x)
      });
    }

  pub fn downsample(&mut self) -> f32 {
    let fir = &mut self.fir_filter;

    let sample = self.buffer[0];
    fir.process(sample)
  }
}

