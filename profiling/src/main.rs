mod utils;
use ds1::{Params, DS1};
use utils::generate_signal;

fn main() {
  let mut ds1 = DS1::new(44100.);
  let mut params = Params::new(44100.);
  params.set(0.5, 0.5, 0.5);

  loop {
    let input = generate_signal();
    ds1.process(input, &mut params);
  }
}
