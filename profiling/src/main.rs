mod utils;
use ds1::DS1;
use utils::generate_signal;

fn main() {
  let mut ds1 = DS1::new(44100.);

  loop {
    let input = generate_signal();
    ds1.process(input, 0.5, 0.5, 0.5);
  }
}
