#!/bin/bash
set -euo pipefail


# Set up environment variables
source "$HOME/.cargo/env"
if ! grep -q 'source "$HOME/.cargo/env"' ~/.bashrc; then
    echo 'source "$HOME/.cargo/env"' >> ~/.bashrc
fi

# RUST_LOG=info cargo run --release 
RUST_LOG=info cargo run --release > /dev/null 2>&1 & disown