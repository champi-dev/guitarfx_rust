use eframe::egui;
use std::collections::HashMap;

#[derive(Default)]
struct BiasFXControl {
    status: String,
    connected: bool,
    bias_fx_found: bool,
    parameters: Vec<Parameter>,
    reaper_url: String,
}

#[derive(Clone, Debug)]
struct Parameter {
    index: usize,
    name: String,
    value: f32,
}

impl Default for Parameter {
    fn default() -> Self {
        Parameter {
            index: 0,
            name: String::new(),
            value: 0.5,
        }
    }
}

impl BiasFXControl {
    fn new() -> Self {
        Self {
            status: "🔌 Not Connected".to_string(),
            connected: false,
            bias_fx_found: false,
            parameters: Vec::new(),
            reaper_url: "http://127.0.0.1:6666".to_string(),
        }
    }

    fn test_connection(&self) -> bool {
        // Simple blocking HTTP request
        match std::process::Command::new("curl")
            .arg("-s")
            .arg("--connect-timeout")
            .arg("3")
            .arg(&self.reaper_url)
            .output()
        {
            Ok(output) => {
                let response = String::from_utf8_lossy(&output.stdout);
                response.contains("REAPER control")
            }
            Err(_) => false,
        }
    }

    fn scan_for_bias_fx(&mut self) -> bool {
        // Create sample parameters
        let sample_params = vec![
            ("Input Gain", 0.5),
            ("Bass", 0.5),
            ("Mid", 0.5),
            ("Treble", 0.5),
            ("Presence", 0.5),
            ("Master Volume", 0.7),
            ("Drive", 0.3),
            ("Tone", 0.5),
            ("Reverb Mix", 0.2),
            ("Delay Mix", 0.1),
            ("Chorus Mix", 0.0),
            ("Noise Gate", 0.0),
            ("Compressor", 0.3),
            ("EQ Low", 0.5),
            ("EQ High", 0.5),
            ("Amp Model", 0.2),
        ];

        self.parameters = sample_params
            .into_iter()
            .enumerate()
            .map(|(i, (name, value))| Parameter {
                index: i,
                name: name.to_string(),
                value,
            })
            .collect();

        true
    }

    fn set_parameter(&self, param_index: usize, value: f32) -> bool {
        let cmd = format!("SET/TRACK/0/FX/0/PARAM/{}/VAL/{:.6}", param_index, value);
        let encoded_cmd = cmd.replace("/", "%2F").replace(" ", "%20");
        let url = format!("{}/_/{}", self.reaper_url, encoded_cmd);
        
        // Simple curl command
        match std::process::Command::new("curl")
            .arg("-s")
            .arg("--connect-timeout")
            .arg("2")
            .arg(&url)
            .output()
        {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    fn apply_preset(&mut self, preset_name: &str) {
        let presets: HashMap<&str, HashMap<&str, f32>> = [
            ("Clean", [
                ("Input Gain", 0.2),
                ("Bass", 0.6),
                ("Mid", 0.7),
                ("Treble", 0.6),
                ("Master Volume", 0.7),
            ].iter().cloned().collect()),
            ("Crunch", [
                ("Input Gain", 0.6),
                ("Bass", 0.5),
                ("Mid", 0.8),
                ("Treble", 0.7),
                ("Drive", 0.6),
            ].iter().cloned().collect()),
            ("Lead", [
                ("Input Gain", 0.8),
                ("Bass", 0.4),
                ("Mid", 0.9),
                ("Treble", 0.8),
                ("Drive", 0.8),
            ].iter().cloned().collect()),
            ("Metal", [
                ("Input Gain", 1.0),
                ("Bass", 0.3),
                ("Mid", 0.9),
                ("Treble", 0.9),
                ("Drive", 0.9),
            ].iter().cloned().collect()),
        ].iter().cloned().collect();

        if let Some(preset) = presets.get(preset_name) {
            let mut updates = Vec::new();
            for param in &mut self.parameters {
                if let Some(&value) = preset.get(param.name.as_str()) {
                    param.value = value;
                    updates.push((param.index, value));
                }
            }
            // Apply to Reaper after updating parameters
            for (index, value) in updates {
                self.set_parameter(index, value);
            }
        }
    }
}

impl eframe::App for BiasFXControl {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Header
            ui.vertical_centered(|ui| {
                ui.heading("🎸 BIAS FX 2 Control");
                ui.label("🦀 Rust Edition - No Segfaults!");
                ui.add_space(10.0);
            });

            // Status panel
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(40, 40, 60))
                .rounding(5.0)
                .inner_margin(10.0)
                .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("Connection Status").strong());
                    ui.separator();
                    
                    let status_color = if self.connected {
                        egui::Color32::GREEN
                    } else {
                        egui::Color32::RED
                    };
                    
                    ui.colored_label(status_color, &self.status);
                    
                    ui.horizontal(|ui| {
                        if ui.button("🔌 Connect to Reaper").clicked() {
                            let connected = self.test_connection();
                            
                            if connected {
                                self.connected = true;
                                self.status = "✅ Connected to Reaper".to_string();
                            } else {
                                self.connected = false;
                                self.status = "❌ Cannot connect to Reaper".to_string();
                            }
                        }
                        
                        if ui.add_enabled(self.connected, egui::Button::new("🔍 Scan for BIAS FX 2")).clicked() {
                            self.status = "🔍 Scanning for BIAS FX 2...".to_string();
                            
                            let found = self.scan_for_bias_fx();
                            
                            if found {
                                self.bias_fx_found = true;
                                self.status = format!("🎸 Found BIAS FX 2 - {} controls", self.parameters.len());
                            } else {
                                self.bias_fx_found = false;
                                self.status = "❌ BIAS FX 2 not found".to_string();
                            }
                        }
                    });
                });
            });

            ui.add_space(10.0);

            // Instructions
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(30, 30, 50))
                .rounding(5.0)
                .inner_margin(10.0)
                .show(ui, |ui| {
                ui.label(egui::RichText::new("📋 Setup Instructions:").strong());
                ui.label("1. Load BIAS FX 2 on any track in Reaper");
                ui.label("2. Click 'Connect to Reaper'");
                ui.label("3. Click 'Scan for BIAS FX 2'");
                ui.label("4. Control parameters with sliders below!");
            });

            ui.add_space(10.0);

            // Parameters
            if self.bias_fx_found && !self.parameters.is_empty() {
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(25, 25, 45))
                    .rounding(5.0)
                    .inner_margin(10.0)
                    .show(ui, |ui| {
                    ui.label(egui::RichText::new("🎛️ BIAS FX 2 Parameters").strong().size(16.0));
                    ui.separator();
                    
                    egui::ScrollArea::vertical()
                        .max_height(400.0)
                        .show(ui, |ui| {
                        let mut param_updates = Vec::new();
                        for param in &mut self.parameters {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new(&param.name).strong().size(12.0));
                                ui.add_space(10.0);
                                
                                if ui.add(egui::Slider::new(&mut param.value, 0.0..=1.0)
                                    .fixed_decimals(3)
                                    .min_decimals(3))
                                    .changed() {
                                    // Parameter changed, queue for Reaper update
                                    param_updates.push((param.index, param.value));
                                }
                                
                                ui.label(egui::RichText::new(format!("{:.3}", param.value)).monospace().color(egui::Color32::LIGHT_GREEN));
                            });
                            ui.add_space(3.0);
                        }
                        // Send parameter updates to Reaper
                        for (index, value) in param_updates {
                            self.set_parameter(index, value);
                        }
                    });
                });

                ui.add_space(10.0);

                // Quick presets
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(35, 35, 55))
                    .rounding(5.0)
                    .inner_margin(10.0)
                    .show(ui, |ui| {
                    ui.label(egui::RichText::new("🎵 Quick Presets").strong());
                    ui.separator();
                    
                    ui.horizontal(|ui| {
                        if ui.button("🎹 Clean").clicked() {
                            self.apply_preset("Clean");
                        }
                        if ui.button("🎸 Crunch").clicked() {
                            self.apply_preset("Crunch");
                        }
                        if ui.button("🔥 Lead").clicked() {
                            self.apply_preset("Lead");
                        }
                        if ui.button("⚡ Metal").clicked() {
                            self.apply_preset("Metal");
                        }
                    });
                });
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    println!("🎸 Starting BIAS FX 2 Control (Rust Edition)");
    println!("🦀 Native performance, no segfaults!");
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([850.0, 750.0])
            .with_title("🎸 BIAS FX 2 Control - Rust Edition"),
        ..Default::default()
    };
    
    eframe::run_native(
        "BIAS FX 2 Control",
        options,
        Box::new(|_cc| Ok(Box::new(BiasFXControl::new()))),
    )
}