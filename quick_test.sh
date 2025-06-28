#!/bin/bash

# Quick Test Script for BIAS FX Rust Plugin
# Performs basic verification tests before REAPER testing

echo "🔧 BIAS FX Rust - Quick Test Suite"
echo "==================================="

# Test 1: Build verification
echo "1️⃣ Testing build artifacts..."
if [ -f "target/release/libbias_fx_rust.so" ]; then
    echo "✅ Plugin library exists"
    FILE_SIZE=$(du -h target/release/libbias_fx_rust.so | cut -f1)
    echo "   Size: $FILE_SIZE (should be ~2MB+ with cabinet simulation)"
else
    echo "❌ Plugin library missing - run: cargo build --lib --release"
    exit 1
fi

# Test 2: VST3 bundle verification  
echo ""
echo "2️⃣ Testing VST3 bundle..."
if [ -d "target/release/BiasFxRust.vst3" ]; then
    echo "✅ VST3 bundle exists"
    if [ -f "target/release/BiasFxRust.vst3/Contents/x86_64-linux/BiasFxRust.so" ]; then
        echo "✅ Plugin binary in bundle"
    else
        echo "❌ Plugin binary missing from bundle"
        exit 1
    fi
else
    echo "❌ VST3 bundle missing - run: ./build_plugin.sh"
    exit 1
fi

# Test 3: Installation verification
echo ""
echo "3️⃣ Testing installation..."
if [ -d "$HOME/.vst3/BiasFxRust.vst3" ]; then
    echo "✅ Plugin installed in user directory"
    INSTALLED_SIZE=$(du -h "$HOME/.vst3/BiasFxRust.vst3/Contents/x86_64-linux/BiasFxRust.so" | cut -f1)
    echo "   Installed size: $INSTALLED_SIZE"
    
    # Check if installed version is up to date
    SOURCE_TIME=$(stat -c %Y "target/release/libbias_fx_rust.so")
    INSTALLED_TIME=$(stat -c %Y "$HOME/.vst3/BiasFxRust.vst3/Contents/x86_64-linux/BiasFxRust.so")
    
    if [ $SOURCE_TIME -gt $INSTALLED_TIME ]; then
        echo "⚠️  Installed version is older than built version"
        echo "   Run: cp -r target/release/BiasFxRust.vst3 ~/.vst3/"
    else
        echo "✅ Installed version is current"
    fi
else
    echo "ℹ️  Plugin not installed yet"
    echo "   Run: cp -r target/release/BiasFxRust.vst3 ~/.vst3/"
fi

# Test 4: Dependencies check
echo ""
echo "4️⃣ Testing dependencies..."
PLUGIN_PATH="$HOME/.vst3/BiasFxRust.vst3/Contents/x86_64-linux/BiasFxRust.so"
if [ -f "$PLUGIN_PATH" ]; then
    echo "   Checking shared library dependencies..."
    MISSING_DEPS=$(ldd "$PLUGIN_PATH" 2>/dev/null | grep "not found")
    if [ -z "$MISSING_DEPS" ]; then
        echo "✅ All dependencies satisfied"
    else
        echo "❌ Missing dependencies:"
        echo "$MISSING_DEPS"
    fi
else
    echo "⚠️  Cannot check dependencies - plugin not installed"
fi

# Test 5: REAPER detection
echo ""
echo "5️⃣ Testing REAPER environment..."
if command -v reaper &> /dev/null; then
    echo "✅ REAPER found in PATH"
    
    # Check for REAPER config directory
    if [ -d "$HOME/.config/REAPER" ]; then
        echo "✅ REAPER config directory exists"
        
        # Check plugin cache files
        if [ -f "$HOME/.config/REAPER/reaper-vst3plugins64.ini" ]; then
            echo "ℹ️  VST3 plugin cache exists"
            if grep -q "BiasFxRust" "$HOME/.config/REAPER/reaper-vst3plugins64.ini" 2>/dev/null; then
                echo "✅ Plugin found in REAPER cache"
            else
                echo "ℹ️  Plugin not yet in REAPER cache - rescan needed"
            fi
        else
            echo "ℹ️  No VST3 plugin cache - first scan will create it"
        fi
    else
        echo "ℹ️  REAPER config directory will be created on first run"
    fi
else
    echo "⚠️  REAPER not found in PATH"
    echo "   Install REAPER from: https://www.reaper.fm/download.php"
fi

# Test 6: Audio system check
echo ""
echo "6️⃣ Testing audio system..."
if groups $USER | grep -q audio; then
    echo "✅ User is in audio group"
else
    echo "⚠️  User not in audio group - may cause audio issues"
    echo "   Fix: sudo usermod -a -G audio $USER (then logout/login)"
fi

if command -v jackd &> /dev/null; then
    echo "✅ JACK Audio available"
    if pgrep jackd > /dev/null; then
        echo "✅ JACK is running"
        # Check JACK sample rate and buffer size
        if command -v jack_samplerate &> /dev/null; then
            JACK_SR=$(jack_samplerate 2>/dev/null || echo "unknown")
            echo "   Sample rate: ${JACK_SR}Hz"
        fi
        if command -v jack_bufsize &> /dev/null; then
            JACK_BUF=$(jack_bufsize 2>/dev/null || echo "unknown")
            echo "   Buffer size: ${JACK_BUF} samples"
        fi
    else
        echo "ℹ️  JACK available but not running"
    fi
else
    echo "ℹ️  JACK not found - will use ALSA/PulseAudio"
fi

# Test Summary
echo ""
echo "📋 Test Summary"
echo "==============="
echo ""
echo "Next steps for REAPER testing:"
echo "1. Start REAPER: reaper"
echo "2. Go to Options → Preferences → Plugins → VST"
echo "3. Click 'Clear cache and re-scan'"
echo "4. Create new track (Ctrl+T)"
echo "5. Add FX → VST3 → BIAS FX Rust"
echo ""
echo "Expected parameters:"
echo "• Input Gain (-30dB to +30dB)"
echo "• Drive (1.0 to 20.0)"
echo "• Bass (-12dB to +12dB)"
echo "• Mid (-12dB to +12dB)" 
echo "• Treble (-12dB to +12dB)"
echo "• Cabinet (Marshall/Fender/Vox/Mesa/Direct)"
echo "• Cabinet Mix (0% to 100%)"
echo "• Output Gain (-30dB to +30dB)"
echo ""
echo "🎸 Ready for testing! See LOCAL_TESTING.md for detailed procedures."