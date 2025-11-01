#!/bin/bash
set -euo pipefail

cd ./agent
RUST_LOG=info ./target/x86_64-unknown-linux-gnu/release/deeptrace -c config/deeptrace.toml > /dev/null 2>&1 & disown