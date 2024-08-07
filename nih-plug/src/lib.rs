use ds1::DS1;
use nih_plug::prelude::*;
use std::sync::Arc;
mod ds1_parameters;
use ds1_parameters::DS1Parameters;
mod editor;

struct DmDS1 {
  params: Arc<DS1Parameters>,
  ds1: DS1,
}

impl DmDS1 {
  pub fn get_params(&self) -> (f32, f32, f32) {
    (
      self.params.tone.value(),
      self.params.level.value(),
      self.params.dist.value(),
    )
  }
}

impl Default for DmDS1 {
  fn default() -> Self {
    let params = Arc::new(DS1Parameters::default());
    Self {
      params: params.clone(),
      ds1: DS1::new(44100.),
    }
  }
}

impl Plugin for DmDS1 {
  const NAME: &'static str = "dm-DS1";
  const VENDOR: &'static str = "DM";
  const URL: &'static str = "https://github.com/davemollen/dm-DS1";
  const EMAIL: &'static str = "davemollen@gmail.com";
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");

  const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
    main_input_channels: NonZeroU32::new(1),
    main_output_channels: NonZeroU32::new(1),
    ..AudioIOLayout::const_default()
  }];
  const MIDI_INPUT: MidiConfig = MidiConfig::None;
  const SAMPLE_ACCURATE_AUTOMATION: bool = true;

  // More advanced plugins can use this to run expensive background tasks. See the field's
  // documentation for more information. `()` means that the plugin does not have any background
  // tasks.
  type BackgroundTask = ();
  type SysExMessage = ();

  fn params(&self) -> Arc<dyn Params> {
    self.params.clone()
  }

  fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
    editor::create(self.params.clone(), self.params.editor_state.clone())
  }

  fn initialize(
    &mut self,
    _audio_io_layout: &AudioIOLayout,
    buffer_config: &BufferConfig,
    _context: &mut impl InitContext<Self>,
  ) -> bool {
    self.ds1 = DS1::new(buffer_config.sample_rate);
    let (tone, level, dist) = self.get_params();
    self.ds1.initialize_params(tone, level, dist);
    true
  }

  fn process(
    &mut self,
    buffer: &mut Buffer,
    _aux: &mut AuxiliaryBuffers,
    _context: &mut impl ProcessContext<Self>,
  ) -> ProcessStatus {
    let (tone, level, dist) = self.get_params();

    buffer.iter_samples().for_each(|mut channel_samples| {
      let sample = channel_samples.iter_mut().next().unwrap();
      let ds1_output = self.ds1.process(*sample, tone, level, dist);
      *sample = ds1_output;
    });
    ProcessStatus::Normal
  }

  // This can be used for cleaning up special resources like socket connections whenever the
  // plugin is deactivated. Most plugins won't need to do anything here.
  fn deactivate(&mut self) {}
}

impl ClapPlugin for DmDS1 {
  const CLAP_ID: &'static str = "dm-DS1";
  const CLAP_DESCRIPTION: Option<&'static str> = Some("A distortion plugin");
  const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
  const CLAP_SUPPORT_URL: Option<&'static str> = None;
  const CLAP_FEATURES: &'static [ClapFeature] = &[
    ClapFeature::AudioEffect,
    ClapFeature::Mono,
    ClapFeature::Utility,
    ClapFeature::Distortion,
  ];
}

impl Vst3Plugin for DmDS1 {
  const VST3_CLASS_ID: [u8; 16] = *b"dm-DS1..........";
  const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
    Vst3SubCategory::Fx,
    Vst3SubCategory::Mono,
    Vst3SubCategory::Distortion,
  ];
}

nih_export_clap!(DmDS1);
nih_export_vst3!(DmDS1);
