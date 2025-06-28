#!/bin/bash

# Audio Troubleshooting Script for BIAS FX Rust Plugin
# Diagnoses and fixes common audio issues in REAPER

echo "🔧 BIAS FX Rust - Audio Troubleshooting"
echo "======================================="

echo "1️⃣ Checking system audio..."

# Check if audio is working at system level
echo "Testing system audio output..."
if command -v speaker-test &> /dev/null; then
    echo "Running speaker test (2 seconds)..."
    timeout 2s speaker-test -t sine -f 440 -c 2 >/dev/null 2>&1
    if [ $? -eq 0 ] || [ $? -eq 124 ]; then  # 124 is timeout exit code
        echo "✅ System audio working"
    else
        echo "❌ System audio issue detected"
        echo "Try: sudo apt install alsa-utils"
    fi
else
    echo "⚠️  speaker-test not available"
fi

echo ""
echo "2️⃣ Checking JACK audio system..."

if command -v jackd &> /dev/null; then
    if pgrep jackd > /dev/null; then
        echo "✅ JACK is running"
        
        # Get JACK info
        if command -v jack_samplerate &> /dev/null; then
            JACK_SR=$(jack_samplerate 2>/dev/null)
            echo "   Sample rate: ${JACK_SR}Hz"
        fi
        
        if command -v jack_bufsize &> /dev/null; then
            JACK_BUF=$(jack_bufsize 2>/dev/null)
            echo "   Buffer size: ${JACK_BUF} samples"
        fi
        
        # Check JACK connections
        if command -v jack_lsp &> /dev/null; then
            echo "   JACK ports:"
            jack_lsp | head -10
        fi
        
    else
        echo "❌ JACK not running"
        echo ""
        echo "Options to fix:"
        echo "A) Start JACK manually:"
        echo "   qjackctl  # GUI control"
        echo "   # or"
        echo "   jackd -d alsa -d hw:0 -r 48000 -p 256"
        echo ""
        echo "B) Configure REAPER for ALSA instead:"
        echo "   REAPER → Options → Preferences → Audio → Device"
        echo "   Select 'ALSA' instead of JACK"
    fi
else
    echo "ℹ️  JACK not installed - using ALSA/PulseAudio"
fi

echo ""
echo "3️⃣ Checking REAPER configuration..."

REAPER_CONFIG="$HOME/.config/REAPER"
if [ -d "$REAPER_CONFIG" ]; then
    echo "✅ REAPER config directory exists"
    
    # Check for REAPER audio config
    if [ -f "$REAPER_CONFIG/reaper.ini" ]; then
        echo "✅ REAPER config file exists"
        
        # Check audio device settings
        AUDIO_DEV=$(grep "audio_dev=" "$REAPER_CONFIG/reaper.ini" 2>/dev/null || echo "not found")
        echo "   Audio device: $AUDIO_DEV"
        
        # Check for master volume automation
        MASTER_AUTO=$(grep -i "master.*auto" "$REAPER_CONFIG/reaper.ini" 2>/dev/null || echo "none found")
        if [ "$MASTER_AUTO" != "none found" ]; then
            echo "⚠️  Master automation detected: $MASTER_AUTO"
        fi
        
    else
        echo "ℹ️  REAPER config will be created on first proper run"
    fi
else
    echo "ℹ️  REAPER not configured yet"
fi

echo ""
echo "4️⃣ Recommended REAPER audio setup..."

echo ""
echo "🔧 REAPER Audio Configuration Steps:"
echo "====================================="
echo ""
echo "1. Start REAPER"
echo "2. Go to: Options → Preferences → Audio → Device"
echo ""
echo "3. Choose audio system:"
if pgrep jackd > /dev/null; then
    echo "   ✅ JACK is running - select 'JACK'"
    echo "   • Input: Use JACK input ports"
    echo "   • Output: Use JACK output ports"
    echo "   • Buffer: Will use JACK settings ($(jack_bufsize 2>/dev/null || echo 'unknown') samples)"
else
    echo "   🔄 JACK not running - select 'ALSA'"
    echo "   • Input: Select your audio interface"
    echo "   • Output: Select your speakers/headphones"
    echo "   • Buffer: Start with 512 samples, reduce if needed"
fi

echo ""
echo "4. Test audio in REAPER:"
echo "   • Options → Preferences → Audio → Test audio"
echo "   • Should hear test tones from both speakers"

echo ""
echo "5. Fix 'master automuted' issue:"
echo "   • In main REAPER window, look at master volume fader"
echo "   • Click the 'AUTO' button if it's lit up (to disable automation)"
echo "   • Or: Track → Master track → Show master track"
echo "   • Delete any automation on master volume"

echo ""
echo "6. Create simple test:"
echo "   • Insert → New track"
echo "   • Track → Input: Select your audio input"
echo "   • Click red record button (arm track)"
echo "   • Click monitor button (speaker icon)"
echo "   • Should hear input signal"

echo ""
echo "🎸 Plugin-specific testing:"
echo "=========================="
echo ""
echo "1. Add BIAS FX Rust to the test track:"
echo "   • Click FX button on track"
echo "   • VST3 → BIAS FX Rust"
echo ""
echo "2. Verify plugin loads with 8 parameters:"
echo "   • Input Gain, Drive, Bass, Mid, Treble"
echo "   • Output Gain, Cabinet, Cabinet Mix"
echo ""
echo "3. Test processing:"
echo "   • Set Drive to 5.0+ to hear distortion"
echo "   • Change Cabinet type to hear differences"
echo "   • Adjust Cabinet Mix 0% → 100%"

echo ""
echo "🚨 Common Issues & Fixes:"
echo "========================"
echo ""
echo "• No sound at all:"
echo "  → Check REAPER audio device settings"
echo "  → Verify system audio works"
echo "  → Check master volume not muted/automated"
echo ""
echo "• 'Master automuted':"
echo "  → Click AUTO button on master fader to disable"
echo "  → Or delete master volume automation"
echo ""
echo "• Plugin loads but no processing:"
echo "  → Check Input Gain not at minimum"
echo "  → Check Output Gain not at minimum" 
echo "  → Verify Cabinet Mix > 0% for cabinet simulation"
echo ""
echo "• Crackling/dropouts:"
echo "  → Increase audio buffer size"
echo "  → Close other audio applications"
echo "  → Check CPU usage"

echo ""
echo "📞 Quick audio test command:"
echo "speaker-test -t sine -f 440 -c 2 -l 3"
echo "(Should hear 440Hz tone for 3 loops)"