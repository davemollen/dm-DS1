use std::simd::f32x8;

pub struct Coefficients;

impl Coefficients {
  pub fn new() -> Vec<f32x8> {
    vec![
      f32x8::from_array([
        0.00138863, 0.00389379, 0.00764977, 0.01314461, 0.02048258, 0.02970446, 0.04061367,
        0.05281857,
      ]),
      f32x8::from_array([
        0.06569567, 0.0784371, 0.09009214, 0.09965243, 0.10614339, 0.10873352, 0.10683284,
        0.10018167,
      ]),
      f32x8::from_array([
        0.08890737,
        0.07354607,
        0.05502176,
        0.03458045,
        0.01368524,
        -0.00611698,
        -0.02334637,
        -0.03673952,
      ]),
      f32x8::from_array([
        -0.04538114,
        -0.04880201,
        -0.04702996,
        -0.0405883,
        -0.03043896,
        -0.0178782,
        -0.00439211,
        0.00850537,
      ]),
      f32x8::from_array([
        0.01943807, 0.02731595, 0.03144571, 0.03159674, 0.02800803, 0.02134161, 0.01258681,
        0.00292778,
      ]),
      f32x8::from_array([
        -0.0064078,
        -0.01430001,
        -0.0198681,
        -0.02256863,
        -0.02224578,
        -0.01913206,
        -0.01379954,
        -0.00706775,
      ]),
      f32x8::from_array([
        0.00011182, 0.00678668, 0.01212583, 0.01552138, 0.01665597, 0.01552786, 0.01243315,
        0.00790995,
      ]),
      f32x8::from_array([
        0.00265265,
        -0.00259045,
        -0.00712021,
        -0.01037777,
        -0.01201176,
        -0.01191261,
        -0.01021337,
        -0.007256,
      ]),
      f32x8::from_array([
        -0.00353073,
        0.00039997,
        0.00398407,
        0.00675546,
        0.00839228,
        0.00875266,
        0.00788157,
        0.00599193,
      ]),
      f32x8::from_array([
        0.00342335,
        0.00058524,
        -0.00210588,
        -0.00428557,
        -0.00568937,
        -0.00618252,
        -0.00576957,
        -0.00458298,
      ]),
      f32x8::from_array([
        -0.00285466,
        -0.00087489,
        0.00105401,
        0.0026618,
        0.00374721,
        0.00420111,
        0.00401478,
        0.00327309,
      ]),
      f32x8::from_array([
        0.0021338,
        0.00079831,
        -0.00052152,
        -0.00163572,
        -0.00240187,
        -0.00274183,
        -0.00264819,
        -0.00217829,
      ]),
      f32x8::from_array([
        -0.00143976,
        -0.00056964,
        0.0002895,
        0.00101094,
        0.00150163,
        0.00171309,
        0.00164435,
        0.00133694,
      ]),
      f32x8::from_array([
        0.00086474,
        0.00031848,
        -0.00020959,
        -0.00064005,
        -0.0009182,
        -0.00101888,
        -0.00094817,
        -0.00073832,
      ]),
      f32x8::from_array([
        -0.00043984,
        -0.00011159,
        0.00018994,
        0.00041912,
        0.00054802,
        0.00056848,
        0.00049174,
        0.00034378,
      ]),
      f32x8::from_array([
        1.59147605e-04,
        -2.55721064e-05,
        -1.78749728e-04,
        -2.77837494e-04,
        -3.12989768e-04,
        -2.86403258e-04,
        -2.11623432e-04,
        -1.08384479e-04,
      ]),
    ]
  }
}
