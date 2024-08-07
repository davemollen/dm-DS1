extern crate ds1;
extern crate lv2;
use ds1::DS1;
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
  is_active: bool,
}

impl Plugin for DmDS1 {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    Some(Self {
      ds1: DS1::new(_plugin_info.sample_rate() as f32),
      is_active: false,
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    let tone = *ports.tone;
    let level = *ports.level;
    let dist = *ports.dist;

    if !self.is_active {
      self.ds1.initialize_params(tone, level, dist);
      self.is_active = true;
    }

    for (input, output) in ports.input.iter().zip(ports.output.iter_mut()) {
      *output = self.ds1.process(*input, tone, level, dist);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmDS1);
