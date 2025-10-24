#!/bin/bash
# Build script for Raspberry Pi deployment

set -e

echo "Building Hello GTK Pi for Raspberry Pi..."

# Check if we're on the Pi or need to deploy
if [ -f "/proc/device-tree/model" ] && grep -q "Raspberry Pi" /proc/device-tree/model; then
    echo "Running on Raspberry Pi - building natively"
    cargo build --release
    echo "Build complete! Binary at: ./target/release/hello-gtk-pi"
    echo "Run with: ./target/release/hello-gtk-pi"
else
    echo "Not on Raspberry Pi. Options:"
    echo "1. Transfer this directory to your Pi and build there (recommended)"
    echo "2. Use Docker for cross-compilation (see DEPLOY.md)"
    echo ""
    echo "To transfer to Pi (from parent directory):"
    echo "  scp -r . pi@raspberrypi.local:~/hello-gtk-pi"
    echo ""
    echo "Then on Pi:"
    echo "  cd hello-gtk-pi"
    echo "  ./build-for-pi.sh"
fi
