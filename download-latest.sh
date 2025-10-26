#!/bin/bash
# Download the latest Omma build from GitHub Actions

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}Omma Linux - Download Latest Build${NC}"
echo "=========================================="

# Detect architecture
ARCH=$(uname -m)
if [ "$ARCH" = "aarch64" ]; then
    TARGET="aarch64-unknown-linux-gnu"
    echo -e "${GREEN}Detected: ARM64 (64-bit)${NC}"
elif [ "$ARCH" = "armv7l" ]; then
    TARGET="armv7-unknown-linux-gnueabihf"
    echo -e "${GREEN}Detected: ARMv7 (32-bit)${NC}"
else
    echo -e "${RED}Unsupported architecture: $ARCH${NC}"
    echo "This script is designed for Raspberry Pi (ARM) systems."
    exit 1
fi

# GitHub repository (update with your username)
REPO="YOUR_GITHUB_USERNAME/omma-linux"
echo -e "${BLUE}Repository: $REPO${NC}"

# Check for GitHub CLI
if command -v gh &> /dev/null; then
    echo -e "${BLUE}Using GitHub CLI to download latest artifact...${NC}"

    # Download using gh CLI
    gh run download --repo "$REPO" --name "omma-$TARGET" --dir /tmp/omma-download

    if [ $? -eq 0 ]; then
        # Move binary
        sudo mv /tmp/omma-download/omma-$TARGET /usr/local/bin/omma
        sudo chmod +x /usr/local/bin/omma
        rm -rf /tmp/omma-download

        echo -e "${GREEN}✓ Omma installed successfully to /usr/local/bin/omma${NC}"
        echo -e "${BLUE}Run 'omma' to start the application${NC}"
    else
        echo -e "${RED}Failed to download artifact${NC}"
        exit 1
    fi
else
    echo -e "${BLUE}GitHub CLI not found. Using release download method...${NC}"
    echo ""
    echo "To download from the latest release:"
    echo -e "${GREEN}1. Visit: https://github.com/$REPO/releases/latest${NC}"
    echo -e "${GREEN}2. Download: omma-$TARGET${NC}"
    echo -e "${GREEN}3. Run: chmod +x omma-$TARGET && sudo mv omma-$TARGET /usr/local/bin/omma${NC}"
    echo ""
    echo "Or install GitHub CLI:"
    echo -e "${BLUE}sudo apt-get install gh${NC}"
fi

# Check dependencies
echo ""
echo -e "${BLUE}Checking dependencies...${NC}"
if ! dpkg -l | grep -q libgtk-4-1; then
    echo -e "${RED}Missing dependency: libgtk-4-1${NC}"
    echo "Install with: sudo apt-get install -y libgtk-4-1 libadwaita-1-0"
else
    echo -e "${GREEN}✓ Dependencies OK${NC}"
fi
