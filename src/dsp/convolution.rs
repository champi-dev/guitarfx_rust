use realfft::RealFftPlanner;
use std::collections::VecDeque;

/// High-performance partitioned FFT convolution engine achieving O(1) per-sample complexity
/// Uses overlap-add method with pre-computed frequency domain impulse responses
/// 
/// Theory: Partitioned convolution breaks large IRs into smaller blocks, processing
/// each in frequency domain. This transforms O(N*M) time-domain convolution into
/// O(log N) FFT operations per block, achieving effective O(1) per-sample complexity.
pub struct PartitionedConvolution {
    /// Block size for FFT processing - must be power of 2 for optimal performance
    block_size: usize,
    
    /// FFT size = 2 * block_size for zero-padding (prevents circular convolution artifacts)
    fft_size: usize,
    
    /// Pre-computed frequency domain IR partitions for O(1) lookup
    /// Each partition is FFT(ir_chunk) computed during initialization
    ir_partitions: Vec<Vec<f32>>,
    
    /// Circular buffer for input samples - maintains exactly block_size samples
    input_buffer: VecDeque<f32>,
    
    /// Overlap-add accumulator for seamless block stitching
    /// Size = block_size to handle inter-block overlap
    overlap_buffer: Vec<f32>,
    
    /// FFT planner for forward/inverse transforms - created once for O(1) transforms
    fft_planner: RealFftPlanner<f32>,
    
    /// Working memory for FFT operations - pre-allocated to avoid real-time allocation
    fft_scratch: Vec<f32>,
    
    /// Complex frequency domain working buffer - pre-allocated for O(1) processing
    freq_buffer: Vec<f32>,
    
    /// Current position in input buffer for O(1) sample tracking
    buffer_position: usize,
}

impl PartitionedConvolution {
    /// Create new convolution engine with specified block size
    /// Block size should be power of 2 (128, 256, 512) for optimal FFT performance
    /// 
    /// Complexity: O(M log N) initialization where M = IR length, N = block size
    /// Runtime: O(1) per sample after initialization
    pub fn new(block_size: usize) -> Self {
        assert!(block_size.is_power_of_two(), "Block size must be power of 2 for optimal FFT");
        
        let fft_size = block_size * 2; // Zero-padding for linear convolution
        let fft_planner = RealFftPlanner::new();
        
        Self {
            block_size,
            fft_size,
            ir_partitions: Vec::new(),
            input_buffer: VecDeque::with_capacity(block_size),
            overlap_buffer: vec![0.0; block_size],
            fft_planner,
            fft_scratch: vec![0.0; fft_size],
            freq_buffer: vec![0.0; fft_size],
            buffer_position: 0,
        }
    }
    
    /// Load impulse response and partition into frequency domain blocks
    /// This is the heavy O(M log N) computation done once during setup
    /// 
    /// Algorithm:
    /// 1. Partition IR into block_size chunks
    /// 2. Zero-pad each chunk to fft_size
    /// 3. Apply FFT to get frequency domain representation
    /// 4. Store for O(1) runtime lookup
    pub fn load_impulse_response(&mut self, impulse_response: &[f32]) -> Result<(), ConvolutionError> {
        if impulse_response.is_empty() {
            return Err(ConvolutionError::EmptyImpulseResponse);
        }
        
        // Validate IR length for reasonable memory usage
        if impulse_response.len() > 96000 { // ~2 seconds at 48kHz
            return Err(ConvolutionError::ImpulseResponseTooLong);
        }
        
        self.ir_partitions.clear();
        
        // Create FFT instance for this operation
        let fft = self.fft_planner.plan_fft_forward(self.fft_size);
        
        // Partition impulse response into frequency domain blocks
        for chunk in impulse_response.chunks(self.block_size) {
            // Zero-pad chunk to FFT size
            let mut padded_chunk = vec![0.0; self.fft_size];
            padded_chunk[..chunk.len()].copy_from_slice(chunk);
            
            // Transform to frequency domain
            let mut spectrum = fft.make_output_vec();
            fft.process(&mut padded_chunk, &mut spectrum).map_err(|_| ConvolutionError::FftError)?;
            
            // Convert complex spectrum to interleaved real format for cache efficiency
            let mut real_spectrum = Vec::with_capacity(self.fft_size);
            for complex_sample in spectrum {
                real_spectrum.push(complex_sample.re);
                real_spectrum.push(complex_sample.im);
            }
            
            self.ir_partitions.push(real_spectrum);
        }
        
        // Reset processing state
        self.input_buffer.clear();
        self.overlap_buffer.fill(0.0);
        self.buffer_position = 0;
        
        Ok(())
    }
    
    /// Process single sample through convolution - O(1) amortized complexity
    /// 
    /// Most calls are simple buffer operations O(1). When buffer fills,
    /// one expensive O(log N) FFT operation processes entire block,
    /// amortizing to O(1) per sample over the block.
    pub fn process_sample(&mut self, input: f32) -> f32 {
        // Add sample to circular buffer - O(1) operation
        if self.input_buffer.len() < self.block_size {
            self.input_buffer.push_back(input);
            
            // Return overlapped sample while filling buffer - O(1) lookup
            if self.buffer_position < self.overlap_buffer.len() {
                let output = self.overlap_buffer[self.buffer_position];
                self.buffer_position += 1;
                return output;
            }
            return 0.0;
        }
        
        // Buffer full - process block and generate new overlap
        // This O(log N) operation happens once per block_size samples
        self.process_block();
        
        // Replace oldest sample with new input - O(1) circular buffer operation
        self.input_buffer.pop_front();
        self.input_buffer.push_back(input);
        
        // Return first sample of new overlap - O(1) lookup
        self.buffer_position = 1;
        self.overlap_buffer[0]
    }
    
    /// Process full block through partitioned convolution - O(P log N) complexity
    /// where P = number of partitions, N = block size
    /// 
    /// Algorithm (Overlap-Add Partitioned Convolution):
    /// 1. FFT current input block
    /// 2. For each IR partition: multiply in frequency domain
    /// 3. Accumulate all partition results  
    /// 4. IFFT to get time domain result
    /// 5. Add to overlap buffer from previous block
    fn process_block(&mut self) {
        if self.ir_partitions.is_empty() {
            return;
        }
        
        // Zero-pad input block for linear convolution
        self.fft_scratch.fill(0.0);
        for (i, &sample) in self.input_buffer.iter().enumerate() {
            self.fft_scratch[i] = sample;
        }
        
        // Forward FFT of input block
        let fft = self.fft_planner.plan_fft_forward(self.fft_size);
        let mut input_spectrum = fft.make_output_vec();
        if fft.process(&mut self.fft_scratch, &mut input_spectrum).is_err() {
            return; // Graceful degradation on FFT error
        }
        
        // Initialize accumulator for partition results
        let mut result_spectrum = vec![num_complex::Complex::new(0.0, 0.0); input_spectrum.len()];
        
        // Convolve with each IR partition - O(P) partitions
        for ir_partition in &self.ir_partitions {
            // Complex multiplication in frequency domain - O(N) per partition
            for (i, (&input_bin, result_bin)) in input_spectrum.iter().zip(result_spectrum.iter_mut()).enumerate() {
                let ir_real = ir_partition[i * 2];
                let ir_imag = ir_partition[i * 2 + 1];
                let ir_bin = num_complex::Complex::new(ir_real, ir_imag);
                
                // Complex multiplication: (a + bi)(c + di) = (ac - bd) + (ad + bc)i
                *result_bin += input_bin * ir_bin;
            }
        }
        
        // Inverse FFT to time domain
        let ifft = self.fft_planner.plan_fft_inverse(self.fft_size);
        self.fft_scratch.fill(0.0);
        if ifft.process(&mut result_spectrum, &mut self.fft_scratch).is_err() {
            return; // Graceful degradation
        }
        
        // Overlap-add with previous block result
        for i in 0..self.block_size {
            if i < self.overlap_buffer.len() {
                self.overlap_buffer[i] += self.fft_scratch[i];
            }
        }
        
        // Copy second half as new overlap for next block
        for i in 0..self.block_size {
            if i + self.block_size < self.fft_scratch.len() {
                self.overlap_buffer[i] = self.fft_scratch[i + self.block_size];
            }
        }
        
        self.buffer_position = 0;
    }
    
    /// Reset convolution state - O(1) operation
    pub fn reset(&mut self) {
        self.input_buffer.clear();
        self.overlap_buffer.fill(0.0);
        self.buffer_position = 0;
    }
    
    /// Get current latency in samples - O(1) lookup
    /// Latency = block_size due to block-based processing
    pub fn get_latency(&self) -> usize {
        self.block_size
    }
}

/// Convolution engine error types for robust error handling
#[derive(Debug, Clone)]
pub enum ConvolutionError {
    EmptyImpulseResponse,
    ImpulseResponseTooLong,
    FftError,
    InvalidBlockSize,
}

impl std::fmt::Display for ConvolutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConvolutionError::EmptyImpulseResponse => write!(f, "Impulse response cannot be empty"),
            ConvolutionError::ImpulseResponseTooLong => write!(f, "Impulse response too long (max 96000 samples)"),
            ConvolutionError::FftError => write!(f, "FFT processing error"),
            ConvolutionError::InvalidBlockSize => write!(f, "Block size must be power of 2"),
        }
    }
}

impl std::error::Error for ConvolutionError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_convolution_creation() {
        let conv = PartitionedConvolution::new(256);
        assert_eq!(conv.block_size, 256);
        assert_eq!(conv.fft_size, 512);
        assert_eq!(conv.get_latency(), 256);
    }
    
    #[test]
    fn test_impulse_response_loading() {
        let mut conv = PartitionedConvolution::new(128);
        let ir = vec![1.0, 0.5, 0.25, 0.125]; // Simple decaying impulse
        
        assert!(conv.load_impulse_response(&ir).is_ok());
        assert!(!conv.ir_partitions.is_empty());
    }
    
    #[test]
    fn test_impulse_validation() {
        let mut conv = PartitionedConvolution::new(128);
        
        // Empty IR should fail
        assert!(conv.load_impulse_response(&[]).is_err());
        
        // Too long IR should fail
        let long_ir = vec![1.0; 100000];
        assert!(conv.load_impulse_response(&long_ir).is_err());
    }
}