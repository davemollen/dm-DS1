#[path = "../src/utils.rs"]
mod utils;
use criterion::{criterion_group, criterion_main, Criterion};
use ds1::DS1;
use utils::generate_signal_stream;

fn ds1_bench(c: &mut Criterion) {
  let mut ds1 = DS1::new(44100.);
  let signal_stream = generate_signal_stream(44100);

  c.bench_function("ds1", |b| {
    b.iter(|| {
      for signal in &signal_stream {
        ds1.process(*signal, 0.5, 0.5, 0.5);
      }
    })
  });
}

criterion_group!(benches, ds1_bench);
criterion_main!(benches);
