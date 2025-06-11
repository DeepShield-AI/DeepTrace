#!/bin/bash
set -euo pipefail

DOCKER_CONFIG="/etc/docker/daemon.json"
TARGET_REGISTRY="47.97.67.233:5000"
TEMP_FILE=$(mktemp)
CONFIG_MODIFIED=false

if [ ! -f "$DOCKER_CONFIG" ] || [ ! -s "$DOCKER_CONFIG" ]; then
    echo "{}" | sudo tee "$DOCKER_CONFIG" >/dev/null
    echo "Created $DOCKER_CONFIG"
fi

if ! jq empty "$DOCKER_CONFIG" &>/dev/null; then
    echo "Error: Invalid JSON in $DOCKER_CONFIG" >&2
    exit 1
fi

if jq -e ".insecure-registries // [] | index(\"$TARGET_REGISTRY\")" "$DOCKER_CONFIG" &>/dev/null; then
    echo "Registry $TARGET_REGISTRY is already configured in $DOCKER_CONFIG"
else
    jq --arg registry "$TARGET_REGISTRY" '
        .insecure-registries |= (
            if type=="array" then 
                if index($registry) then . else . + [$registry] end 
            else 
                [$registry] 
            end
        )
    ' "$DOCKER_CONFIG" > "$TEMP_FILE" && sudo mv "$TEMP_FILE" "$DOCKER_CONFIG"
    
    echo "Added $TARGET_REGISTRY to $DOCKER_CONFIG"
    sudo systemctl daemon-reload
    sudo systemctl restart docker
    echo "Docker restarted due to configuration changes"
fi

docker pull 47.97.67.233:5000/deepshield/deeptrace:latest

docker run --privileged --rm -it -v $(pwd):/DeepTrace 47.97.67.233:5000/deepshield/deeptrace bash -c \
'cd /DeepTrace/agent &&
aya-tool generate task_struct user_msghdr mmsghdr tcp_sock socket files_struct > src/trace/ebpf/src/vmlinux.rs &&
sed -i '"'"'2i\#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unnecessary_transmutes)]'"'"' src/trace/ebpf/src/vmlinux.rs &&
cargo build --release'