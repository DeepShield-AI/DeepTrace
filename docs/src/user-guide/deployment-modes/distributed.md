# Distributed Mode

Distributed deployment separates DeepTrace components across multiple hosts for production environments. This mode provides scalability, high availability, and performance for large-scale distributed systems.

## Architecture Overview

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Server Host   │    │   Agent Host 1  │    │   Agent Host 2  │
│                 │    │                 │    │                 │
│ ┌─────────────┐ │    │ ┌─────────────┐ │    │ ┌─────────────┐ │
│ │ DeepTrace   │ │    │ │ DeepTrace   │ │    │ │ DeepTrace   │ │
│ │   Server    │◄┼────┼►│   Agent     │ │    │ │   Agent     │ │
│ └─────────────┘ │    │ └─────────────┘ │    │ └─────────────┘ │
│ ┌─────────────┐ │    │ ┌─────────────┐ │    │ ┌─────────────┐ │
│ │Elasticsearch│ │    │ │Microservices│ │    │ │Microservices│ │
│ │  Database   │ │    │ │    Apps     │ │    │ │    Apps     │ │
│ └─────────────┘ │    │ └─────────────┘ │    │ └─────────────┘ │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## Key Benefits

- **Scalability**: Handle large-scale distributed systems
- **High Availability**: Redundancy and fault tolerance
- **Performance**: Distributed processing and storage
- **Flexibility**: Independent scaling of components

## Configuration

Configure multiple agents in `server/config/config.toml`:

```toml
[server]
ip = "192.168.1.100"  # Server host IP

[[agents]]
  [agents.agent_info]
  agent_name = "agent-1"
  host_ip = "192.168.1.101"    # Different from server IP
  user_name = "ubuntu"
  ssh_port = 22
  host_password = "password"

[[agents]]
  [agents.agent_info]  
  agent_name = "agent-2"
  host_ip = "192.168.1.102"    # Another host
  user_name = "ubuntu"
  ssh_port = 22
  host_password = "password"
```

## Deployment Steps

1. **Configure server**: Set up server configuration with multiple agents
2. **Deploy server**: Run server on designated host
3. **Install agents**: Deploy agents on target hosts
4. **Verify connectivity**: Ensure all agents can communicate with server
5. **Start monitoring**: Begin collecting traces across all hosts

## Requirements

- **Multiple hosts**: At least 2 hosts (1 server + 1+ agents)
- **Network connectivity**: All hosts must communicate
- **SSH access**: Server needs SSH access to agent hosts
- **Resources**: Varies by scale and workload

## Use Cases

**Choose distributed mode when:**
- Deploying to production
- Monitoring large-scale systems
- Requiring high availability
- Need independent scaling

## Considerations

- More complex setup and maintenance
- Requires network configuration
- Higher resource requirements
- Need proper security configuration

## Next Steps

- **[Configuration Guide](../../getting-started/configuration.md)**: Detailed configuration options
- **[Basic Usage](../basic-usage.md)**: Learn essential operations
- **[Database Management](../database.md)**: Configure Elasticsearch clusters
