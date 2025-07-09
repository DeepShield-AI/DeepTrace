#!/bin/bash
set -euo pipefail

DOCKER_CONFIG="/etc/docker/daemon.json"
TARGET_REGISTRY="47.97.67.233:5000"
TEMP_FILE=$(mktemp)

sudo apt-get update
sudo apt-get install -y jq docker.io

if [ ! -f "$DOCKER_CONFIG" ] || [ ! -s "$DOCKER_CONFIG" ]; then
    echo "{}" | sudo tee "$DOCKER_CONFIG" >/dev/null
    echo "Created new $DOCKER_CONFIG"
fi

if ! sudo jq empty "$DOCKER_CONFIG" &>/dev/null; then
    echo "Error: Invalid JSON in $DOCKER_CONFIG" >&2
    exit 1
fi

if sudo jq -e --arg registry "$TARGET_REGISTRY" '
    .["insecure-registries"] // [] | index($registry)
' "$DOCKER_CONFIG" >/dev/null; then
    echo "Registry $TARGET_REGISTRY is already configured in $DOCKER_CONFIG"
    echo "No Docker restart needed."
else
    echo "Registry $TARGET_REGISTRY is not present, adding..."
    sudo jq --arg registry "$TARGET_REGISTRY" '
        .["insecure-registries"] = (
            (.["insecure-registries"] // []) + [$registry] | unique
        )
    ' "$DOCKER_CONFIG" > "$TEMP_FILE" && sudo mv "$TEMP_FILE" "$DOCKER_CONFIG"
    echo "Reloading and restarting Docker due to config changes..."
    sudo systemctl daemon-reload
    sudo systemctl restart docker
    echo "Docker restarted."
fi