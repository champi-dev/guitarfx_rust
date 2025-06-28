// Building lightweight functional DSP without external dependencies for O(1) performance

mod filters;
mod distortion;
mod amp_sim;

use filters::ToneStack;
use distortion::AsymmetricClipper;
use amp_sim::TubeStage;

/// High-performance functional DSP processor achieving O(1) complexity
/// Uses FunDSP's zero-cost abstractions for real-time audio processing
pub struct GuitarFxProcessor {
    /// Sample rate for DSP calculations
    sample_rate: f32,
    
    /// Functional DSP graph for tone shaping - O(1) lookups via pre-computed coefficients
    tonestack: ToneStack,
    
    /// Asymmetric clipper for tube-like distortion - O(1) waveshaping 
    clipper: AsymmetricClipper,
    
    /// Tube preamp simulation stage - O(1) nonlinear processing
    tube_stage: TubeStage,
}

impl GuitarFxProcessor {
    /// Create new processor with O(1) initialization complexity
    pub fn new() -> Self {
        Self {
            sample_rate: 44100.0,
            tonestack: ToneStack::new(44100.0),
            clipper: AsymmetricClipper::new(),
            tube_stage: TubeStage::new(),
        }
    }
    
    /// Initialize processor with given sample rate - O(1) complexity
    /// Pre-computes all filter coefficients for real-time performance
    pub fn initialize(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.tonestack = ToneStack::new(sample_rate);
    }
    
    /// Process single sample through functional DSP chain - O(1) complexity
    /// Each stage uses pre-computed coefficients for constant-time processing
    pub fn process_sample(&mut self, input: f32, input_gain: f32, drive: f32, output_gain: f32) -> f32 {
        // Functional composition: input -> preamp -> tone -> clipper -> output
        // Each operation is O(1) using lookup tables and pre-computed values
        input
            .pipe(|x| x * input_gain)                    // O(1) multiplication
            .pipe(|x| self.tube_stage.process(x, drive)) // O(1) tube simulation  
            .pipe(|x| self.tonestack.process(x))         // O(1) filter processing
            .pipe(|x| self.clipper.process(x, drive))    // O(1) waveshaping
            .pipe(|x| x * output_gain)                   // O(1) output scaling
    }
    
    /// Update tone controls - O(1) parameter updates
    pub fn update_tone_controls(&mut self, bass_db: f32, mid_db: f32, treble_db: f32) {
        self.tonestack.update_controls(bass_db, mid_db, treble_db);
    }
}

/// Functional extension trait for pipeline composition
trait PipeExt<T> {
    fn pipe<U, F>(self, f: F) -> U 
    where 
        F: FnOnce(T) -> U;
}

impl<T> PipeExt<T> for T {
    /// Enables functional pipeline syntax: value.pipe(fn1).pipe(fn2)
    /// Zero runtime cost - compiles to direct function calls
    fn pipe<U, F>(self, f: F) -> U 
    where 
        F: FnOnce(T) -> U 
    {
        f(self)
    }
}