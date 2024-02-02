mod lookup_table;
use lookup_table::LOOKUP_TABLE;
mod oversample;
use oversample::Oversample;

pub struct Clipper {
  oversample: Oversample<8>,
  lookup_table: [f32; 4096]
}

impl Clipper {
  pub fn new() -> Self {
    Self {
      oversample: Oversample::new(),
      lookup_table: LOOKUP_TABLE
    }
  }

  pub fn process(&mut self, input: f32) -> f32 {
    let lookup_table = self.lookup_table;

    self.oversample.process(
      input, 
      |x| Self::non_linear_process(x, lookup_table)
    )
  }

  fn linear_interp(index: usize, lookup_table: [f32; 4096], mix: f32) -> f32 {
    let x = lookup_table[index];
    let y = lookup_table[index + 1];
    x * (1. - mix) + y * mix
  }

  fn non_linear_process(input: f32, lookup_table: [f32; 4096]) -> f32 {
    let table_length: usize = lookup_table.len() - 1;

    let phase: f32 = input * 0.111111 + 0.5;
    let index = phase * table_length as f32;
    let truncated_index = index.trunc();
    let mix = index - truncated_index;
    
    if truncated_index >= table_length as f32 {
      lookup_table[table_length]
    } else if truncated_index <= 0. {
      lookup_table[0]
    } else {
      Self::linear_interp(index as usize, lookup_table, mix)
    }
  } 
}