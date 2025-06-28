use super::distortion::{AsymmetricClipper, TubeSaturation};
use super::filters::BiquadFilter;

/// Complete tube amplifier stage simulation - O(1) processing complexity
/// Combines preamp, tonestack, and power amp stages in functional pipeline
pub struct TubeStage {
    /// Input tube saturation stage - O(1) processing
    input_tube: TubeSaturation,
    
    /// Asymmetric clipper for additional harmonic content - O(1) lookup
    clipper: AsymmetricClipper,
    
    /// High-frequency rolloff filter for realistic tube response - O(1) filtering
    hf_rolloff: BiquadFilter,
    
    /// DC blocking filter to prevent bias drift - O(1) high-pass filtering
    dc_blocker: BiquadFilter,
}

impl TubeStage {
    /// Create new tube stage with O(1) initialization complexity
    /// Pre-configures all filters for optimal guitar processing
    pub fn new() -> Self {
        let mut stage = Self {
            input_tube: TubeSaturation::new(),
            clipper: AsymmetricClipper::new(),
            hf_rolloff: BiquadFilter::new(),
            dc_blocker: BiquadFilter::new(),
        };
        
        // Configure filters for authentic tube response - O(1) setup
        stage.initialize_filters(44100.0);
        stage
    }
    
    /// Initialize filters with sample rate - O(1) coefficient calculation
    fn initialize_filters(&mut self, sample_rate: f32) {
        // High-frequency rolloff at 8kHz for tube warmth - O(1) setup
        self.hf_rolloff.low_pass(8000.0, 0.707, sample_rate);
        
        // DC blocking at 20Hz to prevent bias accumulation - O(1) setup  
        self.dc_blocker.high_pass(20.0, 0.707, sample_rate);
    }
    
    /// Process sample through complete tube stage - O(1) complexity
    /// Functional pipeline: input -> tube -> clip -> filter -> output
    pub fn process(&mut self, input: f32, drive: f32) -> f32 {
        input
            .pipe(|x| self.input_tube.process(x, drive))      // O(1) tube saturation
            .pipe(|x| self.clipper.process(x, drive * 0.5))   // O(1) asymmetric clipping
            .pipe(|x| self.hf_rolloff.process(x))             // O(1) high-frequency rolloff
            .pipe(|x| self.dc_blocker.process(x))             // O(1) DC blocking
    }
}

/// Power amplifier simulation with compression and saturation - O(1) complexity
/// Models output transformer saturation and speaker loading effects
pub struct PowerAmp {
    /// Compression amount based on output level - O(1) tracking
    compression_level: f32,
    
    /// Compression time constant for realistic power amp response
    compression_coeff: f32,
    
    /// Output transformer saturation model - O(1) processing
    transformer_sat: f32,
}

impl PowerAmp {
    /// Create new power amp simulation - O(1) initialization
    pub fn new() -> Self {
        Self {
            compression_level: 0.0,
            compression_coeff: 0.9995, // Slow compression for power amp feel
            transformer_sat: 0.0,
        }
    }
    
    /// Process sample through power amp stage - O(1) complexity
    /// Models output compression and transformer saturation
    pub fn process(&mut self, input: f32, volume: f32) -> f32 {
        // Calculate output level for compression - O(1)
        let output_level = (input * volume).abs();
        
        // Update compression with exponential smoothing - O(1)
        let target_compression = (output_level - 0.7).max(0.0) * 0.3;
        self.compression_level = self.compression_level * self.compression_coeff 
            + target_compression * (1.0 - self.compression_coeff);
        
        // Apply compression to input - O(1)
        let compression_factor = 1.0 / (1.0 + self.compression_level);
        let compressed_input = input * compression_factor;
        
        // Output transformer saturation - O(1) soft clipping
        let driven_signal = compressed_input * volume;
        Self::transformer_saturation(driven_signal)
    }
    
    /// Output transformer saturation model - O(1) complexity
    /// Simulates magnetic core saturation for warm power amp distortion
    fn transformer_saturation(input: f32) -> f32 {
        // Soft saturation curve modeling transformer core saturation
        let normalized = input.clamp(-2.0, 2.0);
        normalized / (1.0 + normalized.abs() * 0.4)
    }
}

/// Complete amplifier head simulation - O(1) processing complexity
/// Combines preamp, tonestack, and power amp in single functional unit
pub struct AmpHead {
    preamp: TubeStage,
    power_amp: PowerAmp,
}

impl AmpHead {
    /// Create new amp head - O(1) initialization
    pub fn new() -> Self {
        Self {
            preamp: TubeStage::new(),
            power_amp: PowerAmp::new(),
        }
    }
    
    /// Process sample through complete amp - O(1) complexity
    /// Full signal chain: preamp -> power amp processing
    pub fn process(&mut self, input: f32, drive: f32, volume: f32) -> f32 {
        input
            .pipe(|x| self.preamp.process(x, drive))   // O(1) preamp processing
            .pipe(|x| self.power_amp.process(x, volume)) // O(1) power amp processing
    }
}

/// Functional extension trait for pipeline composition
trait PipeExt<T> {
    fn pipe<U, F>(self, f: F) -> U where F: FnOnce(T) -> U;
}

impl<T> PipeExt<T> for T {
    fn pipe<U, F>(self, f: F) -> U where F: FnOnce(T) -> U {
        f(self)
    }
}

/// Filter extensions for biquad configuration
impl BiquadFilter {
    /// Configure as low-pass filter - O(1) coefficient calculation
    pub fn low_pass(&mut self, freq: f32, q: f32, sample_rate: f32) {
        let w = 2.0 * std::f32::consts::PI * freq / sample_rate;
        let cos_w = w.cos();
        let sin_w = w.sin();
        let alpha = sin_w / (2.0 * q);
        
        let b0 = (1.0 - cos_w) / 2.0;
        let b1 = 1.0 - cos_w;
        let b2 = (1.0 - cos_w) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_w;
        let a2 = 1.0 - alpha;
        
        self.set_coefficients(b0/a0, b1/a0, b2/a0, a1/a0, a2/a0);
    }
    
    /// Configure as high-pass filter - O(1) coefficient calculation  
    pub fn high_pass(&mut self, freq: f32, q: f32, sample_rate: f32) {
        let w = 2.0 * std::f32::consts::PI * freq / sample_rate;
        let cos_w = w.cos();
        let sin_w = w.sin();
        let alpha = sin_w / (2.0 * q);
        
        let b0 = (1.0 + cos_w) / 2.0;
        let b1 = -(1.0 + cos_w);
        let b2 = (1.0 + cos_w) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_w;
        let a2 = 1.0 - alpha;
        
        self.set_coefficients(b0/a0, b1/a0, b2/a0, a1/a0, a2/a0);
    }
    
}