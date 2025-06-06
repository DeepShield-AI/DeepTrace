# DeepTrace Server Configuration Example

This document provides an example and explanation for the `config.toml` file used to configure the DeepTrace server and its agents.

---

## Elasticsearch Configuration

```toml
[elastic]  # Elasticsearch related configuration
elastic_password = "**"         # Password for the Elasticsearch user
port = 9200                     # Elasticsearch server port
address = "**"     # Elasticsearch server address
kibana_password = "**"          # Password for the Kibana user
bulk_size = 1024                # Bulk write size
request_timeout = 10            # Request timeout (seconds)
agent_status_index = "agent_status"  # Index name for agent status
```

## Agent Configuration
Each agent is configured as a table in the agents array. Below is an example for agent1:
```toml
# ========== agent1 configuration ==========
[[agents]]
  [agents.agent_info]  # Basic agent information
  agent_name = "agent1"           # Name of the agent (unique identifier)
  user_name = "root"              # Username for logging into the agent host
  host_ip = "**"                  # IP address of the agent host
  ssh_port = 22                   # SSH port of the agent host
  host_password = "**"            # Password for the agent host
  deeptrace_port = 52001          # DeepTrace service port
  code_path = "/root"             # Path to the code directory on the agent host
  workers = 16                    # Number of worker threads

  [agents.sender]  # Data sending configuration
  index_name = "spans_agent1"     # Elasticsearch index name for this agent's spans
  mem_buffer_size = 16            # Memory buffer size
  file_buffer_size = 32           # File buffer size
  file_size_limit = 1024          # File size limit
  batch_size = 1024               # Batch sending size

  [agents.trace]  # eBPF configuration
  pids = [3056354, 3056217, 3056210]  # List of process PIDs to trace

  [agents.api]  # Agent API service configuration
  port = 7899                   # API service port
  address = "0.0.0.0"           # API listening address
  workers = 1                   # Number of API service threads
  ident = "deeptrace"           # Service identifier
```

## Note

- Replace `**` with your actual passwords and IP addresses.
- You can add multiple agents by duplicating the [[agents]] block and modifying the relevant fields.
