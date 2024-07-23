mod bilinear_transform;
mod biquad_filter;
use {bilinear_transform::BilinearTransform, biquad_filter::BiquadFilter};

const R1: f32 = 2200.;
const R2: f32 = 6800.;
const R3: f32 = 20000.;
const R4: f32 = 6800.;
const C1: f32 = 2.2e-8;
const C2: f32 = 1e-7;

pub struct Tone {
  bilinear_transform: BilinearTransform,
  filter: BiquadFilter,
}

impl Tone {
  const C1C2: f32 = C1 * C2;
  const R2R4: f32 = R2 * R4;
  const C1C2R2R4: f32 = Self::C1C2 * Self::R2R4;
  const C1C2R1R2: f32 = Self::C1C2 * R1 * R2;
  const C1R4: f32 = C1 * R4;
  const C1R1: f32 = C1 * R1;
  const C2R2: f32 = C2 * R2;

  pub fn new(sample_rate: f32) -> Self {
    Self {
      bilinear_transform: BilinearTransform::new(sample_rate),
      filter: BiquadFilter::new(),
    }
  }

  pub fn process(&mut self, input: f32, tone: f32) -> f32 {
    let s_domain_coefficients = Self::get_s_domain_coefficients(tone);
    let z_domain_coefficients = self.bilinear_transform.process(s_domain_coefficients);
    self.filter.process(input, z_domain_coefficients)
  }

  fn get_s_domain_coefficients(tone: f32) -> ([f32; 3], [f32; 3]) {
    let r3_a = tone * R3;
    let r3_b = (1. - tone) * R3;

    let b0 = Self::C1C2R2R4 * r3_a;
    let b1 =
      Self::C1R4 * r3_b + Self::C1R4 * r3_a + C1 * Self::R2R4 + Self::C1R1 * R4 + Self::C1R1 * r3_b;
    let b2 = R4 + r3_b;
    let a0 = b0
      + Self::C1C2R2R4 * r3_b
      + Self::C1C2R2R4 * R1
      + Self::C1C2R1R2 * r3_b
      + Self::C1C2R1R2 * r3_a;
    let a1 = b1
      + Self::C1R1 * r3_a
      + Self::C1R1 * R2
      + Self::C2R2 * R4
      + Self::C2R2 * r3_b
      + Self::C2R2 * r3_a;
    let a2 = R4 + r3_b + r3_a + R2;

    ([b0, b1, b2], [a0, a1, a2])
  }
}
