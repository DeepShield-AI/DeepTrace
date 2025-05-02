#!/bin/bash
set -euo pipefail

# Navigate to the docker images directory
cd resources/docker/images || { echo "Error: dictory not found" >&2; exit 1; }

# Check the architecture
arch=$(uname -m)
case "$arch" in
    x86_64)  subdir="x86_64" ;;
    aarch64) subdir="aarch64" ;;
    *) 
        echo "Docker images not supported for this architecture: $arch" >&2
        exit 1
        ;;
esac

# Enter the subdirectory for the architecture
cd "$subdir" || { echo "Error: dictory $subdir is not exist" >&2; exit 1; }

# List of Docker images to load
images=("memcached" "redis" "mongo")

# Load Docker images
for image in "${images[@]}"; do
    file="${image}.tar"
    if [ ! -f "$file" ]; then
        echo "Error: File $file does not exist in $subdir directory" >&2
        exit 1
    fi
    # Load the Docker image
    echo "Loading $image image..."
    docker load -i "$file" || exit 1
done

echo "✅ Docker images loaded successfully!"