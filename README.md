# What is DeepTrace

The DeepTrace open-source project aims to provide deep observability for complex cloud-native applications. DeepTrace implements kernel offloading for request collection using eBPF, not only achieving **zero code** monitoring but also accelerating request collection through the **kernel offloading**. 
To reconstruct the end-to-end trace of requests, DeepTrace offers a trace reconstruction strategy based on **method-level delay distributions**, selecting the request execution path that best matches the causal delay distribution among different methods within the application.
It is capable of providing a tracing accuracy rate of over 90% in a non-intrusive manner.

# Key Features

- **In-Kernel Request Collection** DeepTrace employs a hybrid kernel-user space architecture to optimize request collection. For non-multiplexed protocols (such as HTTP/1 and MongoDB), it leverages custom static protocol parsing rules and eBPF technology to offload the collection task to the kernel. This avoiding the substantial overhead of copying large amounts of messages from the kernel to user space. Meanwhile, the user space retains the capability to handle complex protocols (such as HTTP/2 and gRPC).

- **Universal Map** DeepTrace provides a **zero-code** universal map implemented by eBPF for production environments, covering application services, AI services, and infrastructure services in **any language**. It also uses Grafana to query and visualize the collected data.

- **Distributed Tracing** DeepTrace achieves precise end-to-end request tracing through **method-level delay distribution**. It assesses the parent-child relationship probabilities of candidate span mappings based on the estimated delay distributions of different method pairs.  By employing an iterative algorithm to correlate spans, DeepTrace sorts the mappings by probability and iteratively selects subsets of mappings that satisfy the constraints, efficiently reconstructing the request trace.


# Documentation


# Get Started

It takes just a few minutes to install DeepTrace. To get started, check out the Install Guides.

## Compile DeepTrace from Source Code

- [Compile DeepTrace use docker](docs/build/build.md)

## Test DeepTrace
- [Test DeepTrace](docs/tests/README.md)

# Software Architecture

DeepTrace consists of two components, Agent and Server. An Agent runs in each K8s node, legacy host and cloud host, and is responsible for non-intrusive request collection of all application processes on the host. Server runs in a K8s cluster and provides Agent management, trace reconstruction, data ingest and query services.


