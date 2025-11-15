# All-in-One Deployment

All-in-one deployment runs both the DeepTrace server and agent on a single host. This is the **recommended starting point** for new users.

> 🚀 **Ready to start?** Follow the [Quick Start Guide](./quick-start.md) for complete step-by-step instructions.

## What is All-in-One Mode?

In all-in-one deployment, all DeepTrace components run on the same host:

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

## Requirements

- **OS**: Ubuntu 24.04 LTS
- **Memory**: 8GB RAM minimum
- **Storage**: 40GB free space
- **Docker**: v26.1.3+

## Key Configuration Note

In all-in-one mode, the server and agent IPs must be identical:

```toml
[server]
ip = "192.168.1.100"              # Your host IP

[[agents]]
  [agents.agent_info]
  host_ip = "192.168.1.100"       # Same as server.ip
```

## Next Steps

- **[Quick Start Guide](./quick-start.md)** - Complete deployment walkthrough
- **[Configuration Guide](./configuration.md)** - Detailed configuration options

For production deployments, see [Deployment Modes](../user-guide/deployment-modes.md).