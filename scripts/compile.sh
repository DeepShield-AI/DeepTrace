#!/bin/bash
set -euo pipefail


# Set up environment variables
source "$HOME/.cargo/env"
if ! grep -q 'source "$HOME/.cargo/env"' ~/.bashrc; then
    echo 'source "$HOME/.cargo/env"' >> ~/.bashrc
fi
echo "source ~/.bashrc"
cargo build --release
echo "compile done"


# RUST_LOG=info cargo run --release --config 'target."cfg(all())".runner="sudo -E"'