# DeepTrace eBPF Program Documentation

This document provides an overview of the eBPF program developed for DeepTrace, detailing its instrumentation hooks for monitoring network traffic efficiently. The eBPF (Extended Berkeley Packet Filter) program is designed to hook into various system call entry and exit points, as well as socket operations, to observe and potentially alter the behavior of network communication.

## DeepTrace Documentation Overview

This directory contains detailed documentation for various components of the DeepTrace tool. Each subdirectory focuses on different aspects of DeepTrace, providing insights into its implementation and usage. DeepTrace consists of two main components: an **agent** deployed on each host and a **server** running in the microservices cluster. The agent is responsible for collecting request metadata through eBPF, including timestamps, protocol types, request or response types, and linking requests and responses to build spans, while the server is responsible for linking parent-child spans and assembling traces.
The following is a general overview of each module on the agent and server.

### Agent

#### [eBPF](./eBPF/)
The eBPF code in the agent is responsible for capturing request metadata by intercepting 10 system calls related to network communication, including timestamps, process IDs, thread IDs, socket quintuples, and message content. Its documentation includes the following files:

- [**Overview.md**](./eBPF/Overview.md)
  - Description: This file describes the execution process of eBPF code.
- [**Hooks.md**](./eBPF/Hooks.md)
  - Description: Details the specific system hooks that DeepTrace utilizes to monitor and interact with the system. This includes hooks into network events, system calls, and other critical points that DeepTrace uses to extract or inject data.
- [**Maps.md**](./eBPF/Maps.md)
  - Description: Outlines the various maps used by DeepTrace for storing and accessing data efficiently. It describes each map's purpose, structure, and role in the overall functionality of the tool.
- [**Structures.md**](./eBPF/Structures.md)
  - Description: Discusses the data structures defined and used by DeepTrace. This file is crucial for understanding how data is organized, manipulated, and stored during the operation of DeepTrace.
- [**Overhead.md**](./eBPF/Overhead.md)
  - Description: This file demonstrates the performance overhead of DeepTrace's eBPF code on the observed application.

### Server

  - TODO

### Usage
- [**Usage.md**](./usage/Usage.md)
  - Description: Provides step-by-step instructions on how to set up and use the DeepTrace tool. It includes details on configuration, deployment, and operational best practices to help users maximize the tool's capabilities.






## Tested Environments

DeepTrace is tested deployable and runnable on 

- Kubernetes v1.29.0, Kernel version 6.8.0-55-generic, Ubuntu 22.04.2 LTS

We strongly recommend deploying on Linux 6.8.0 version, as there may be strange bugs in lower versions