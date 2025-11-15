# Deployment Modes

DeepTrace supports different deployment modes to accommodate various use cases, from development and testing to production environments.

## Available Deployment Modes

| Mode | Description | Use Case | Complexity |
|------|-------------|----------|------------|
| **All-in-One** | Single host deployment | Development, testing, learning | Simple |
| **Distributed** | Multi-host deployment | Production, large scale | Advanced |

## All-in-One Mode

**For complete all-in-one setup**, see the [All-in-One Deployment Guide](../getting-started/all-in-one.md) and [Quick Start Guide](../getting-started/quick-start.md).

### Key Characteristics

- Single host runs all components
- Simplified configuration and management
- Perfect for learning and development
- Limited scalability

## Distributed Mode

### Overview

Distributed deployment separates DeepTrace components across multiple hosts for production environments. This mode provides:

- **Scalability**: Handle large-scale distributed systems
- **High Availability**: Redundancy and fault tolerance
- **Performance**: Distributed processing and storage
- **Flexibility**: Independent scaling of components

### Architecture

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

### Configuration

For distributed deployment, configure multiple agents in `server/config/config.toml`:

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

### Deployment Steps

1. **Configure server**: Set up server configuration with multiple agents
2. **Deploy server**: Run server on designated host
3. **Install agents**: Deploy agents on target hosts
4. **Verify connectivity**: Ensure all agents can communicate with server
5. **Start monitoring**: Begin collecting traces across all hosts

## Choosing the Right Mode

### All-in-One Mode

**Choose when:**
- Learning DeepTrace functionality
- Developing and testing applications
- Demonstrating tracing capabilities
- Working with small-scale systems

**Limitations:**
- Limited to single host resources
- Not suitable for production
- No high availability

### Distributed Mode  

**Choose when:**
- Deploying to production
- Monitoring large-scale systems
- Requiring high availability
- Need independent scaling

**Considerations:**
- More complex setup and maintenance
- Requires network configuration
- Higher resource requirements

## Next Steps

- **All-in-One**: Start with [Quick Start Guide](../getting-started/quick-start.md)
- **Distributed**: Review [Configuration Guide](../getting-started/configuration.md)
- **Basic Usage**: Learn [essential operations](./basic-usage.md)
- **Database Setup**: Configure [Elasticsearch clusters](./database.md)
