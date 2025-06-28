# Local Testing Guide for BIAS FX Rust Plugin

## Quick Start Testing (5 minutes)

### 1. Test Plugin Loading
```bash
# First, verify plugin is built and installed
ls -la ~/.vst3/BiasFxRust.vst3/Contents/x86_64-linux/BiasFxRust.so

# Start REAPER
reaper
```

### 2. Basic Plugin Test
1. **Create new project**: `Ctrl+N`
2. **Create audio track**: `Ctrl+T` 
3. **Add plugin**: Click "FX" → Browse → "VST3" → "BIAS FX Rust"
4. **Verify parameters**: Should see 8 controls:
   - Input Gain, Drive, Bass, Mid, Treble, Output Gain, Cabinet, Cabinet Mix

### 3. Quick Audio Test
**Option A: With Guitar Input**
1. Connect guitar to audio interface
2. Set track input to your interface
3. Click red record button (arm track)
4. Click monitor button (speaker icon)
5. Play guitar - should hear processed sound

**Option B: With Test Audio File**
1. Import any audio file: `Ctrl+Alt+I`
2. Drag to timeline
3. Add BIAS FX Rust to that track
4. Press spacebar to play

## Detailed Testing Procedures

### Parameter Testing

#### Input/Output Gain Testing
```
1. Set Input Gain to -30dB → Should be very quiet
2. Set Input Gain to +30dB → Should be much louder  
3. Repeat for Output Gain
4. Test automation: Right-click → "Show in track automation lane"
```

#### Distortion Testing
```
1. Set Drive to 1.0 → Clean sound
2. Gradually increase Drive to 20.0 → Progressive distortion
3. Test with different input levels
4. Combine with different cabinet types
```

#### EQ Testing
```
1. Bass: -12dB to +12dB (affects ~100Hz)
2. Mid: -12dB to +12dB (affects ~500Hz)  
3. Treble: -12dB to +12dB (affects ~3kHz)
4. Test extreme settings to hear differences
```

#### Cabinet Simulation Testing
```
1. Set Cabinet Mix to 0% → Dry signal (no cabinet)
2. Set Cabinet Mix to 100% → Full cabinet simulation
3. Test each cabinet type:
   - Marshall 4x12 V30: Aggressive, midrange-heavy
   - Fender Twin 2x12: Scooped, clean American sound
   - Vox AC30 Blue: Vintage British, midrange-focused
   - Mesa 4x12 Recto: Modern, tight, high-gain
   - Direct: Bypasses cabinet (should sound like 0% mix)
```

## Performance Testing

### CPU Usage Monitoring
```bash
# In terminal, monitor REAPER CPU usage
htop
# Look for 'reaper' process - should be low CPU even with plugin active
```

### Latency Testing
1. **Check reported latency**: REAPER should show 256 samples plugin delay
2. **Buffer size testing**:
   - 64 samples: Very responsive, higher CPU
   - 128 samples: Good compromise
   - 256 samples: Plugin's native size, optimal
   - 512-1024 samples: Higher latency, lower CPU

### Memory Testing
```bash
# Monitor memory usage
ps aux | grep reaper
# VSS/RSS should be reasonable (<500MB total)
```

## Audio Quality Testing

### Test Signals
Create these in REAPER for systematic testing:

#### 1. Sine Wave Test
```
1. Insert → Media Item → Empty Item
2. Item → Take → Generate → Sine Wave
3. Frequency: 440Hz, 1kHz, 5kHz
4. Test how plugin affects pure tones
```

#### 2. White Noise Test
```
1. Generate white noise
2. Test EQ response - should hear clear filtering
3. Test cabinet simulation frequency shaping
```

#### 3. Guitar DI Test
```
1. Record clean guitar DI (direct input)
2. Test plugin on recorded signal
3. A/B test: bypassed vs. processed
```

### Listening Tests

#### Clean Tones (Low Drive)
- **Target**: Transparent with subtle cabinet coloration
- **Test**: Single notes, chords, fingerpicking
- **Cabinet comparison**: Fender Twin vs. Vox AC30

#### Driven Tones (High Drive)
- **Target**: Musical distortion, not harsh
- **Test**: Power chords, single note runs
- **Cabinet comparison**: Marshall V30 vs. Mesa Recto

#### Extreme Settings
- **Max Drive + EQ boosts**: Should remain musical
- **Cabinet Mix transitions**: No clicks/pops when changing

## Advanced Testing

### Automation Testing
```
1. Create automation lanes for all parameters
2. Draw smooth curves and sharp changes
3. Play project - should be no audio artifacts
4. Test parameter changes during playback
```

### Multiple Instance Testing
```
1. Load plugin on multiple tracks simultaneously
2. Monitor CPU usage scaling
3. Test different cabinet types on different tracks
```

### Project Save/Load Testing
```
1. Set up plugin with specific settings
2. Save project
3. Close REAPER
4. Reopen project
5. Verify all settings restored correctly
```

### Host Compatibility
```
1. Test plugin delay compensation in REAPER
2. Check track timing with other plugins
3. Test in different REAPER project templates
```

## Troubleshooting Tests

### Plugin Loading Issues
```bash
# Check plugin dependencies
ldd ~/.vst3/BiasFxRust.vst3/Contents/x86_64-linux/BiasFxRust.so

# Check for missing libraries
# All should show "found" - no "not found" entries
```

### Audio System Issues  
```bash
# Test basic audio
speaker-test -c 2

# Check JACK status (if using JACK)
jack_control status

# Check audio group membership
groups | grep audio
```

### Plugin Crash Testing
```
1. Load plugin
2. Rapidly change all parameters
3. Change cabinet types quickly
4. Automate multiple parameters simultaneously
5. Should remain stable throughout
```

## Expected Results

### ✅ Working Plugin Should:
- Load instantly in REAPER without errors
- Show 8 parameters as described
- Process audio in real-time without dropouts
- Report 256 samples latency to host
- Use <5% CPU on modern systems
- Provide musical, professional sound quality
- Handle parameter automation smoothly
- Switch cabinet types without clicks/pops

### ❌ Signs of Problems:
- Plugin doesn't appear in REAPER scan
- Missing parameters (less than 8)
- Audio dropouts or crackling
- High CPU usage (>20%)
- Clicks when changing parameters
- Distorted/harsh sound quality
- REAPER crashes when loading plugin

## Quick Test Checklist

```
□ Plugin builds without errors
□ Plugin installs to ~/.vst3/
□ REAPER finds plugin after rescan
□ Plugin loads on track without errors
□ All 8 parameters visible and responsive
□ Audio processes in real-time
□ Cabinet types sound different from each other
□ No audio artifacts during parameter changes
□ CPU usage reasonable (<10%)
□ Plugin reports 256 samples latency
□ Save/load project preserves settings
```

## Test Audio Files

For consistent testing, use:
- **Clean guitar DI recording** (test cabinet simulation)
- **Drum loop** (test how plugin affects transients) 
- **Vocal** (test musical processing on non-guitar sources)
- **White noise** (test EQ frequency response)
- **Sine sweeps** (test for artifacts across frequency range)

The plugin should provide professional guitar tone shaping that sounds musical and responds naturally to input dynamics and playing techniques.