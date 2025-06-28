use std::path::Path;
use std::fs;

/// Impulse Response Loader for Real Cabinet Simulation
/// Loads WAV files and converts them to f32 arrays for convolution
pub struct IrLoader;

impl IrLoader {
    /// Load impulse response from WAV file with O(1) runtime lookup
    /// Heavy O(N) loading is done once during plugin initialization
    pub fn load_ir_file(file_path: &Path) -> Result<Vec<f32>, IrLoadError> {
        if !file_path.exists() {
            return Err(IrLoadError::FileNotFound);
        }
        
        // Read WAV file - this is a simplified implementation
        // In a real implementation, you'd use a proper WAV reader
        let file_data = fs::read(file_path).map_err(|_| IrLoadError::ReadError)?;
        
        // Basic WAV parsing - looking for 44-byte header + data
        if file_data.len() < 44 {
            return Err(IrLoadError::InvalidFormat);
        }
        
        // Check WAV signature
        if &file_data[0..4] != b"RIFF" || &file_data[8..12] != b"WAVE" {
            return Err(IrLoadError::InvalidFormat);
        }
        
        // Parse basic WAV info from header
        let sample_rate = u32::from_le_bytes([file_data[24], file_data[25], file_data[26], file_data[27]]);
        let bit_depth = u16::from_le_bytes([file_data[34], file_data[35]]);
        let channels = u16::from_le_bytes([file_data[22], file_data[23]]);
        
        // Find data chunk
        let mut data_start = 44;
        let mut data_size = 0;
        
        // Simple data chunk search
        for i in 12..file_data.len().saturating_sub(8) {
            if &file_data[i..i+4] == b"data" {
                data_size = u32::from_le_bytes([
                    file_data[i+4], file_data[i+5], file_data[i+6], file_data[i+7]
                ]) as usize;
                data_start = i + 8;
                break;
            }
        }
        
        if data_size == 0 || data_start + data_size > file_data.len() {
            return Err(IrLoadError::InvalidFormat);
        }
        
        // Convert audio data to f32 based on bit depth
        let audio_data = &file_data[data_start..data_start + data_size];
        let mut samples = Vec::new();
        
        match bit_depth {
            16 => {
                // 16-bit samples
                let bytes_per_sample = 2 * channels as usize;
                for chunk in audio_data.chunks_exact(bytes_per_sample) {
                    if chunk.len() >= 2 {
                        // Take left channel (or mono)
                        let sample_i16 = i16::from_le_bytes([chunk[0], chunk[1]]);
                        let sample_f32 = sample_i16 as f32 / 32768.0;
                        samples.push(sample_f32);
                    }
                }
            }
            24 => {
                // 24-bit samples
                let bytes_per_sample = 3 * channels as usize;
                for chunk in audio_data.chunks_exact(bytes_per_sample) {
                    if chunk.len() >= 3 {
                        // Take left channel (or mono) - 24-bit little endian
                        let sample_i32 = ((chunk[2] as i32) << 16) | 
                                        ((chunk[1] as i32) << 8) | 
                                        (chunk[0] as i32);
                        // Sign extend from 24-bit to 32-bit
                        let sample_i32 = if sample_i32 & 0x800000 != 0 {
                            sample_i32 | 0xFF000000u32 as i32
                        } else {
                            sample_i32
                        };
                        let sample_f32 = sample_i32 as f32 / 8388608.0;
                        samples.push(sample_f32);
                    }
                }
            }
            32 => {
                // Assume 32-bit float
                let bytes_per_sample = 4 * channels as usize;
                for chunk in audio_data.chunks_exact(bytes_per_sample) {
                    if chunk.len() >= 4 {
                        // Take left channel (or mono)
                        let sample_f32 = f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                        samples.push(sample_f32);
                    }
                }
            }
            _ => return Err(IrLoadError::UnsupportedFormat),
        }
        
        // Limit IR length for performance (max 4 seconds at 48kHz = 192k samples)
        if samples.len() > 192000 {
            samples.truncate(192000);
        }
        
        // Normalize to prevent clipping
        let max_sample = samples.iter().map(|s| s.abs()).fold(0.0f32, f32::max);
        if max_sample > 0.0 {
            let gain = 0.5 / max_sample; // Normalize to 50% of full scale
            for sample in &mut samples {
                *sample *= gain;
            }
        }
        
        Ok(samples)
    }
    
    /// Load all cabinet impulse responses from directory
    /// Returns a map of cabinet type to IR samples
    pub fn load_cabinet_irs(ir_directory: &Path) -> Vec<(super::cabinet::CabinetType, Vec<f32>)> {
        let mut irs = Vec::new();
        
        let ir_files = [
            (super::cabinet::CabinetType::Marshall4x12V30, "marshall_4x12_v30.wav"),
            (super::cabinet::CabinetType::FenderTwin2x12, "fender_twin_2x12.wav"),
            (super::cabinet::CabinetType::VoxAC30Blue, "vox_ac30_blue.wav"),
            (super::cabinet::CabinetType::Mesa4x12Recto, "mesa_4x12_recto.wav"),
        ];
        
        for (cabinet_type, filename) in &ir_files {
            let file_path = ir_directory.join(filename);
            
            match Self::load_ir_file(&file_path) {
                Ok(samples) => {
                    println!("✅ Loaded IR: {} ({} samples)", filename, samples.len());
                    irs.push((*cabinet_type, samples));
                }
                Err(e) => {
                    println!("⚠️  Failed to load {}: {:?}", filename, e);
                    // Use fallback - shorter version of built-in IR
                    let fallback = Self::create_fallback_ir(*cabinet_type);
                    irs.push((*cabinet_type, fallback));
                }
            }
        }
        
        irs
    }
    
    /// Create fallback IR if file loading fails
    fn create_fallback_ir(cabinet_type: super::cabinet::CabinetType) -> Vec<f32> {
        use super::cabinet::CabinetType;
        
        // Create a simple but musical IR based on cabinet type
        let mut ir = vec![0.0f32; 512]; // Shorter fallback IR
        
        match cabinet_type {
            CabinetType::Marshall4x12V30 => {
                // Marshall: aggressive attack, midrange emphasis
                ir[0] = 0.8;
                for i in 1..512 {
                    let t = i as f32 / 512.0;
                    let decay = (-t * 12.0).exp();
                    let midrange = (t * std::f32::consts::PI * 8.0).sin() * 0.3 + 1.0;
                    ir[i] = decay * midrange * 0.7;
                }
            }
            CabinetType::FenderTwin2x12 => {
                // Fender: cleaner, scooped response
                ir[0] = 0.7;
                for i in 1..512 {
                    let t = i as f32 / 512.0;
                    let decay = (-t * 15.0).exp();
                    let scoop = 1.0 - (t * std::f32::consts::PI * 6.0).sin().abs() * 0.2;
                    ir[i] = decay * scoop * 0.6;
                }
            }
            CabinetType::VoxAC30Blue => {
                // Vox: vintage character, longer decay
                ir[0] = 0.75;
                for i in 1..512 {
                    let t = i as f32 / 512.0;
                    let decay = (-t * 10.0).exp();
                    let vintage = (t * std::f32::consts::PI * 5.0).sin() * 0.2 + 1.0;
                    ir[i] = decay * vintage * 0.65;
                }
            }
            CabinetType::Mesa4x12Recto => {
                // Mesa: tight, modern response
                ir[0] = 0.85;
                for i in 1..512 {
                    let t = i as f32 / 512.0;
                    let decay = (-t * 18.0).exp();
                    let tight = 1.0 / (1.0 + t * 2.0);
                    ir[i] = decay * tight * 0.75;
                }
            }
            CabinetType::Direct => {
                // Direct: impulse (no processing)
                ir[0] = 1.0;
                for i in 1..512 {
                    ir[i] = 0.0;
                }
            }
        }
        
        ir
    }
}

#[derive(Debug, Clone)]
pub enum IrLoadError {
    FileNotFound,
    ReadError,
    InvalidFormat,
    UnsupportedFormat,
}

impl std::fmt::Display for IrLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IrLoadError::FileNotFound => write!(f, "IR file not found"),
            IrLoadError::ReadError => write!(f, "Failed to read IR file"),
            IrLoadError::InvalidFormat => write!(f, "Invalid WAV format"),
            IrLoadError::UnsupportedFormat => write!(f, "Unsupported audio format"),
        }
    }
}

impl std::error::Error for IrLoadError {}