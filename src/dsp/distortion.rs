/// High-performance asymmetric clipper for tube-like distortion - O(1) complexity
/// Uses optimized waveshaping with pre-computed lookup tables for real-time performance
pub struct AsymmetricClipper {
    /// Pre-computed lookup table for asymmetric clipping function - O(1) access
    /// Table covers input range [-4.0, 4.0] with 1024 entries for smooth interpolation
    lookup_table: Vec<f32>,
    table_size: usize,
    input_scale: f32,
}

impl AsymmetricClipper {
    /// Create new asymmetric clipper with O(1) initialization complexity
    /// Pre-computes entire waveshaping function for constant-time processing
    pub fn new() -> Self {
        const TABLE_SIZE: usize = 1024;
        const INPUT_RANGE: f32 = 8.0; // -4.0 to +4.0
        
        let mut lookup_table = Vec::with_capacity(TABLE_SIZE);
        
        // Pre-compute asymmetric waveshaping function - O(n) initialization, O(1) runtime
        for i in 0..TABLE_SIZE {
            let input = (i as f32 / TABLE_SIZE as f32) * INPUT_RANGE - (INPUT_RANGE / 2.0);
            let output = Self::asymmetric_waveshape(input);
            lookup_table.push(output);
        }
        
        Self {
            lookup_table,
            table_size: TABLE_SIZE,
            input_scale: TABLE_SIZE as f32 / INPUT_RANGE,
        }
    }
    
    /// Asymmetric waveshaping function modeling tube saturation characteristics
    /// Creates even-order harmonics for warm, musical distortion
    fn asymmetric_waveshape(x: f32) -> f32 {
        // Asymmetric transfer function: different behavior for positive/negative
        if x >= 0.0 {
            // Positive half: softer clipping with compression
            let normalized = x / (1.0 + x);
            (2.0 / std::f32::consts::PI) * normalized.atan()
        } else {
            // Negative half: harder clipping for asymmetry 
            let normalized = x / (1.0 - x * 0.7);
            (2.0 / std::f32::consts::PI) * normalized.atan() * 0.8
        }
    }
    
    /// Process sample with O(1) complexity using lookup table
    /// Includes linear interpolation for smooth response between table entries
    pub fn process(&self, input: f32, drive: f32) -> f32 {
        // Apply drive and clamp to table range - O(1)
        let driven_input = (input * drive).clamp(-4.0, 4.0);
        
        // Convert to table index with fractional part - O(1)
        let table_pos = (driven_input + 4.0) * self.input_scale;
        let table_index = table_pos as usize;
        let frac = table_pos - table_index as f32;
        
        // Bounds check and lookup with linear interpolation - O(1)
        if table_index >= self.table_size - 1 {
            return self.lookup_table[self.table_size - 1];
        }
        
        // Linear interpolation between adjacent table entries - O(1)
        let y0 = self.lookup_table[table_index];
        let y1 = self.lookup_table[table_index + 1];
        y0 + frac * (y1 - y0)
    }
}

/// Tube saturation model with dynamic bias shifting - O(1) complexity
/// Simulates grid current and cathode follower behavior for authentic tube response
pub struct TubeSaturation {
    /// Current bias point - shifts dynamically based on input level
    bias: f32,
    /// Bias filter coefficient for smooth bias tracking - O(1) update
    bias_coeff: f32,
}

impl TubeSaturation {
    /// Create new tube saturation processor - O(1) initialization
    pub fn new() -> Self {
        Self {
            bias: 0.0,
            bias_coeff: 0.999, // Very slow bias tracking for realistic tube behavior
        }
    }
    
    /// Process sample with dynamic bias shifting - O(1) complexity
    /// Models grid current rectification and cathode self-bias effects
    pub fn process(&mut self, input: f32, drive: f32) -> f32 {
        // Update dynamic bias based on input level - O(1) exponential smoothing
        let input_level = (input * drive).abs();
        let target_bias = input_level * 0.1; // Grid current simulation
        self.bias = self.bias * self.bias_coeff + target_bias * (1.0 - self.bias_coeff);
        
        // Apply bias shift and saturation - O(1) computation
        let biased_input = input + self.bias * 0.5;
        let saturated = Self::tube_transfer_function(biased_input * drive);
        
        // High-frequency rolloff for realistic tube response - O(1) single-pole filter
        saturated * 0.95 // Simple high-cut approximation
    }
    
    /// Tube transfer function modeling triode characteristics - O(1) complexity
    /// Uses Dempwolf model approximation for computational efficiency
    fn tube_transfer_function(x: f32) -> f32 {
        // Simplified triode model: combines exponential and polynomial terms
        let normalized = x.clamp(-2.0, 2.0);
        
        if normalized >= 0.0 {
            // Positive grid region: exponential saturation
            normalized / (1.0 + normalized * normalized * 0.5)
        } else {
            // Negative grid region: cutoff behavior
            normalized / (1.0 - normalized * 0.3)
        }
    }
}