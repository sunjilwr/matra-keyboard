#!/usr/bin/env bash
# Script to build .deb installer package for Ubuntu/Debian/Mint
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

PKG_NAME="matra"
PKG_VERSION="1.0.0"
ARCH="amd64"
DEB_DIR="$PROJECT_ROOT/dist/${PKG_NAME}_${PKG_VERSION}_${ARCH}"

echo "Building Debian package for Matra v${PKG_VERSION}..."

rm -rf "$DEB_DIR"
mkdir -p "$DEB_DIR/DEBIAN"
mkdir -p "$DEB_DIR/usr/bin"
mkdir -p "$DEB_DIR/usr/share/applications"
mkdir -p "$DEB_DIR/usr/share/icons/hicolor/512x512/apps"
mkdir -p "$DEB_DIR/usr/share/doc/$PKG_NAME"

# Copy binary
cp "$PROJECT_ROOT/target/release/matra" "$DEB_DIR/usr/bin/matra"
chmod 755 "$DEB_DIR/usr/bin/matra"

# Copy desktop entry & icon
cp "$SCRIPT_DIR/matra.desktop" "$DEB_DIR/usr/share/applications/matra.desktop"
cp "$PROJECT_ROOT/assets/logo.png" "$DEB_DIR/usr/share/icons/hicolor/512x512/apps/matra.png"
cp "$PROJECT_ROOT/README.md" "$DEB_DIR/usr/share/doc/$PKG_NAME/README"

# Copy control & postinst
cp "$SCRIPT_DIR/control" "$DEB_DIR/DEBIAN/control"
cp "$SCRIPT_DIR/postinst" "$DEB_DIR/DEBIAN/postinst"
chmod 755 "$DEB_DIR/DEBIAN/postinst"

# Build debian archive
mkdir -p "$PROJECT_ROOT/dist"
dpkg-deb --build "$DEB_DIR" "$PROJECT_ROOT/dist/${PKG_NAME}_${PKG_VERSION}_${ARCH}.deb"
rm -rf "$DEB_DIR"

echo "✅ Generated: dist/${PKG_NAME}_${PKG_VERSION}_${ARCH}.deb"

