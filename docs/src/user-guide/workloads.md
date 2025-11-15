# Workload Applications

DeepTrace includes test microservice applications for demonstrating distributed tracing capabilities. These applications generate realistic network traffic patterns that help you understand how DeepTrace collects and correlates traces.

## Available Workloads

| Application | Complexity | Services | Use Case |
|-------------|------------|----------|----------|
| **BookInfo** | Simple | 4 services | Learning, basic testing |
| **Social Network** | Complex | 15+ services | Advanced testing, performance evaluation |

## BookInfo Application

### Overview

BookInfo is a simple microservices application that displays information about books. It consists of four services:

- **Product Page**: Frontend service that displays book information
- **Details Service**: Provides book details (author, ISBN, etc.)
- **Reviews Service**: Manages book reviews
- **Ratings Service**: Provides book ratings

### Quick Deployment

```bash
# Navigate to BookInfo directory
cd tests/workload/bookinfo

# Deploy all services
sudo bash deploy.sh
```

### Generate Traffic

```bash
# Generate test traffic to create traces
sudo bash client.sh
```

The client script will:
- Send HTTP requests to the product page
- Trigger inter-service communication
- Generate network traffic for DeepTrace to capture

### Cleanup

```bash
# Stop and remove all services
sudo bash clear.sh
```

## Social Network Application

### Overview

Social Network is a complex microservices application that implements a Twitter-like social media platform. It includes services for:

- User management and authentication
- Timeline and post management  
- Media handling and storage
- Social graph and recommendations
- Notification systems

### Quick Deployment

```bash
# Navigate to Social Network directory
cd tests/workload/socialnetwork

# Deploy the full application stack
bash deploy.sh
```

### Generate Traffic

```bash
# Generate realistic social media traffic
bash client.sh
```

The client generates:
- User registration and login requests
- Post creation and timeline updates
- Social interactions (likes, follows)
- Media uploads and downloads

### Cleanup

```bash
# Stop and remove all services
bash clear.sh
```

## Integration with DeepTrace

### Workflow with Workloads

1. **Deploy DeepTrace**: Follow the [Quick Start Guide](../getting-started/quick-start.md)
2. **Deploy workload**: Choose BookInfo or Social Network
3. **Start agent**: Begin collecting traces
4. **Generate traffic**: Run client scripts to create network activity
5. **Process traces**: Run correlation and assembly
6. **Analyze results**: View traces in Kibana

### Example Complete Workflow

```bash
# 1. Deploy workload application
cd tests/workload/bookinfo
sudo bash deploy.sh

# 2. Start DeepTrace agent
sudo docker exec -it deeptrace_server python -m cli.src.cmd agent run

# 3. Generate traffic
sudo bash client.sh

# 4. Process traces
sudo docker exec -it deeptrace_server python -m cli.src.cmd asso algo deeptrace
sudo docker exec -it deeptrace_server python -m cli.src.cmd assemble

# 5. View results in Kibana at http://YOUR_SERVER_IP:5601
```

## Additional Resources

For detailed deployment instructions and architecture information:

- **BookInfo**: [BookInfo README](https://github.com/DeepShield-AI/DeepTrace/blob/main/tests/workload/bookinfo/README.md)
- **Social Network**: [Social Network README](https://github.com/DeepShield-AI/DeepTrace/blob/main/tests/workload/socialnetwork/README.md)
- **Basic Usage**: [Basic Usage Guide](./basic-usage.md) for trace collection and analysis
