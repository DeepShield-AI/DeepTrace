# DeepTrace Workload Testing Guide

## Prerequisites

Docker Environment: Ensure Docker and Docker Compose are installed (v20.10+ recommended)

1. Build DeepTrace with docker

    ```bash
    docker build --build-arg APP_NAME=deeptrace --network=host -t deeptrace -f deployment/docker/Dockerfile .
    ```

## Setup & Execution

1. Start Workload Services

    In `deeptrace` folder, use

    ```bash
    docker-compose -f deployment/docker/Workload.yaml up -d
    ```

    This launches memcached, redis, and mongo server containers in background.
2. Initiate DeepTrace Container

    ```bash
    docker run --privileged --pid=host -it deeptrace bash
    ```

    Note: The --privileged flag and PID namespace sharing are required for system tracing  
3. Mount Tracing Subsystem

    ```bash
    mount -t tracefs nodev /sys/kernel/tracing
    ```

    This exposes kernel tracing capabilities to DeepTrace.
4. Identify Target Processes

    ```bash
    pgrep -f "memcached|redis"
    ```

    Example output:
    91634 # memcached
    91636 # redis
    _Note: These PIDs will be used for performance monitoring and trace collection._
5. Start DeepTrace Collector

    ```bash
    cargo run --release -- --pids 91634,91636
    ```

    Flags explanation:
        --release: Optimized build for performance-sensitive tracing
        --pids: Specifies processes to monitor (comma-separated)
6. Generate Test Spans (Host Machine)
    From host terminal:

    ```bash
    cd tests
    python3 -m venv workload/env
    source workload/env/bin/activate
    pip install redis python-binary-memcached
    python3 -m workload.prepare_spans
    ```

    This script:
        Generates synthetic workload patterns
7. Stop Collection
    Use Ctrl+C in DeepTrace container to:
        Finalize trace data
        Generate span artifacts
        Clean up tracing resources
8. Validate Span Construction
    Within container:

    ```bash
    cd tests/workload
    python3 test_span_construct.py
    ```

    Validation checks
