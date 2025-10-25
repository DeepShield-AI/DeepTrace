# Test Overview

This directory provides test documents and corresponding test code for different modules of DeepTrace.

### [eBPF](./eBPF/)

- [**Overhead**](./eBPF/overhead)
  - eBPF code's impact on application performance overhead

- [**Functionality**](./eBPF/functionality)
  - Integrity verification of data captured by eBPF code


### [Protocol](./Protocol/)

- TODO

### [Span](./Span/)

- TODO




## Tested Environments

DeepTrace is tested deployable and runnable on 

- Kubernetes v1.29.0, Kernel version 6.8.0-55-generic, Ubuntu 22.04.2 LTS

We strongly recommend deploying on Linux 6.8.0 version, as there may be strange bugs in lower versions