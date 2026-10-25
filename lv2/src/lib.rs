extern crate ds1;
extern crate lv2;
use ds1::{Bypass, Params, DS1};
use lv2::prelude::*;

#[derive(PortCollection)]
struct Ports {
  tone: InputPort<InPlaceControl>,
  level: InputPort<InPlaceControl>,
  dist: InputPort<InPlaceControl>,
  input: InputPort<InPlaceAudio>,
  output: OutputPort<InPlaceAudio>,
  enabled: InputPort<InPlaceControl>,
}

#[uri("https://github.com/davemollen/dm-DS1")]
struct DmDS1 {
  ds1: DS1,
  params: Params,
  bypass: Bypass,
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
      bypass: Bypass::new(sample_rate),
    })
  }

  /// Reset the internal state of the plugin.
  fn activate(&mut self, _features: &mut Self::InitFeatures) {
    self.ds1.reset();
    self.params.reset();
    self.bypass.reset();
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), sample_count: u32) {
    self.bypass.set_enabled(ports.enabled.get() == 0., || {
      self.ds1.reset();
      self.params.reset();
    });

    if self.bypass.is_done_activating() {
      ports.output.copy_from(&ports.input, sample_count);
      return;
    }

    self
      .params
      .set(ports.tone.get(), ports.level.get(), ports.dist.get());

    for (input, output) in ports.input.iter().zip(ports.output.iter()) {
      let dry = input.get();
      let wet = self.ds1.process(dry, &mut self.params);
      let ds1_output = self.bypass.process(dry, wet);
      output.set(ds1_output);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmDS1);
