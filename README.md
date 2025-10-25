# What is DeepTrace

**DeepTrace** is a non-intrusive distributed tracing framework designed for microservices, enabling accurate end-to-end observation of request execution paths without requiring code instrumentation. It leverages transaction semantics derived from request content (e.g., API endpoints and persistent fields like user IDs) to categorize requests into logical groups called *transactions*. By combining transaction analysis with multidimensional metrics (e.g., temporal proximity and causal patterns), DeepTrace achieves over 95% tracing accuracy even under high concurrency. Deployed in production systems across industries (e.g., finance, e-commerce), it supports troubleshooting tasks like latency diagnosis and DDoS analysis while minimizing overhead.

# Key Features

### 1. Protocol-Aware Span Construction  
DeepTrace uses **eBPF-based packet capture** and **protocol templates** to non-intrusively parse over 20 application-layer protocols (e.g., HTTP, gRPC, Redis). It segments requests via length-field jumps (e.g., MongoDB’s `OpCode`) or full parsing for protocols lacking length fields (e.g., Redis), ensuring accurate request boundary detection. Unlike intrusive tools (e.g., Jaeger), it avoids manual code changes, and unlike prior non-intrusive solutions, it eliminates computationally expensive full-payload inspection. This enables efficient span creation with critical metadata like API endpoints and request sizes, foundational for downstream correlation.

### 2. Transaction-Based Span Correlation  
DeepTrace introduces a **dual-phase transaction inference** mechanism:  
- **Nested API affinity**: Computes traffic intensity correlations (Pearson coefficient) between APIs to identify parent-child relationships (e.g., `Login → VerifyID`).  
- **Persistent field similarity**: Uses TF-IDF-weighted cosine similarity to filter schema noise (e.g., version numbers) and isolate transaction fields (e.g., user IDs).  
These probabilities are fused with metrics (delays, durations) via entropy-weighted adaptive scoring, prioritizing transaction semantics when available and falling back to causality metrics otherwise. This approach reduces misattributions by 15% compared to delay/FIFO-based methods under concurrency.

### 3. Query-Driven Trace Assembling  
To minimize overhead, DeepTrace employs **on-host compression** and **dual-indexing** (tag-based inverted indexes + metric histograms). Operators submit queries (e.g., "traces with latency >95th percentile"), triggering iterative trace reconstruction: the server collects relevant span mappings from agents, expands traces by fetching parent/child spans, and discards unrelated data. This avoids centralized span collection, reducing transmission overhead by **94%** compared to frameworks like Jaeger (100% sampling) while retaining query flexibility. Span data is ephemerally cached on agents, alleviating memory pressure.


# Documentation

You can get more information in our comprehensive [documentation](https://deepshield-ai.github.io/DeepTrace/index.html)

# Getting Started with DeepTrace

Welcome to DeepTrace! You can refer to [All-in-One.md](docs/usage/All-in-One.md) to deploy DeepTrace.


# Software Architecture

DeepTrace consists of two components, Agent and Server. An Agent runs in each K8s node, legacy host and cloud host, and is responsible for non-intrusive request collection of all application processes on the host. Server runs in a K8s cluster and provides Agent management, trace reconstruction, data ingest and query services.


