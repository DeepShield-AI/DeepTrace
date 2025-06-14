#!/bin/bash
set -euo pipefail

cd ./agent
./target/release/agent -f ./config/default.toml > /dev/null 2>&1 & disown