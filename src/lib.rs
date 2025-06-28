use nih_plug::prelude::*;
use std::sync::Arc;

mod dsp;
mod parameters;

use dsp::GuitarFxProcessor;
use parameters::GuitarFxParams;

pub struct GuitarFx {
    params: Arc<GuitarFxParams>,
    processor: GuitarFxProcessor,
}

impl Default for GuitarFx {
    fn default() -> Self {
        Self {
            params: Arc::new(GuitarFxParams::default()),
            processor: GuitarFxProcessor::new(),
        }
    }
}

impl Plugin for GuitarFx {
    const NAME: &'static str = "BIAS FX Rust";
    const VENDOR: &'static str = "Rust Audio";
    const URL: &'static str = "https://github.com/rust-audio/bias-fx-rust";
    const EMAIL: &'static str = "rust@audio.dev";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];
    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.processor.initialize(buffer_config.sample_rate);
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Functional processing pipeline
        for channel_samples in buffer.iter_samples() {
            let input_gain = self.params.input_gain.smoothed.next();
            let output_gain = self.params.output_gain.smoothed.next();
            let drive = self.params.drive.smoothed.next();
            let bass = self.params.bass.smoothed.next();
            let mid = self.params.mid.smoothed.next();
            let treble = self.params.treble.smoothed.next();
            
            // Update tone controls - O(1) per-sample update
            self.processor.update_tone_controls(bass, mid, treble);
            
            // Apply functional DSP chain
            for sample in channel_samples {
                *sample = self.processor.process_sample(*sample, input_gain, drive, output_gain);
            }
        }
        
        ProcessStatus::Normal
    }
}

impl Vst3Plugin for GuitarFx {
    const VST3_CLASS_ID: [u8; 16] = *b"BiasFxRust000001";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Distortion,
        Vst3SubCategory::Filter,
    ];
}

nih_export_vst3!(GuitarFx);