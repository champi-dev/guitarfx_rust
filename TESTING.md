# Testing BIAS FX Rust Plugin in REAPER on Linux

## Prerequisites

1. **REAPER installed on Linux**
   ```bash
   # Download from https://www.reaper.fm/download.php
   # Or install via package manager if available
   ```

2. **Audio system configured**
   ```bash
   # Install JACK for professional audio (recommended)
   sudo apt install jackd2 qjackctl
   
   # Or use ALSA/PulseAudio for basic testing
   ```

## Step 1: Build the Plugin

```bash
# In the project directory
./build_plugin.sh
```

This creates: `target/release/BiasFxRust.vst3/`

## Step 2: Install Plugin for REAPER

### Option A: User Installation (Recommended for testing)
```bash
# Create VST3 directory if it doesn't exist
mkdir -p ~/.vst3

# Copy plugin bundle
cp -r target/release/BiasFxRust.vst3 ~/.vst3/

# Verify installation
ls -la ~/.vst3/BiasFxRust.vst3/
```

### Option B: System-wide Installation
```bash
# Install system-wide (requires sudo)
sudo cp -r target/release/BiasFxRust.vst3 /usr/lib/vst3/
```

## Step 3: Configure REAPER

1. **Start REAPER**
   ```bash
   reaper
   ```

2. **Configure Audio Device**
   - Go to `Options → Preferences → Audio → Device`
   - Select your audio interface or JACK
   - Set buffer size: 256-1024 samples (lower = less latency)
   - Sample rate: 44.1kHz or 48kHz

3. **Scan for Plugins**
   - Go to `Options → Preferences → Plugins → VST`
   - Add path: `/home/$(whoami)/.vst3` (if using user install)
   - Click "Re-scan" or "Clear cache and re-scan"
   - Verify "BIAS FX Rust" appears in the plugin list

## Step 4: Load and Test Plugin

### Basic Setup:
1. **Create new project**: `Ctrl+N`
2. **Create audio track**: `Ctrl+T`
3. **Add plugin**:
   - Click "FX" button on track
   - Navigate to "VST3" → "BIAS FX Rust"
   - Double-click to load

### Test with Guitar Input:
1. **Connect guitar** to audio interface
2. **Arm track for recording**: Click red record button on track
3. **Enable input monitoring**: Click speaker icon on track
4. **Play guitar** - you should hear processed sound

### Test with Audio File:
1. **Import audio file**: `Ctrl+Alt+I`
2. **Drag guitar DI track** to timeline
3. **Add BIAS FX Rust** to track FX chain
4. **Press spacebar** to play

## Step 5: Parameter Testing

### Test Each Parameter:
- **Input Gain**: -30dB to +30dB range
- **Drive**: 1.0 to 20.0 for distortion amount
- **Bass**: -12dB to +12dB at 100Hz
- **Mid**: -12dB to +12dB at 500Hz  
- **Treble**: -12dB to +12dB at 3kHz
- **Cabinet Type**: Marshall V30, Fender Twin, Vox AC30, Mesa Recto, Direct
- **Cabinet Mix**: 0% (dry) to 100% (wet cabinet simulation)
- **Output Gain**: -30dB to +30dB range

### Test Automation:
1. **Right-click parameter** → "Show in track automation lane"
2. **Draw automation curves**
3. **Play project** - parameters should change smoothly

## Step 6: Performance Testing

### Monitor CPU Usage:
```bash
# In terminal, monitor CPU usage
htop
# Look for REAPER process CPU usage
```

### Test Different Buffer Sizes:
- **64 samples**: Very low latency, higher CPU load
- **128 samples**: Low latency, moderate CPU load  
- **256 samples**: Plugin's native block size, optimal performance
- **512 samples**: Higher latency, lower CPU load
- **1024 samples**: High latency, lowest CPU load

### Latency Verification:
- Plugin reports **256 samples latency** to REAPER
- REAPER should show this in track delay compensation
- Check: Track → Track delay compensation

## Step 7: Troubleshooting

### Plugin Not Found:
```bash
# Check plugin was built
ls -la target/release/BiasFxRust.vst3/Contents/x86_64-linux/

# Check plugin was installed  
ls -la ~/.vst3/BiasFxRust.vst3/

# Check REAPER plugin paths
# In REAPER: Options → Preferences → Plugins → VST → Show plugin paths
```

### Audio Issues:
```bash
# Check audio permissions
groups $USER
# Should include 'audio' group

# Add user to audio group if missing
sudo usermod -a -G audio $USER
# Then logout/login

# Test audio system
speaker-test -c 2 -t sine
```

### Plugin Crashes:
```bash
# Check REAPER error log
tail -f ~/.config/REAPER/reaper.log

# Run REAPER from terminal to see debug output
reaper
```

### Performance Issues:
- Increase audio buffer size in REAPER
- Close other audio applications
- Check CPU frequency scaling:
  ```bash
  cpupower frequency-info
  ```

## Step 8: A/B Testing

### Compare with Reference:
1. **Load reference plugin** (BIAS FX 2, Guitar Rig, etc.) on duplicate track
2. **Match settings** as closely as possible
3. **Use track routing** to switch between plugins
4. **Compare**:
   - Frequency response
   - Distortion character  
   - Cabinet simulation quality
   - CPU usage
   - Latency

### Test Scenarios:
- **Clean tones**: Low drive, cabinet simulation focus
- **Crunch tones**: Medium drive, EQ interaction
- **High-gain**: Maximum drive, cabinet type differences
- **Direct comparison**: Use "Direct" cabinet mode vs. reference

## Expected Results

### Working Plugin Should:
- ✅ Load without errors in REAPER
- ✅ Process audio in real-time without dropouts
- ✅ Respond to all parameter changes smoothly
- ✅ Provide 256-sample latency compensation
- ✅ Use reasonable CPU resources
- ✅ Sound professional and musical

### Performance Targets:
- **CPU Usage**: <5% on modern CPU at 256-sample buffer
- **Memory**: <50MB RAM usage
- **Latency**: Exactly 256 samples reported
- **Stability**: No crashes during parameter changes or cabinet switching

## Audio Test Files

For consistent testing, use these types of audio:
- **Clean guitar DI**: Test cabinet simulation transparency
- **Distorted guitar**: Test drive interaction with cabinets
- **Single notes**: Test attack/decay characteristics  
- **Chords**: Test harmonic content and cabinet frequency response
- **Sweep tones**: Test frequency response across spectrum

The plugin should provide professional guitar tone shaping comparable to commercial alternatives while maintaining the O(1) performance guarantees.