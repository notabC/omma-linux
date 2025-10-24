#!/bin/bash
# Build script for Omma

echo "Building Omma..."
source "$HOME/.cargo/env"
cargo build --release

if [ $? -eq 0 ]; then
    echo "✓ Build successful!"
    echo "Binary location: ./target/release/omma"
    echo ""
    echo "To run: ./run.sh"
else
    echo "✗ Build failed!"
    exit 1
fi
