#!/bin/bash

# Audio Test Script - Creates test project for BIAS FX Rust
# Generates a simple REAPER project with the plugin loaded

echo "🎵 Creating REAPER test project for BIAS FX Rust..."

# Create test project directory
mkdir -p test_project

# Generate a simple REAPER project file with our plugin
cat > test_project/bias_fx_test.rpp << 'EOF'
<REAPER_PROJECT 0.1 "7.0/linux-x86_64" 1719571200
  RIPPLE 0
  GROUPOVERRIDE 0 0 0
  AUTOXFADE 129
  ENVATTACH 3
  POOLEDENVATTACH 0
  MIXERUIFLAGS 11 48
  PEAKGAIN 1
  FEEDBACK 0
  PANLAW 1
  PROJOFFS 0 0 0
  MAXPROJLEN 0 600
  GRID 3199 8 1 8 1 0 0 0
  TIMEMODE 1 5 -1 30 0 0 -1
  VIDEO_CONFIG 0 0 1
  PANMODE 3
  CURSOR 0
  ZOOM 100 0 0
  VZOOMEX 6 0
  USE_REC_CFG 0
  RECMODE 1
  SMPTESYNC 0 30 100 40 1000 300 0 0 1 0 0
  LOOP 0
  LOOPGRAN 0 4
  RECORD_PATH "" ""
  <RECORD_CFG
    ZXZhdxgAAQ==
  >
  <APPLYFX_CFG
  >
  RENDER_FILE ""
  RENDER_PATTERN ""
  RENDER_FMT 0 2 0
  RENDER_1X 0
  RENDER_RANGE 1 0 0 18 1000
  RENDER_RESAMPLE 3 0 1
  RENDER_ADDTOPROJ 0
  RENDER_STEMS 0
  RENDER_DITHER 0
  TIMELOCKMODE 1
  TEMPOENVLOCKMODE 1
  ITEMMIX 1
  DEFPITCHMODE 589824 0
  TAKELANE 1
  SAMPLERATE 48000 0 0
  <TRACK {12345678-1234-1234-1234-123456789abc}
    NAME "Guitar Track"
    PEAKCOL 16576
    BEAT -1
    AUTOMODE 0
    VOLPAN 1 0 -1 -1 1
    MUTESOLO 0 0 0
    IPHASE 0
    PLAYOFFS 0 1
    ISBUS 0 0
    BUSCOMP 0 0 0 0 0
    SHOWINMIX 1 0.6667 0.5 1 0.5 0 0 0
    FREEMODE 0
    SEL 1
    REC 0 0 1 0 0 0 0 0
    VU 2
    TRACKHEIGHT 0 0 0 0 0 0
    INQ 0 0 0 0.5 100 0 0 100
    NCHAN 2
    FX 1
    TRACKID {12345678-1234-1234-1234-123456789abc}
    PERF 0
    MIDIOUT -1
    MAINSEND 1 0
    <FXCHAIN
      WNDRECT 0 0 0 0
      SHOW 0
      LASTSEL 0
      DOCKED 0
      BYPASS 0 0 0
      <VST "VST3: BIAS FX Rust (Rust Audio)" "BiasFxRust.vst3" 0 "" 1919248245{56535433-4269-6173-4658-527573740000}
        QmlhcyBGWCBSdXN0AAAAAQAAAAEAAAAiAAAAMFg0WTJjNUUwLVM1aEJPc0FLMFQ1TkNvUzVyUTVoQk9zNGRlZnVsdA==
        776265652e3078
        QmlhcyBGWCBSdXN0AAAAAQAAAAEAAAAi
      >
      FLOATPOS 0 0 0 0
      FXID {12345678-1234-1234-1234-123456789def}
      WAK 0 0
    >
  >
EOF

echo "✅ Test project created: test_project/bias_fx_test.rpp"
echo ""
echo "🚀 To test the plugin:"
echo "1. Start REAPER:"
echo "   reaper test_project/bias_fx_test.rpp"
echo ""
echo "2. The project should load with:"
echo "   • One guitar track with BIAS FX Rust already loaded"
echo "   • Plugin should show 8 parameters"
echo ""
echo "3. To test audio:"
echo "   • Connect guitar and arm track for recording"
echo "   • Or import an audio file and drag to track"
echo "   • Adjust plugin parameters to hear processing"
echo ""
echo "4. Test cabinet simulation:"
echo "   • Try different Cabinet types"
echo "   • Adjust Cabinet Mix from 0% to 100%"
echo "   • Set Drive higher to hear cabinet differences"

# Create a simple test tone generator script
cat > test_project/generate_test_tone.sh << 'EOF'
#!/bin/bash
# Generate test tone for plugin testing

echo "🎵 Generating test tone..."

# Create a 5-second 440Hz sine wave at -12dB
sox -n test_tone.wav synth 5 sine 440 vol -12dB

echo "✅ Test tone created: test_tone.wav"
echo "Import this into REAPER to test the plugin!"
EOF

chmod +x test_project/generate_test_tone.sh

echo ""
echo "💡 Extra: Generate test audio with:"
echo "   cd test_project && ./generate_test_tone.sh"
echo "   Then import test_tone.wav into REAPER"