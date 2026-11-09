#!/usr/bin/env bash
# Matra Keyboard (মাত্রা) - Quick macOS Installer
set -e

GREEN='\033[0;32m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}====================================================================${NC}"
echo -e "${GREEN}  ⌨️   Installing Matra (মাত্রা) for macOS...${NC}"
echo -e "${CYAN}====================================================================${NC}"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Build .app if not already built
if [ ! -d "$PROJECT_ROOT/dist/Matra.app" ]; then
    bash "$SCRIPT_DIR/build_dmg.sh"
fi

echo "Copying Matra.app to /Applications..."
cp -R "$PROJECT_ROOT/dist/Matra.app" /Applications/

echo -e "${GREEN}====================================================================${NC}"
echo -e "${GREEN}  ✅ Matra.app installed to /Applications successfully!${NC}"
echo -e "${CYAN}  You can launch it from Spotlight or Launchpad.${NC}"
echo -e "${GREEN}====================================================================${NC}"

