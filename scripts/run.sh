#!/bin/bash
set -euo pipefail

# Check Workloads
echo "Checking workloads..."
pids=$(pgrep -f "memcached|redis" | tr '\n' ',' | sed 's/,$//')

if [ -z "$pids" ]; then
    echo "Workloads not found. Please start the workloads first." >&2
    exit 1
fi

echo "Workloads found: ${pids//,/ , }"

# Start DeepTrace
echo "Starting DeepTrace... (PID: ${pids})"
RUST_LOG=info cargo run \
    --release \
    --config 'target."cfg(all())".runner="sudo -E"' -- \
    --pids "$pids"
    