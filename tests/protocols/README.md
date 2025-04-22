# Network Packet Capture Guide (Kubernetes Environment)

1. Locate Target Pod
```sh
kubectl get pods -A | grep <pod-keyword>
```
Filter pods using keywords. For precise matching, specify the namespace with -n.
2. Retrieve Container Information
```sh
kubectl describe pod <pod-name> -n <namespace> | grep "Container ID"
```
Extract the Container ID (e.g., containerd://xxxxx) for subsequent steps.
3. Obtain Container Process PID
* For containerd runtime:
```sh
sudo crictl inspect <container-id> | grep pid
```
* For Docker runtime:
```sh
docker inspect <container-id> -f "{{.State.Pid}}"
```
The PID (e.g., 11956) will be used to enter the container’s network namespace.
4. Enter Container Network Namespace
```sh
sudo nsenter -t <PID> -n ip addr show
```
Verify the network interface inside the container (typically `eth0`, but may vary based on configuration).
5. Start Packet Capture
```sh
sudo nsenter -t <PID> -n tcpdump -i eth0 -c 100 -w protocol_name.pcap
```
* Key Parameters:
```plaintext
-i eth0: Specify the container’s network interface (adjust based on Step 4).
-w: Save output to a .pcap file for analysis with tools like Wireshark.
```
6. Test
Test all
```sh
cargo test --package mercury-ebpf -- --nocapture
```
For test single protocol, use 
```sh
cargo test --package mercury-ebpf -- --nocapture {mongodb|memcached|thrift|redis}

```
