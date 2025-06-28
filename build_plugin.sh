#!/bin/bash

# BIAS FX Rust - Professional Guitar VST Plugin Builder
# Achieving O(1) performance with functional programming principles

echo "🎸 Building BIAS FX Rust - Professional Guitar VST Plugin"
echo "🦀 Using functional programming for O(1) real-time performance"

# Build optimized release version
echo "⚡ Building optimized release..."
cargo build --lib --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

# Create VST3 bundle structure
PLUGIN_NAME="BiasFxRust"
VST3_DIR="target/release/${PLUGIN_NAME}.vst3"
CONTENTS_DIR="${VST3_DIR}/Contents"

echo "📦 Creating VST3 bundle structure..."
mkdir -p "${CONTENTS_DIR}/x86_64-linux"

# Copy plugin binary
cp "target/release/libbias_fx_rust.so" "${CONTENTS_DIR}/x86_64-linux/${PLUGIN_NAME}.so"

# Create Info.plist
cat > "${CONTENTS_DIR}/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>${PLUGIN_NAME}</string>
    <key>CFBundleIconFile</key>
    <string></string>
    <key>CFBundleIdentifier</key>
    <string>com.rust-audio.bias-fx-rust</string>
    <key>CFBundleName</key>
    <string>BIAS FX Rust</string>
    <key>CFBundleDisplayName</key>
    <string>BIAS FX Rust</string>
    <key>CFBundlePackageType</key>
    <string>BNDL</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
</dict>
</plist>
EOF

echo "✅ VST3 plugin built successfully!"
echo "📁 Plugin location: ${VST3_DIR}"
echo ""
echo "🔧 Installation instructions:"
echo "   For system-wide installation:"
echo "   sudo cp -r \"${VST3_DIR}\" /usr/lib/vst3/"
echo ""
echo "   For user installation:"
echo "   mkdir -p ~/.vst3"
echo "   cp -r \"${VST3_DIR}\" ~/.vst3/"
echo ""
echo "🎸 Ready to use in REAPER and other Linux DAWs!"
echo "⚡ Features O(1) functional DSP processing for professional performance"