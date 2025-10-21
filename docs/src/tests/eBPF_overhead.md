# DeepTrace eBPF Overhead Testing Guide

This guide provides instructions for testing the eBPF overhead of DeepTrace.
## Setup & Execution

### Inject eBPF Program into Kernel

```bash
cd DeepTrace/agent
cp config/default.toml.example config/default.toml
RUST_LOG=info cargo run --release --config 'target."cfg(all())".runner="sudo -E"' -- -f config/default.toml
```

### Repeatedly Call a Syscall and Measure the Needed Time (Repeat 10^5 times, take the average of 100)

```bash
cd tests/eBPF/overhead
bash run.sh <syscall>
```

\<syscall\> = write | read | sendto | recvfrom | sendmsg | sendmmsg | recvmsg | recvmmsg |writev | readv | empty | ssl_write | ssl_read |ssl | empty

_Note: While test recvfrom, sendto, recvmsg, sendmsg, recvmmsg, sendmmsg, you need to call the sending syscall with the corresponding receiving syscall._