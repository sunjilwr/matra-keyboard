#!/usr/bin/env bash
# Matra Keyboard (মাত্রা) - Universal Linux Installer
set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

echo -e "${CYAN}====================================================================${NC}"
echo -e "${GREEN}  ⌨️   Installing Matra (মাত্রা) Keyboard for Linux...${NC}"
echo -e "${CYAN}====================================================================${NC}"

# Check for root / sudo
if [ "$EUID" -ne 0 ]; then
  echo -e "${RED}[ERROR] Please run with sudo: sudo ./install.sh${NC}"
  exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# 1. Install binary
BIN_PATH=""
if [ -f "$SCRIPT_DIR/matra" ]; then
    BIN_PATH="$SCRIPT_DIR/matra"
elif [ -f "$PROJECT_ROOT/target/x86_64-unknown-linux-musl/release/matra" ]; then
    BIN_PATH="$PROJECT_ROOT/target/x86_64-unknown-linux-musl/release/matra"
elif [ -f "$PROJECT_ROOT/target/release/matra" ]; then
    BIN_PATH="$PROJECT_ROOT/target/release/matra"
fi

if [ -n "$BIN_PATH" ]; then
    echo "Installing binary to /usr/local/bin/matra..."
    install -m 755 "$BIN_PATH" /usr/local/bin/matra
else
    echo -e "${RED}[ERROR] Release binary 'matra' not found!${NC}"
    exit 1
fi

# 2. Install desktop launcher
echo "Installing desktop launcher..."
install -d /usr/share/applications
if [ -f "$SCRIPT_DIR/matra.desktop" ]; then
    install -m 644 "$SCRIPT_DIR/matra.desktop" /usr/share/applications/matra.desktop
fi

# 3. Install application icon
echo "Installing application icons..."
install -d /usr/share/icons/hicolor/512x512/apps
if [ -f "$SCRIPT_DIR/matra.png" ]; then
    install -m 644 "$SCRIPT_DIR/matra.png" /usr/share/icons/hicolor/512x512/apps/matra.png
elif [ -f "$PROJECT_ROOT/assets/logo.png" ]; then
    install -m 644 "$PROJECT_ROOT/assets/logo.png" /usr/share/icons/hicolor/512x512/apps/matra.png
fi

# 4. Update desktop & icon caches
if which update-desktop-database >/dev/null 2>&1; then
    update-desktop-database -q /usr/share/applications || true
fi
if which gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache -q -t -f /usr/share/icons/hicolor || true
fi

echo -e "${GREEN}====================================================================${NC}"
echo -e "${GREEN}  ✅ Matra (মাত্রা) installed successfully!${NC}"
echo -e "${CYAN}  Launch it from your Applications menu or type 'matra' in terminal.${NC}"
echo -e "${GREEN}====================================================================${NC}"

