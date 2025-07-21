extern crate ds1;
extern crate lv2;
use ds1::{Params, DS1};
use lv2::prelude::*;

#[derive(PortCollection)]
struct Ports {
  tone: InputPort<InPlaceControl>,
  level: InputPort<InPlaceControl>,
  dist: InputPort<InPlaceControl>,
  input: InputPort<InPlaceAudio>,
  output: OutputPort<InPlaceAudio>,
}

#[uri("https://github.com/davemollen/dm-DS1")]
struct DmDS1 {
  ds1: DS1,
  params: Params,
}

impl Plugin for DmDS1 {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    let sample_rate = plugin_info.sample_rate() as f32;

    Some(Self {
      ds1: DS1::new(sample_rate),
      params: Params::new(sample_rate),
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    self
      .params
      .set(ports.tone.get(), ports.level.get(), ports.dist.get());

    for (input, output) in ports.input.iter().zip(ports.output.iter()) {
      let ds1_output = self.ds1.process(input.get(), &mut self.params);
      output.set(ds1_output);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmDS1);
