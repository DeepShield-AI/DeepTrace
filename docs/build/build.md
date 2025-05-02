# DeepTrace Agent Compilation Guide

## Use Docker

The easiest way is to use our docker image:

### Ⅰ. Docker Installation by OS

You can install Docker by following the official instructions: [Docker Installation](https://docs.docker.com/get-started/get-docker/)

Check if Docker is installed correctly:
```bash
docker --version
```

---

### Ⅱ. Image Import Operations

Once Docker is installed, you can import our pre-built Docker image:
```bash
cd DeepTrace/
chmod +x ./scripts/load_docker_images.sh
./scripts/load_docker_images.sh

# Check
docker images | grep deeptrace
```
If you load the images, you don't need to compile the image again.

---  

### Ⅲ. Build & Deployment (Optional)

#### 1. **Compile Agent**
```bash
cd DeepTrace

docker build \
  --build-arg APP_NAME=deeptrace \
  --network=host \
  -t deeptrace \
  -f deployment/docker/Dockerfile .
```

_Note: The Docker image build process can be time-consuming. Please be patient during this period and ensure stable network connectivity to both github.com and Docker Hub (hub.docker.com), especially if relying on remote image repositories rather than local cached images._

#### 2. **Check Image**
```bash
docker images | grep deeptrace
# Output:
deeptrace            latest      04ed8cf89494   now    5.89GB
```
The output of `docker images | grep deeptrace` should show the image you just built. If it doesn't, you may need to check your Docker build process or network connectivity.

## Manually compilation

TODO