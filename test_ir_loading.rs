// Simple test for IR loading functionality
use std::path::Path;

mod dsp;
use dsp::cabinet::{CabinetSimulator, CabinetType};

fn main() {
    println!("🔧 Testing BIAS FX Rust IR Loading System");
    println!("==========================================");
    
    // Test cabinet simulator initialization
    let mut cabinet = CabinetSimulator::new(256, 44100.0);
    
    println!("✅ Cabinet simulator initialized");
    println!("📊 Current cabinet: {:?}", cabinet.get_current_cabinet());
    println!("⏱️  Latency: {} samples", cabinet.get_latency());
    
    // Test cabinet switching
    println!("\n🔄 Testing cabinet switching...");
    
    let cabinets = [
        CabinetType::Marshall4x12V30,
        CabinetType::FenderTwin2x12,
        CabinetType::VoxAC30Blue,
        CabinetType::Mesa4x12Recto,
        CabinetType::Direct,
    ];
    
    for cabinet_type in &cabinets {
        match cabinet.load_cabinet(*cabinet_type) {
            Ok(()) => {
                println!("  ✅ Loaded: {:?}", cabinet_type);
                
                // Test processing a few samples
                let test_input = 0.5;
                let output = cabinet.process_sample(test_input);
                println!("     Input: {:.3}, Output: {:.3}", test_input, output);
            }
            Err(e) => {
                println!("  ❌ Failed to load {:?}: {}", cabinet_type, e);
            }
        }
    }
    
    // Test mix control
    println!("\n🎛️  Testing cabinet mix control...");
    cabinet.load_cabinet(CabinetType::Marshall4x12V30).unwrap();
    
    let test_input = 0.5;
    
    cabinet.set_mix(0.0); // Dry
    let dry_output = cabinet.process_sample(test_input);
    
    cabinet.set_mix(1.0); // Wet
    let wet_output = cabinet.process_sample(test_input);
    
    println!("  Mix 0% (dry): {:.3}", dry_output);
    println!("  Mix 100% (wet): {:.3}", wet_output);
    
    if (dry_output - wet_output).abs() > 0.001 {
        println!("  ✅ Cabinet mix control working - noticeable difference");
    } else {
        println!("  ⚠️  Cabinet mix may not be working properly");
    }
    
    println!("\n🎸 IR Loading Test Complete!");
}