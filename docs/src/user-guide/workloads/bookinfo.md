# BookInfo Application

BookInfo is a simple microservices application that displays information about books. It's perfect for learning distributed tracing concepts and testing DeepTrace functionality.

## Application Architecture

BookInfo consists of four microservices:

```
┌─────────────────┐    ┌─────────────────┐
│   Product Page  │◄──►│  Details Service│
│   (Frontend)    │    │                 │
└─────────┬───────┘    └─────────────────┘
          │
          ▼
┌─────────────────┐    ┌─────────────────┐
│ Reviews Service │◄──►│ Ratings Service │
│                 │    │                 │
└─────────────────┘    └─────────────────┘
```

### Services Overview

| Service | Description | Technology | Port |
|---------|-------------|------------|------|
| **Product Page** | Frontend service that displays book information | Python | 9080 |
| **Details Service** | Provides book details (author, ISBN, etc.) | Ruby | 9080 |
| **Reviews Service** | Manages book reviews | Java | 9080 |
| **Ratings Service** | Provides book ratings | Node.js | 9080 |

## Prerequisites

- Docker and Docker Compose installed
- DeepTrace server running
- Ports 9080 available

## Quick Deployment

### 1. Deploy BookInfo Services

Navigate to the BookInfo directory and deploy all services:

```bash
cd tests/workload/bookinfo
sudo bash deploy.sh
```

The deployment script will:
- Install Docker and Docker Compose (if needed)
- Pull required Docker images
- Launch all services using Docker Compose
- Set up service networking

### 2. Verify Deployment

Check that all services are running:

```bash
sudo docker ps | grep bookinfo
```

You should see containers for:
- `bookinfo-productpage`
- `bookinfo-details`
- `bookinfo-reviews`
- `bookinfo-ratings`

### 3. Access the Application

Open your browser and visit:
```
http://localhost:9080/productpage
```

You should see the BookInfo product page displaying book information.

## Generate Traffic for Tracing

### Automated Traffic Generation

Use the provided client script to generate test traffic:

```bash
sudo bash client.sh
```

This script will:
- Start an interactive shell inside the client container
- Issue frontend requests against the BookInfo application
- Generate HTTP traffic between microservices
- Create network traces for DeepTrace to capture

### Manual Traffic Generation

You can also generate traffic manually:

```bash
# Generate multiple requests
for i in {1..100}; do
  curl -s http://localhost:9080/productpage > /dev/null
  echo "Request $i completed"
  sleep 1
done
```

### Traffic Patterns

The BookInfo application generates the following traffic patterns:

1. **Frontend Requests**: User requests to product page
2. **Service-to-Service Calls**: 
   - Product Page → Details Service
   - Product Page → Reviews Service
   - Reviews Service → Ratings Service
3. **Database Queries**: Internal service data access

## Integration with DeepTrace

### Complete Workflow

1. **Deploy BookInfo**:
   ```bash
   cd tests/workload/bookinfo
   sudo bash deploy.sh
   ```

2. **Start DeepTrace Agent**:
   ```bash
   sudo docker exec -it deeptrace_server python -m cli.src.cmd agent run
   ```

3. **Generate Traffic**:
   ```bash
   sudo bash client.sh
   ```

4. **Process Traces**:
   ```bash
   sudo docker exec -it deeptrace_server python -m cli.src.cmd asso algo deeptrace
   sudo docker exec -it deeptrace_server python -m cli.src.cmd assemble
   ```

5. **View Results**: Access Kibana at `http://YOUR_SERVER_IP:5601`

### Expected Trace Data

When properly configured, you should see traces showing:
- HTTP requests between services
- Service response times
- Request flow through the microservice architecture
- Network connection details

## Troubleshooting

### Services Not Starting

```bash
# Check container logs
sudo docker logs bookinfo-productpage
sudo docker logs bookinfo-details
sudo docker logs bookinfo-reviews
sudo docker logs bookinfo-ratings

# Restart services
sudo docker-compose restart
```

### No Network Traffic Captured

1. Ensure DeepTrace agent is running
2. Verify services are generating traffic
3. Check that containers are on the same network
4. Confirm eBPF programs are loaded

### Port Conflicts

If port 9080 is already in use:

```bash
# Check what's using the port
sudo netstat -tulpn | grep 9080

# Stop conflicting services or modify docker-compose.yaml
```

## Cleanup

### Stop BookInfo Services

```bash
sudo bash clear.sh
```

This will:
- Stop all BookInfo containers
- Remove containers and networks
- Clean up Docker resources

### Complete Cleanup

```bash
# Remove all BookInfo images
sudo docker rmi $(sudo docker images | grep bookinfo | awk '{print $3}')

# Remove unused networks
sudo docker network prune -f
```

## Advanced Configuration

### Custom Configuration

You can modify the `docker-compose.yaml` file to:
- Change service ports
- Add environment variables
- Configure resource limits
- Enable additional logging

### Performance Testing

For performance testing with BookInfo:

```bash
# Install Apache Bench
sudo apt-get install apache2-utils

# Run load test
ab -n 1000 -c 10 http://localhost:9080/productpage
```

## Next Steps

- **[Social Network Application](./social-network.md)**: Try a more complex microservices application
- **[Trace Analysis](../trace-analysis.md)**: Learn how to analyze the collected traces
- **[Basic Usage](../basic-usage.md)**: Explore advanced DeepTrace operations
