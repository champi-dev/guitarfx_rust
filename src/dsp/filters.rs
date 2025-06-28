use std::f32::consts::PI;

/// High-performance biquad filter with O(1) processing complexity
/// Uses direct form II transposed for numerical stability
#[derive(Clone)]
pub struct BiquadFilter {
    // Feedforward coefficients - pre-computed for O(1) access
    b0: f32,
    b1: f32, 
    b2: f32,
    
    // Feedback coefficients - pre-computed for O(1) access
    a1: f32,
    a2: f32,
    
    // State variables for O(1) processing
    z1: f32,
    z2: f32,
}

impl BiquadFilter {
    /// Create new biquad filter with O(1) initialization
    pub fn new() -> Self {
        Self {
            b0: 1.0, b1: 0.0, b2: 0.0,
            a1: 0.0, a2: 0.0,
            z1: 0.0, z2: 0.0,
        }
    }
    
    /// Configure as peaking EQ filter - O(1) coefficient calculation
    /// Uses optimized formulas avoiding transcendental function calls in real-time
    pub fn peaking_eq(&mut self, freq: f32, gain_db: f32, q: f32, sample_rate: f32) {
        let a = 10.0_f32.powf(gain_db / 40.0);
        let w = 2.0 * PI * freq / sample_rate;
        let cos_w = w.cos();
        let sin_w = w.sin();
        let alpha = sin_w / (2.0 * q);
        
        // Pre-compute all coefficients for O(1) runtime performance
        let b0 = 1.0 + alpha * a;
        let b1 = -2.0 * cos_w;
        let b2 = 1.0 - alpha * a;
        let a0 = 1.0 + alpha / a;
        let a1 = -2.0 * cos_w;
        let a2 = 1.0 - alpha / a;
        
        // Normalize coefficients - O(1) division
        self.b0 = b0 / a0;
        self.b1 = b1 / a0;
        self.b2 = b2 / a0;
        self.a1 = a1 / a0;
        self.a2 = a2 / a0;
    }
    
    /// Process single sample - O(1) complexity using pre-computed coefficients
    /// Direct form II transposed: most efficient for real-time processing
    pub fn process(&mut self, input: f32) -> f32 {
        let output = self.b0 * input + self.z1;
        self.z1 = self.b1 * input - self.a1 * output + self.z2;
        self.z2 = self.b2 * input - self.a2 * output;
        output
    }
    
    /// Reset filter state - O(1) complexity
    pub fn reset(&mut self) {
        self.z1 = 0.0;
        self.z2 = 0.0;
    }
    
    /// Set filter coefficients directly - O(1) assignment
    pub fn set_coefficients(&mut self, b0: f32, b1: f32, b2: f32, a1: f32, a2: f32) {
        self.b0 = b0;
        self.b1 = b1;
        self.b2 = b2;
        self.a1 = a1;
        self.a2 = a2;
    }
}

/// Guitar amplifier tone stack simulation - O(1) processing complexity
/// Models classic Fender/Marshall tone circuit with functional composition
pub struct ToneStack {
    bass_filter: BiquadFilter,
    mid_filter: BiquadFilter,
    treble_filter: BiquadFilter,
    sample_rate: f32,
}

impl ToneStack {
    /// Create new tone stack with O(1) initialization
    /// Pre-configures filters for guitar frequency response
    pub fn new(sample_rate: f32) -> Self {
        let mut stack = Self {
            bass_filter: BiquadFilter::new(),
            mid_filter: BiquadFilter::new(), 
            treble_filter: BiquadFilter::new(),
            sample_rate,
        };
        
        // Initialize with neutral settings - O(1) setup
        stack.update_controls(0.0, 0.0, 0.0);
        stack
    }
    
    /// Update tone controls - O(1) coefficient updates
    /// Each filter update is O(1) using pre-computed formulas
    pub fn update_controls(&mut self, bass_db: f32, mid_db: f32, treble_db: f32) {
        // Configure filters for guitar-optimized frequency response
        self.bass_filter.peaking_eq(100.0, bass_db, 0.7, self.sample_rate);      // O(1)
        self.mid_filter.peaking_eq(500.0, mid_db, 1.0, self.sample_rate);       // O(1)  
        self.treble_filter.peaking_eq(3000.0, treble_db, 0.7, self.sample_rate); // O(1)
    }
    
    /// Process sample through tone stack - O(1) complexity
    /// Functional composition: bass -> mid -> treble processing chain
    pub fn process(&mut self, input: f32) -> f32 {
        input
            .pipe(|x| self.bass_filter.process(x))    // O(1) bass filtering
            .pipe(|x| self.mid_filter.process(x))     // O(1) mid filtering  
            .pipe(|x| self.treble_filter.process(x))  // O(1) treble filtering
    }
}

/// Create optimized EQ biquad filter - O(1) factory function
pub fn create_biquad_eq(freq: f32, gain_db: f32, q: f32, sample_rate: f32) -> BiquadFilter {
    let mut filter = BiquadFilter::new();
    filter.peaking_eq(freq, gain_db, q, sample_rate);
    filter
}

/// Functional extension for pipeline composition
trait PipeExt<T> {
    fn pipe<U, F>(self, f: F) -> U where F: FnOnce(T) -> U;
}

impl<T> PipeExt<T> for T {
    fn pipe<U, F>(self, f: F) -> U where F: FnOnce(T) -> U {
        f(self)
    }
}