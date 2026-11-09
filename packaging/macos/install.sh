#!/usr/bin/env bash
# Matra Keyboard (মাত্রা) - One-Click macOS Installer
set -e

GREEN='\033[0;32m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${CYAN}====================================================================${NC}"
echo -e "${GREEN}  ⌨️   Installing Matra (মাত্রা) Keyboard for macOS...${NC}"
echo -e "${CYAN}====================================================================${NC}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
APP_DIR="/Applications/Matra.app"

# Check if Matra.app already exists in the package
if [ -d "$SCRIPT_DIR/Matra.app" ]; then
    echo "Installing Matra.app to /Applications..."
    rm -rf "$APP_DIR"
    cp -R "$SCRIPT_DIR/Matra.app" /Applications/
elif [ -d "$SCRIPT_DIR/../../dist/Matra.app" ]; then
    echo "Installing Matra.app to /Applications..."
    rm -rf "$APP_DIR"
    cp -R "$SCRIPT_DIR/../../dist/Matra.app" /Applications/
else
    echo "Building Matra from source with Rust..."
    if which cargo >/dev/null 2>&1; then
        cargo build --release
        mkdir -p "$SCRIPT_DIR/Matra.app/Contents/MacOS"
        mkdir -p "$SCRIPT_DIR/Matra.app/Contents/Resources"
        cp "target/release/matra" "$SCRIPT_DIR/Matra.app/Contents/MacOS/matra"
        chmod +x "$SCRIPT_DIR/Matra.app/Contents/MacOS/matra"
        cp "$SCRIPT_DIR/Info.plist" "$SCRIPT_DIR/Matra.app/Contents/Info.plist" 2>/dev/null || true
        cp "$SCRIPT_DIR/../../assets/logo.png" "$SCRIPT_DIR/Matra.app/Contents/Resources/matra.png" 2>/dev/null || true
        rm -rf "$APP_DIR"
        cp -R "$SCRIPT_DIR/Matra.app" /Applications/
    else
        echo -e "${YELLOW}[!] Note: Rust/cargo is required to compile on macOS.${NC}"
        echo -e "    Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
fi

# Ensure executable permissions
if [ -f "$APP_DIR/Contents/MacOS/matra" ]; then
    chmod +x "$APP_DIR/Contents/MacOS/matra"
fi

# Create symlink in /usr/local/bin if directory exists and is writable
if [ -d "/usr/local/bin" ] && [ -w "/usr/local/bin" ]; then
    ln -sf "$APP_DIR/Contents/MacOS/matra" /usr/local/bin/matra 2>/dev/null || true
fi

echo -e "${GREEN}====================================================================${NC}"
echo -e "${GREEN}  ✅ Matra.app successfully installed to /Applications!${NC}"
echo -e "${CYAN}  Launch it from Spotlight, Launchpad, or run 'open /Applications/Matra.app'${NC}"
echo -e "${GREEN}====================================================================${NC}"
