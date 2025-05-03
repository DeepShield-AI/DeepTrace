# DeepTrace eBPF Protocol Infer Testing Guide

## Deployment Workload Server

First, deploy a workload server.

You can easily deploy a workload server using Docker or write a simple Python script.

_Note: We provide three protocol examples (mongodb/memcached/redis)._

## Obtain Container Process PID

### Retrieve container ID.
```bash
docker ps
```

### Get PID based on container runtime.

```bash
docker inspect <container-id> -f "{{.State.Pid}}"
```

## Capture eBPF message
### Start eBPF monitoring:
-  In one terminal
```bash
RUST_LOG=info cargo run --release --config 'target."cfg(all())".runner="sudo -E"' -- --pids <PID>
```
- In another terminal, generate workload traffic:
```bash
cd tests/eBPF/protocols/
python3 -m venv env  # Create Python virtual environment
source env/bin/activate  # Activate Python virtual environment
pip install -r redis python-binary-memcached pymongo
cd {protocol}            # Target protocol directory
python3 client.py
```

_Note: Currently, we only support mongodb, redis, and memcached protocol. For other protocols, you can modify the client.py script to generate traffic for your desired protocol._

### Terminate the eBPF program after ~5 seconds of traffic generation. 

Output file: `tests/output/ebpf.txt`

## Validate Results
### Move and analyze the eBPF output:
```bash
cd tests/eBPF/protocols
python3 parse_ebpf.py --protocol {protocol}   # Process raw data
python3 check.py --protocol {protocol}        # Calculate accuracy metrics
```
_Final accuracy results will be displayed in the terminal._
