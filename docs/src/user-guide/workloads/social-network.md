# Social Network Application

Social Network is a complex microservices application that implements a Twitter-like social media platform. It's ideal for testing DeepTrace with realistic, large-scale distributed systems.

## Application Architecture

The Social Network application consists of 15+ microservices communicating via Thrift RPCs:

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Nginx Web     │◄──►│  User Service   │◄──►│ User Timeline   │
│    Server       │    │                 │    │   Service       │
└─────────┬───────┘    └─────────────────┘    └─────────────────┘
          │
          ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│ Compose Post    │◄──►│ Post Storage    │◄──►│ Home Timeline   │
│   Service       │    │   Service       │    │   Service       │
└─────────────────┘    └─────────────────┘    └─────────────────┘
          │
          ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│ Social Graph    │◄──►│ URL Shortener   │◄──►│ Media Service   │
│   Service       │    │   Service       │    │                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Core Services

| Service | Description | Technology | Port |
|---------|-------------|------------|------|
| **Nginx Web Server** | Frontend proxy and web interface | Nginx | 8080 |
| **User Service** | User management and authentication | C++ | 9090 |
| **Compose Post Service** | Post creation and processing | C++ | 9090 |
| **Post Storage Service** | Post data persistence | C++ | 9090 |
| **User Timeline Service** | Individual user timelines | C++ | 9090 |
| **Home Timeline Service** | Aggregated home timelines | C++ | 9090 |
| **Social Graph Service** | Follow relationships | C++ | 9090 |
| **URL Shortener Service** | URL shortening functionality | C++ | 9090 |
| **Media Service** | Image and video handling | C++ | 9090 |
| **Text Service** | Text processing and filtering | C++ | 9090 |
| **Unique ID Service** | Unique identifier generation | C++ | 9090 |

### Supporting Infrastructure

| Component | Description | Port |
|-----------|-------------|------|
| **MongoDB** | Primary database | 27017 |
| **Redis** | Caching layer | 6379 |
| **Memcached** | Additional caching | 11211 |
| **Jaeger** | Distributed tracing (optional) | 16686 |

## Prerequisites

- Docker and Docker Compose installed
- DeepTrace server running
- At least 8GB RAM available
- Ports 8080, 8081, 16686 available
- Python 3.5+ with asyncio and aiohttp
- Build dependencies: libssl-dev, libz-dev, luarocks, luasocket

## Quick Deployment

### 1. Deploy Social Network Services

Navigate to the Social Network directory and deploy:

```bash
cd tests/workload/socialnetwork
sudo bash deploy.sh
```

The deployment script will:
- Install required dependencies
- Build Docker images
- Start all microservices
- Initialize databases
- Set up networking

### 2. Verify Deployment

Check that all services are running:

```bash
sudo docker ps | grep social
```

You should see containers for all microservices, databases, and supporting infrastructure.

### 3. Access the Application

#### Web Interface
Open your browser and visit:
```
http://localhost:8080
```

#### Media Frontend
Access the media interface at:
```
http://localhost:8081
```

#### Jaeger Tracing (if enabled)
View built-in traces at:
```
http://localhost:16686
```

## Initialize Social Graph

Before generating traffic, initialize the social graph with users and relationships:

```bash
# Initialize with small Reed98 Facebook network
python3 scripts/init_social_graph.py --graph=socfb-Reed98

# Initialize with medium Ego Twitter network
python3 scripts/init_social_graph.py --graph=ego-twitter

# Initialize with large Twitter follows network
python3 scripts/init_social_graph.py --graph=soc-twitter-follows-mun
```

For remote deployments, specify IP and port:
```bash
python3 scripts/init_social_graph.py --graph=socfb-Reed98 --ip=YOUR_SERVER_IP --port=8080
```

## Generate Traffic for Tracing

### Automated Traffic Generation

Use the provided client script:

```bash
sudo bash client.sh
```

### Manual Workload Generation

The Social Network application supports various workload patterns:

#### 1. Compose Posts

```bash
../wrk2/wrk -D exp -t 12 -c 400 -d 300 -L \
  -s ./wrk2/scripts/social-network/compose-post.lua \
  http://localhost:8080/wrk2-api/post/compose -R 10
```

#### 2. Read Home Timelines

```bash
../wrk2/wrk -D exp -t 12 -c 400 -d 300 -L \
  -s ./wrk2/scripts/social-network/read-home-timeline.lua \
  http://localhost:8080/wrk2-api/home-timeline/read -R 10
```

#### 3. Read User Timelines

```bash
../wrk2/wrk -D exp -t 12 -c 400 -d 300 -L \
  -s ./wrk2/scripts/social-network/read-user-timeline.lua \
  http://localhost:8080/wrk2-api/user-timeline/read -R 10
```

### Traffic Patterns

The Social Network application generates complex traffic patterns:

1. **User Authentication**: Login/logout requests
2. **Post Operations**: Create, read, update posts
3. **Timeline Operations**: Home and user timeline requests
4. **Social Operations**: Follow/unfollow, recommendations
5. **Media Operations**: Image/video upload and retrieval
6. **Database Operations**: MongoDB and Redis queries
7. **Cache Operations**: Memcached read/write operations

## Integration with DeepTrace

### Complete Workflow

1. **Deploy Social Network**:
   ```bash
   cd tests/workload/socialnetwork
   sudo bash deploy.sh
   ```

2. **Initialize Social Graph**:
   ```bash
   python3 scripts/init_social_graph.py --graph=socfb-Reed98
   ```

3. **Start DeepTrace Agent**:
   ```bash
   sudo docker exec -it deeptrace_server python -m cli.src.cmd agent run
   ```

4. **Generate Traffic**:
   ```bash
   sudo bash client.sh
   ```

5. **Process Traces**:
   ```bash
   sudo docker exec -it deeptrace_server python -m cli.src.cmd asso algo deeptrace
   sudo docker exec -it deeptrace_server python -m cli.src.cmd assemble
   ```

6. **View Results**: Access Kibana at `http://YOUR_SERVER_IP:5601`

### Expected Trace Data

With Social Network, you should see rich trace data including:
- Complex service-to-service communication patterns
- Database query traces (MongoDB, Redis, Memcached)
- HTTP and Thrift RPC calls
- Media upload/download operations
- Authentication and authorization flows
- Caching layer interactions

## Advanced Features

### Enable TLS

For TLS-enabled deployment:

```bash
sudo docker-compose -f docker-compose-tls.yml up -d
```

### Enable Redis Sharding

For cache and database sharding:

```bash
sudo docker-compose -f docker-compose-sharding.yml up -d
```

### Docker Swarm Deployment

For multi-node deployment:

```bash
docker stack deploy --compose-file=docker-compose-swarm.yml social-network
```

## Performance Testing

### Load Testing with wrk2

First, build the workload generator:

```bash
cd ../wrk2
make
cd ../socialNetwork
```

Then run various load tests:

```bash
# High-throughput compose posts
../wrk2/wrk -D exp -t 20 -c 800 -d 600 -L \
  -s ./wrk2/scripts/social-network/compose-post.lua \
  http://localhost:8080/wrk2-api/post/compose -R 100

# Mixed workload
../wrk2/wrk -D exp -t 16 -c 600 -d 300 -L \
  -s ./wrk2/scripts/social-network/mixed-workload.lua \
  http://localhost:8080 -R 50
```

## Troubleshooting

### Services Not Starting

```bash
# Check individual service logs
sudo docker logs social-network-nginx-thrift
sudo docker logs social-network-user-service
sudo docker logs social-network-compose-post-service

# Check database connectivity
sudo docker logs social-network-mongodb
sudo docker logs social-network-redis
```

### Database Connection Issues

```bash
# Restart databases
sudo docker restart social-network-mongodb social-network-redis

# Check database status
sudo docker exec social-network-mongodb mongo --eval "db.stats()"
sudo docker exec social-network-redis redis-cli ping
```

### Memory Issues

The Social Network application is resource-intensive:

```bash
# Monitor resource usage
sudo docker stats

# Increase Docker memory limits if needed
# Edit docker-compose.yml and add memory limits
```

### No Network Traffic Captured

1. Ensure all services are fully started (can take 2-3 minutes)
2. Verify social graph initialization completed
3. Check that DeepTrace agent is monitoring the correct containers
4. Confirm eBPF programs are loaded for all relevant processes

## Cleanup

### Stop Social Network Services

```bash
sudo bash clear.sh
```

### Complete Cleanup

```bash
# Remove all Social Network images
sudo docker rmi $(sudo docker images | grep social | awk '{print $3}')

# Clean up volumes
sudo docker volume prune -f

# Remove networks
sudo docker network prune -f
```

## Monitoring and Observability

### Built-in Jaeger Tracing

The Social Network application includes Jaeger tracing:

```bash
# Access Jaeger UI
http://localhost:16686
```

### Custom Metrics

Monitor application metrics:

```bash
# Service health endpoints
curl http://localhost:8080/health
curl http://localhost:8081/health

# Database metrics
sudo docker exec social-network-mongodb mongo --eval "db.serverStatus()"
sudo docker exec social-network-redis redis-cli info
```

## Development and Customization

### Modifying Services

The Social Network application is actively developed. You can:

1. Modify service configurations in `config/`
2. Customize workload scripts in `wrk2/scripts/`
3. Adjust Docker Compose configurations
4. Add custom monitoring and logging

### Building from Source

```bash
# Build custom images
sudo docker-compose build

# Build specific services
sudo docker-compose build user-service
sudo docker-compose build compose-post-service
```

## Next Steps

- **[BookInfo Application](./bookinfo.md)**: Try a simpler microservices application
- **[Trace Analysis](../trace-analysis.md)**: Learn advanced trace analysis techniques
- **[Basic Usage](../basic-usage.md)**: Explore DeepTrace operations
- **[Database Management](../database.md)**: Configure advanced Elasticsearch setups
