# Single Host Mode

Single host mode (also known as All-in-One mode) runs both the DeepTrace server and agent on a single host. This is the **recommended starting point** for new users and ideal for development, testing, and learning.

## Architecture Overview

```
┌─────────────────────────────────────────────────┐
│              Single Host (All-in-One)           │
│                                                 │
│  ┌──────────────┐      ┌──────────────┐       │
│  │  DeepTrace   │◄────►│ Elasticsearch│       │
│  │   Server     │      │   Database   │       │
│  └──────────────┘      └──────────────┘       │
│         ▲                                      │
│         │                                      │
│         ▼                                      │
│  ┌──────────────┐      ┌──────────────┐       │
│  │  DeepTrace   │◄────►│ Microservice │       │
│  │    Agent     │      │     Apps     │       │
│  └──────────────┘      └──────────────┘       │
│                                                 │
└─────────────────────────────────────────────────┘
```

## Key Benefits

- **Simple Setup**: Single command deployment
- **Quick Learning**: Understand all components on one host  
- **Easy Testing**: Perfect for evaluation and development
- **Minimal Resources**: Requires only one host
- **Fast Iteration**: Quick development and testing cycles

## Configuration

In single host mode, the server and agent IPs must be identical:

```toml
[server]
ip = "192.168.1.100"              # Your host IP

[[agents]]
  [agents.agent_info]
  agent_name = "local-agent"
  host_ip = "192.168.1.100"       # Same as server.ip
  user_name = "ubuntu"
  ssh_port = 22
  host_password = "password"
```

> 💡 **Important**: `server.ip` and `agents.agent_info.host_ip` must be identical.

## Quick Start

For complete single host deployment, follow the [Quick Start Guide](../../getting-started/quick-start.md):

1. **Clone repository** and configure settings
2. **Deploy server** (DeepTrace + Elasticsearch)
3. **Deploy sample app** (BookInfo or Social Network)
4. **Install agent** (compiles and starts monitoring)
5. **Generate traffic** and build traces
6. **View results** in Kibana dashboard

## Requirements

- **OS**: Ubuntu 24.04 LTS
- **Memory**: 8GB RAM minimum
- **Storage**: 40GB free space
- **Docker**: v26.1.3+
- **Network**: Internet connectivity

## Use Cases

**Choose single host mode when:**
- Learning DeepTrace functionality
- Developing and testing applications
- Demonstrating tracing capabilities
- Working with small-scale systems
- Prototyping and proof-of-concept work

## Limitations

- Limited to single host resources
- Not suitable for production at scale
- No high availability
- Cannot distribute load across multiple hosts
- Limited by single host performance

## Comparison with Distributed Mode

| Aspect | Single Host | Distributed |
|--------|------------|-------------|
| **Complexity** | Simple, single host | Complex, multiple hosts |
| **Use Case** | Testing, development | Production, large scale |
| **Resources** | 8GB RAM, 40GB disk | Varies by scale |
| **Scalability** | Limited to one host | Highly scalable |
| **Maintenance** | Easy | Requires orchestration |
| **Setup Time** | 10 minutes | Hours to days |

## Troubleshooting

### Common Issues

1. **Port conflicts**: Ensure ports 5601, 9200, and application ports are available
2. **Resource constraints**: Monitor memory and disk usage
3. **Docker issues**: Verify Docker is running and has sufficient resources
4. **Network connectivity**: Check that services can communicate

### Performance Optimization

```bash
# Monitor resource usage
sudo docker stats

# Check available disk space
df -h

# Monitor memory usage
free -h

# Check system load
top
```

## Migration to Distributed

When ready to move to production, you can migrate to distributed mode:

1. **Export configuration**: Save current settings
2. **Plan architecture**: Design multi-host deployment
3. **Configure distributed setup**: Update configuration files
4. **Deploy incrementally**: Start with server, then add agents
5. **Validate functionality**: Ensure all components work correctly

## Next Steps

- **[Quick Start Guide](../../getting-started/quick-start.md)**: Complete deployment walkthrough
- **[Distributed Mode](./distributed.md)**: Learn about production deployment
- **[Basic Usage](../basic-usage.md)**: Explore DeepTrace operations
- **[Workload Applications](../workloads.md)**: Deploy test applications
