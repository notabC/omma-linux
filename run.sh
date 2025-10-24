#!/bin/bash
# Quick launcher for Omma

# Check if app is already running (match exact binary path)
if pgrep -f "./target/release/omma" > /dev/null; then
    echo "Omma is already running!"
    echo ""
    ps aux | grep "./target/release/omma" | grep -v grep | awk '{print "RAM Usage: " $6/1024 " MB\nCPU: " $3 "%"}'
else
    echo "Starting Omma..."
    ./target/release/omma &
    sleep 1
    echo "App started!"
fi
