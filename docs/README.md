# DeepTrace eBPF Program Documentation

This document provides an overview of the eBPF program developed for DeepTrace, detailing its instrumentation hooks for monitoring network traffic efficiently. The eBPF (Extended Berkeley Packet Filter) program is designed to hook into various system call entry and exit points, as well as socket operations, to observe and potentially alter the behavior of network communication.

## DeepTrace Documentation Overview

This directory contains detailed documentation for various components of the DeepTrace tool. Each subdirectory focuses on different aspects of DeepTrace, providing insights into its implementation and usage. Below is an overview of what each documentation file covers:

### Files

- [**Hooks.md**](./Hooks.md)
  - Description: Details the specific system hooks that DeepTrace utilizes to monitor and interact with the system. This includes hooks into network events, system calls, and other critical points that DeepTrace uses to extract or inject data.
- [**Maps.md**](./Maps.md)
  - Description: Outlines the various maps used by DeepTrace for storing and accessing data efficiently. It describes each map's purpose, structure, and role in the overall functionality of the tool.
- [**Structures.md**](./Structures.md)
  - Description: Discusses the data structures defined and used by DeepTrace. This file is crucial for understanding how data is organized, manipulated, and stored during the operation of DeepTrace.
- [**Usage.md**](./Usage.md)
  - Description: Provides step-by-step instructions on how to set up and use the DeepTrace tool. It includes details on configuration, deployment, and operational best practices to help users maximize the tool's capabilities.
- [**Overview.md**](./Overview.md)
  - Description: Provides an in-depth explanation of how context propagation is implemented in DeepTrace. It covers the mechanisms used to track and manage context across different parts of the system.


Each document is designed to provide a comprehensive understanding of its respective topic, ensuring that developers and users alike can get the most out of DeepTrace.

Please refer to each individual file for more detailed information on each topic.

## Prerequisites

1. stable rust toolchains: `rustup toolchain install stable`
2. nightly rust toolchains: `rustup toolchain install nightly --component rust-src`
3. (if cross-compiling) rustup target: `rustup target add ${ARCH}-unknown-linux-musl`
4. (if cross-compiling) LLVM: (e.g.) `brew install llvm` (on macOS)
5. (if cross-compiling) C toolchain: (e.g.) [`brew install filosottile/musl-cross/musl-cross`](https://github.com/FiloSottile/homebrew-musl-cross) (on macOS)
6. bpf-linker: `cargo install bpf-linker` (`--no-default-features` on macOS)

## Source Code Architecture

## Tested Environments

DeepTrace is tested deployable and runnable on 

- Kubernetes v1.29.0, Kernel version 6.8.0-55-generic, Ubuntu 22.04.2 LTS

We strongly recommend deploying on Linux 6.8.0 version, as there may be strange bugs in lower versions