#!/bin/bash

# BIAS FX Rust - Installation Test Script
# Verifies plugin build and installation for REAPER testing

echo "🔧 BIAS FX Rust Installation Test"
echo "=================================="

# Check if plugin was built
echo "📦 Checking build artifacts..."
if [ -f "target/release/libbias_fx_rust.so" ]; then
    echo "✅ Plugin library built successfully"
    ls -lh target/release/libbias_fx_rust.so
else
    echo "❌ Plugin library not found - run ./build_plugin.sh first"
    exit 1
fi

if [ -d "target/release/BiasFxRust.vst3" ]; then
    echo "✅ VST3 bundle created successfully"
    echo "📁 Bundle contents:"
    find target/release/BiasFxRust.vst3 -type f -exec ls -lh {} \;
else
    echo "❌ VST3 bundle not found - run ./build_plugin.sh first"
    exit 1
fi

# Check VST3 directory
echo ""
echo "📂 Checking VST3 installation directories..."

USER_VST3="$HOME/.vst3"
SYSTEM_VST3="/usr/lib/vst3"

if [ -d "$USER_VST3" ]; then
    echo "✅ User VST3 directory exists: $USER_VST3"
else
    echo "ℹ️  Creating user VST3 directory: $USER_VST3"
    mkdir -p "$USER_VST3"
fi

if [ -d "$SYSTEM_VST3" ]; then
    echo "✅ System VST3 directory exists: $SYSTEM_VST3"
else
    echo "⚠️  System VST3 directory not found: $SYSTEM_VST3"
fi

# Check if plugin is already installed
if [ -d "$USER_VST3/BiasFxRust.vst3" ]; then
    echo "✅ Plugin already installed in user directory"
    echo "📅 Installation date: $(stat -c %y "$USER_VST3/BiasFxRust.vst3")"
else
    echo "ℹ️  Plugin not yet installed in user directory"
fi

# Check REAPER
echo ""
echo "🎵 Checking REAPER..."
if command -v reaper &> /dev/null; then
    echo "✅ REAPER found in PATH"
    REAPER_VERSION=$(reaper -version 2>/dev/null || echo "Unknown")
    echo "📊 Version: $REAPER_VERSION"
else
    echo "⚠️  REAPER not found in PATH"
    echo "   Install REAPER from: https://www.reaper.fm/download.php"
fi

# Check audio system
echo ""
echo "🔊 Checking audio system..."
if command -v jackd &> /dev/null; then
    echo "✅ JACK Audio found"
    if pgrep jackd > /dev/null; then
        echo "✅ JACK is running"
    else
        echo "ℹ️  JACK is available but not running"
    fi
else
    echo "ℹ️  JACK not found - using ALSA/PulseAudio"
fi

# Check audio group membership
if groups $USER | grep -q audio; then
    echo "✅ User is in audio group"
else
    echo "⚠️  User not in audio group - may need to add:"
    echo "   sudo usermod -a -G audio $USER"
    echo "   Then logout/login"
fi

# Installation options
echo ""
echo "🚀 Installation Commands:"
echo "========================="
echo ""
echo "User installation (recommended for testing):"
echo "cp -r target/release/BiasFxRust.vst3 ~/.vst3/"
echo ""
echo "System-wide installation:"
echo "sudo cp -r target/release/BiasFxRust.vst3 /usr/lib/vst3/"
echo ""
echo "After installation:"
echo "1. Start REAPER"
echo "2. Go to Options → Preferences → Plugins → VST"
echo "3. Click 'Re-scan' or 'Clear cache and re-scan'"
echo "4. Look for 'BIAS FX Rust' in plugin list"
echo ""
echo "📖 Full testing guide available in TESTING.md"