pub struct BilinearTransform {
  s: [f32; 2],
}

impl BilinearTransform {
  pub fn new(sample_rate: f32) -> Self {
    let t = sample_rate.recip();
    Self {
      s: [t / 2., t * t / 4.],
    }
  }

  /// First tuple element represents b0, because b1 & b2 are expected to equal zero.
  pub fn process(&self, (b, mut a): (f32, [f32; 3])) -> ([f32; 3], [f32; 3]) {
    a[1] *= self.s[0];
    a[2] *= self.s[1];

    let a0 = a[0] + a[1] + a[2];
    let a1 = (-2. * a[0] + 2. * a[2]) / a0;
    let a2 = (a[0] - a[1] + a[2]) / a0;

    let b0 = b / a0;
    let b1 = (-2. * b) / a0;

    ([b0, b1, b0], [1., a1, a2])
  }
}

#[cfg(test)]
mod tests {
  use super::BilinearTransform;

  #[test]
  fn bilinear_transform_should_be_correct() {
    let bilinear_transform = BilinearTransform::new(44100.);

    let coeffs: (f32, [f32; 3]) = (1., [1., 4252.681457679466, 90029.89067946235]);
    assert_eq!(
      bilinear_transform.process(coeffs),
      (
        [0.9539910019469044, -1.907982003893809, 0.9539910019469044],
        [1.0, -1.9079599226350703, 0.9080040851525475]
      )
    );
  }
}
