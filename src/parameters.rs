use nih_plug::prelude::*;
use crate::dsp::CabinetType;

#[derive(Params)]
pub struct GuitarFxParams {
    /// Input gain with smooth parameter changes for O(1) real-time performance
    #[id = "input_gain"]
    pub input_gain: FloatParam,
    
    /// Drive amount for distortion effect
    #[id = "drive"]  
    pub drive: FloatParam,
    
    /// Low frequency control (bass)
    #[id = "bass"]
    pub bass: FloatParam,
    
    /// Mid frequency control 
    #[id = "mid"]
    pub mid: FloatParam,
    
    /// High frequency control (treble)
    #[id = "treble"] 
    pub treble: FloatParam,
    
    /// Output gain with smooth parameter changes
    #[id = "output_gain"]
    pub output_gain: FloatParam,
    
    /// Cabinet type selection for professional speaker simulation
    #[id = "cabinet_type"]
    pub cabinet_type: EnumParam<CabinetType>,
    
    /// Cabinet wet/dry mix for blending direct and cabinet-processed signal
    #[id = "cabinet_mix"]
    pub cabinet_mix: FloatParam,
}

impl Default for GuitarFxParams {
    fn default() -> Self {
        Self {
            input_gain: FloatParam::new(
                "Input Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            
            drive: FloatParam::new(
                "Drive", 
                1.0,
                FloatRange::Linear { min: 1.0, max: 20.0 }
            )
            .with_smoother(SmoothingStyle::Linear(50.0)),
            
            bass: FloatParam::new(
                "Bass",
                0.0,
                FloatRange::Linear { min: -12.0, max: 12.0 }
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit(" dB"),
            
            mid: FloatParam::new(
                "Mid", 
                0.0,
                FloatRange::Linear { min: -12.0, max: 12.0 }
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit(" dB"),
            
            treble: FloatParam::new(
                "Treble",
                0.0, 
                FloatRange::Linear { min: -12.0, max: 12.0 }
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit(" dB"),
            
            output_gain: FloatParam::new(
                "Output Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            
            cabinet_type: EnumParam::new(
                "Cabinet",
                CabinetType::Marshall4x12V30
            ),
            
            cabinet_mix: FloatParam::new(
                "Cabinet Mix",
                1.0,
                FloatRange::Linear { min: 0.0, max: 1.0 }
            )
            .with_smoother(SmoothingStyle::Linear(10.0))
            .with_unit("%")
            .with_value_to_string(formatters::v2s_f32_percentage(1))
            .with_string_to_value(formatters::s2v_f32_percentage()),
        }
    }
}