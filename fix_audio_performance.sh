#!/bin/bash

# Audio Performance Fix Script for BIAS FX Rust
# Fixes crackling, slow audio, and performance issues

echo "🔧 BIAS FX Rust - Audio Performance Fix"
echo "======================================="

echo "🔍 Diagnosing audio performance issues..."

# Check current JACK settings
if pgrep jackd > /dev/null; then
    echo "📊 Current JACK settings:"
    if command -v jack_samplerate &> /dev/null; then
        JACK_SR=$(jack_samplerate 2>/dev/null)
        echo "   Sample rate: ${JACK_SR}Hz"
    fi
    
    if command -v jack_bufsize &> /dev/null; then
        JACK_BUF=$(jack_bufsize 2>/dev/null)
        echo "   Buffer size: ${JACK_BUF} samples"
        
        # Calculate latency
        if [ ! -z "$JACK_SR" ] && [ ! -z "$JACK_BUF" ]; then
            LATENCY=$(echo "scale=1; $JACK_BUF * 1000 / $JACK_SR" | bc -l 2>/dev/null || echo "unknown")
            echo "   Latency: ${LATENCY}ms"
            
            if (( $(echo "$JACK_BUF > 512" | bc -l) )); then
                echo "   ⚠️  Buffer size is high - may cause slow/sluggish audio"
            fi
        fi
    fi
    
    # Check JACK load
    if command -v jack_cpu_load &> /dev/null; then
        JACK_LOAD=$(jack_cpu_load 2>/dev/null)
        echo "   CPU load: ${JACK_LOAD}%"
        
        if (( $(echo "$JACK_LOAD > 50" | bc -l 2>/dev/null || echo 0) )); then
            echo "   ⚠️  High CPU load detected"
        fi
    fi
else
    echo "❌ JACK not running - this could cause performance issues"
fi

echo ""
echo "💻 System performance check:"

# Check CPU frequency scaling
if [ -f /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor ]; then
    GOV=$(cat /sys/devices/system/cpu/cpu0/cpufreq/scaling_governor)
    echo "   CPU governor: $GOV"
    if [ "$GOV" != "performance" ]; then
        echo "   ⚠️  CPU not in performance mode - may cause audio issues"
    fi
fi

# Check real-time priorities
if command -v chrt &> /dev/null; then
    REAPER_PID=$(pgrep reaper)
    if [ ! -z "$REAPER_PID" ]; then
        REAPER_PRIO=$(chrt -p $REAPER_PID 2>/dev/null | grep "priority" | awk '{print $4}' || echo "0")
        echo "   REAPER priority: $REAPER_PRIO"
        if [ "$REAPER_PRIO" -eq 0 ]; then
            echo "   ℹ️  REAPER not running with real-time priority"
        fi
    fi
fi

echo ""
echo "🔧 RECOMMENDED FIXES:"
echo "===================="

echo ""
echo "1️⃣ Fix JACK buffer size (for crackling/slow audio):"
echo "   Current buffer: ${JACK_BUF:-unknown} samples"
echo ""
echo "   For low latency:"
echo "   jack_bufsize 128    # 2.7ms latency at 48kHz"
echo "   jack_bufsize 256    # 5.3ms latency at 48kHz"
echo ""
echo "   For stability (if crackling persists):"
echo "   jack_bufsize 512    # 10.7ms latency at 48kHz"

echo ""
echo "2️⃣ Restart JACK with optimal settings:"
echo "   killall jackd"
echo "   jackd -d alsa -d hw:UMC1820 -r 48000 -p 256 -n 2"
echo ""
echo "   Or use qjackctl GUI:"
echo "   qjackctl"
echo "   Setup → Buffer Size: 256"
echo "   Setup → Sample Rate: 48000"

echo ""
echo "3️⃣ Set CPU to performance mode:"
echo "   sudo cpupower frequency-set -g performance"
echo "   # This prevents CPU throttling during audio processing"

echo ""
echo "4️⃣ In REAPER - reduce plugin latency:"
echo "   Options → Preferences → Audio → Device"
echo "   • Request buffer size: 256 (match JACK)"
echo "   • Thread priority: Highest"
echo "   • Allow live FX multi-processing: ON"

echo ""
echo "5️⃣ Plugin-specific fixes:"
echo "   • The plugin has 256-sample internal latency"
echo "   • This should match your JACK buffer size"
echo "   • If JACK buffer = 1024, plugin processes 4x slower!"

echo ""
echo "6️⃣ Test cabinet differences properly:"
echo "   • Set Drive to 8.0+ (higher drive shows cabinet differences)"
echo "   • Set Cabinet Mix to 100%"
echo "   • Compare: Marshall V30 vs Direct (should be very different)"
echo "   • Marshall = aggressive mids, Direct = no cabinet coloration"

echo ""
echo "🚀 QUICK FIX COMMANDS:"
echo "====================="

echo ""
echo "# Stop current JACK"
echo "killall jackd"
echo ""
echo "# Start JACK with optimal settings for your UMC1820"
echo "jackd -d alsa -d hw:UMC1820 -r 48000 -p 256 -n 2 &"
echo ""
echo "# Set CPU performance mode"
echo "sudo cpupower frequency-set -g performance"
echo ""
echo "# Then restart REAPER and test"

echo ""
echo "🎯 Expected results after fixes:"
echo "• No crackling or dropouts"
echo "• Responsive, not slow/sluggish"
echo "• Clear difference between Marshall V30 and Direct cabinet"
echo "• Drive parameter creates musical distortion"
echo "• Cabinet Mix 0% vs 100% sounds very different"

echo ""
echo "📞 Test command after fixes:"
echo "jack_simple_client    # Should play smooth sine wave"