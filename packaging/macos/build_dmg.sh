#!/usr/bin/env bash
# Script to build .app bundle and .dmg installer for macOS
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

APP_NAME="Matra"
VERSION="1.1.0"
DMG_NAME="Matra-v${VERSION}.dmg"
APP_BUNDLE="$PROJECT_ROOT/dist/${APP_NAME}.app"
DMG_DIR="$PROJECT_ROOT/dist/dmg_staging"

echo "Building macOS .app bundle and DMG installer for Matra v${VERSION}..."

rm -rf "$APP_BUNDLE" "$DMG_DIR" "$PROJECT_ROOT/dist/$DMG_NAME"
mkdir -p "$APP_BUNDLE/Contents/MacOS"
mkdir -p "$APP_BUNDLE/Contents/Resources"
mkdir -p "$DMG_DIR"

# 1. Copy binary
cp "$PROJECT_ROOT/target/release/matra" "$APP_BUNDLE/Contents/MacOS/matra"
chmod +x "$APP_BUNDLE/Contents/MacOS/matra"

# 2. Copy Info.plist
cp "$SCRIPT_DIR/Info.plist" "$APP_BUNDLE/Contents/Info.plist"

# 3. Create .icns from logo.png if iconutil/sips is available
if which sips >/dev/null 2>&1 && which iconutil >/dev/null 2>&1; then
    echo "Generating native macOS ICNS icon..."
    ICONSET="$PROJECT_ROOT/dist/matra.iconset"
    mkdir -p "$ICONSET"
    sips -z 16 16     "$PROJECT_ROOT/assets/logo.png" --out "$ICONSET/icon_16x16.png" >/dev/null
    sips -z 32 32     "$PROJECT_ROOT/assets/logo.png" --out "$ICONSET/icon_16x16@2x.png" >/dev/null
    sips -z 32 32     "$PROJECT_ROOT/assets/logo.png" --out "$ICONSET/icon_32x32.png" >/dev/null
    sips -z 64 64     "$PROJECT_ROOT/assets/logo.png" --out "$ICONSET/icon_32x32@2x.png" >/dev/null
    sips -z 128 128   "$PROJECT_ROOT/assets/logo.png" --out "$ICONSET/icon_128x128.png" >/dev/null
    sips -z 256 256   "$PROJECT_ROOT/assets/logo.png" --out "$ICONSET/icon_128x128@2x.png" >/dev/null
    sips -z 256 256   "$PROJECT_ROOT/assets/logo.png" --out "$ICONSET/icon_256x256.png" >/dev/null
    sips -z 512 512   "$PROJECT_ROOT/assets/logo.png" --out "$ICONSET/icon_256x256@2x.png" >/dev/null
    sips -z 512 512   "$PROJECT_ROOT/assets/logo.png" --out "$ICONSET/icon_512x512.png" >/dev/null
    sips -z 1024 1024 "$PROJECT_ROOT/assets/logo.png" --out "$ICONSET/icon_512x512@2x.png" >/dev/null
    iconutil -c icns "$ICONSET" -o "$APP_BUNDLE/Contents/Resources/matra.icns"
    rm -rf "$ICONSET"
fi

# 4. Prepare DMG staging directory with drag-to-Applications symlink
cp -r "$APP_BUNDLE" "$DMG_DIR/"
ln -s /Applications "$DMG_DIR/Applications"

# 5. Build user-friendly Drag-and-Drop DMG
if which hdiutil >/dev/null 2>&1; then
    echo "Creating DMG disk image..."
    hdiutil create -volname "${APP_NAME}" -srcfolder "$DMG_DIR" -ov -format UDZO "$PROJECT_ROOT/dist/$DMG_NAME"
    rm -rf "$DMG_DIR"
    echo "✅ Generated: dist/$DMG_NAME"
else
    echo "hdiutil not available, app bundle prepared at: dist/${APP_NAME}.app"
fi

