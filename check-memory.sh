#!/bin/bash
# Check memory usage of Omma

if pgrep -f "./target/release/omma" > /dev/null; then
    echo "=== Omma Memory Usage ==="
    ps aux | grep "./target/release/omma" | grep -v grep | awk '{
        printf "RAM: %.2f MB\n", $6/1024
        printf "CPU: %s%%\n", $3
        printf "PID: %s\n", $2
    }'
    echo ""
    echo "For real-time monitoring, run:"
    echo "  watch -n 1 ./check-memory.sh"
else
    echo "App is not running."
    echo "Start it with: ./run.sh"
fi
