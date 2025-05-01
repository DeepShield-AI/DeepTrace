# Test Steps

## Deployment Workload Server
1. Connect to the deployment server via SSH:
```sh
ssh ubuntu@202.112.237.40
```
2. Navigate to the protocol example directory:
```sh
cd ~/smore/beta/test/{protocol}  # Replace {protocol} with mongodb/memcached/redis
```
3. Deploy Kubernetes resources:
```sh
kubectl apply -f {protocol}_ns.yaml
kubectl apply -f {protocol}_deployment.yaml
kubectl apply -f {protocol}_service.yaml
```
_Note: We provide three protocol examples (mongodb/memcached/redis). For additional protocols, create corresponding YAML files._

## Locate Target Pod
1. List pods with keyword filtering:
```sh
kubectl get pods -A | grep <pod-keyword>
```
_For precise matching, specify the namespace using -n <namespace>._

## Retrieve Container Information
1. Get container ID from pod details:
```sh
kubectl describe pod <pod-name> -n <namespace> | grep "Container ID"
```
_Output example: containerd://xxxxx - retain this ID for subsequent steps._

## Obtain Container Process PID
1. Connect to the eBPF host:
```sh
ssh ubuntu@202.112.237.33
```
2. Get PID based on container runtime.
* For containerd runtime:
```sh
sudo crictl inspect <container-id> | grep pid
```
* For Docker runtime:
```sh
docker inspect <container-id> -f "{{.State.Pid}}"
```
## Capture eBPF message
1. Start eBPF monitoring in one terminal:
```sh
RUST_LOG=info cargo run --release --config 'target."cfg(all())".runner="sudo -E"' -- --pids <PID>
```
2. In a separate terminal, generate workload traffic:
```sh
cd ~/smore/deeptrace/tests/workload/
source env/bin/activate  # Activate Python virtual environment
cd {protocol}            # Target protocol directory
python3 client.py
```
3. Terminate the eBPF program after ~5 seconds of traffic generation. _Output file: `tests/output/ebpf.txt`_

## Validate Results
1. Move and analyze the eBPF output:
```sh
mv tests/output/ebpf.txt tests/workload/{protocol}/result/
cd workload
python3 parse_ebpf.py --protocol {protocol}   # Process raw data
python3 check.py --protocol {protocol}        # Calculate accuracy metrics
```
_Final accuracy results will be displayed in the terminal._
