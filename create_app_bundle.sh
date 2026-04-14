#!/bin/bash
# create_app_bundle.sh — Package CopyCan as a macOS .app bundle
#
# Usage: ./create_app_bundle.sh
#
# This creates CopyCan.app in the project root that can be:
#   - Dragged to /Applications
#   - Double-clicked to launch
#   - Set as a Login Item via System Settings

set -euo pipefail

APP_NAME="CopyCan"
BUNDLE_DIR="${APP_NAME}.app"
CONTENTS_DIR="${BUNDLE_DIR}/Contents"
MACOS_DIR="${CONTENTS_DIR}/MacOS"
RESOURCES_DIR="${CONTENTS_DIR}/Resources"

echo "Building release binary..."
swift build -c release --arch arm64 --arch x86_64

echo "Creating app bundle: ${BUNDLE_DIR}"
rm -rf "${BUNDLE_DIR}"
mkdir -p "${MACOS_DIR}"
mkdir -p "${RESOURCES_DIR}"

# Copy binary
cp ".build/apple/Products/Release/CopyCanSwift" "${MACOS_DIR}/${APP_NAME}"

# Copy icon
if [ -f "assets/CopyCan.icns" ]; then
    cp "assets/CopyCan.icns" "${RESOURCES_DIR}/CopyCan.icns"
    echo "Icon copied to bundle"
fi

# Create Info.plist
cat > "${CONTENTS_DIR}/Info.plist" << 'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>CopyCan</string>
    <key>CFBundleDisplayName</key>
    <string>CopyCan</string>
    <key>CFBundleIdentifier</key>
    <string>com.copycan.app</string>
    <key>CFBundleVersion</key>
    <string>0.1.0</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
    <key>CFBundleExecutable</key>
    <string>CopyCan</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>CFBundleIconFile</key>
    <string>CopyCan</string>
    <key>LSUIElement</key>
    <true/>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
PLIST

echo ""
echo "✅ ${BUNDLE_DIR} created successfully!"
echo ""
echo "Binary size: $(du -sh "${MACOS_DIR}/${APP_NAME}" | cut -f1)"
echo ""
echo "To install:"
echo "  cp -r ${BUNDLE_DIR} /Applications/"
echo ""
echo "To run:"
echo "  open ${BUNDLE_DIR}"
echo "  # or: open /Applications/${BUNDLE_DIR}"
