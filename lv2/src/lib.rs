extern crate ds1;
extern crate lv2;
use ds1::{Params, DS1};
use lv2::prelude::*;

#[derive(PortCollection)]
struct Ports {
  tone: InputPort<Control>,
  level: InputPort<Control>,
  dist: InputPort<Control>,
  input: InputPort<Audio>,
  output: OutputPort<Audio>,
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
    self.params.set(*ports.tone, *ports.level, *ports.dist);

    for (input, output) in ports.input.iter().zip(ports.output.iter_mut()) {
      *output = self.ds1.process(*input, &mut self.params);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmDS1);
