# DeepTrace Span Construct Testing Guide

The DeepTrace system is designed to construct spans from network transactions. This guide will walk you through the process of testing span construction, including identifying the request-response relationships and calculating span construct accuracy.

## Overview

In this guide, we need to set up a test environment, initiate a DeepTrace container, identify target processes, start the DeepTrace collector, generate test spans, and validate span construction.

## Setup & Execution

### Start Workload Services

```bash
cd DeepTrace
docker-compose -f deployment/docker/Workload.yaml up -d
# Verify:
docker ps
# Example output:
# a69c7abafa85   memcached:1.6.7   "docker-entrypoint.s…"   7 hours ago   Up 7 hours   0.0.0.0:11211->11211/tcp   memcached-workload
# a27b67cd869a   mongo:5.0.15      "docker-entrypoint.s…"   7 hours ago   Up 7 hours   0.0.0.0:27017->27017/tcp   mongo-workload
# 631c9e145055   redis:6.2.4       "docker-entrypoint.s…"   7 hours ago   Up 7 hours   0.0.0.0:6379->6379/tcp     redis-workload
```

This launches memcached, redis, and mongo server containers in background.
### Initiate DeepTrace

```bash
chmod +x ./scripts/run.sh
./scripts/run.sh
```

### Generate Test Spans

```bash
cd tests
python3 -m venv workload/env
source workload/env/bin/activate
pip install redis python-binary-memcached
python3 -m workload.prepare_spans
# Output:
# redis workload completed successfully.
# memcached workload completed successfully.
```

This script:
    Generates synthetic workload patterns
### Stop Collection
Use Ctrl+C in DeepTrace container to:
    Finalize trace data
    Generate span artifacts
    Clean up tracing resources
### Validate Span Construction

```bash
cd tests/workload
python3 test_span_construct.py
# Output:
# Protocol: Redis
# Total:  1000
# Correct:  1000
# Accuracy:  1.0
# No spans found for HTTP1 protocol.
# Protocol: Memcached
# Total:  1000
# Correct:  1000
# Accuracy:  1.0
```

This script list the protocols and their accuracy. For example, the accuracy for Redis is 1.0, indicating that all spans were correctly constructed.
