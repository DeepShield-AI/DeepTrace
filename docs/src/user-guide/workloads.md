# Workload Applications

DeepTrace includes test microservices applications for demonstrating distributed tracing capabilities.

## Available Workloads

- **BookInfo**: Simple microservices application
- **Social Network**: Complex microservices application

## BookInfo Microservices

### Overview

BookInfo is a simple microservices application with four services:
- Product Page (frontend)
- Details Service  
- Reviews Service
- Ratings Service

### Deployment

#### Deployment

```bash
# Navigate to BookInfo directory
cd tests/workload/bookinfo

# Deploy all services
sudo bash deploy.sh
```

### Generate Traffic

```bash
# Generate test traffic
sudo bash client.sh
```

### Cleanup

```bash
# Stop and remove all services
sudo bash clear.sh
```

## Social Network Microservices

### Overview

Social Network is a complex microservices application that implements a Twitter-like social media platform with multiple interconnected services.

### Deployment

```bash
# Navigate to Social Network directory
cd tests/workload/socialnetwork

# Deploy full stack
bash deploy.sh
```

### Generate Traffic

```bash
# Generate test traffic
bash client.sh
```

### Cleanup

```bash
# Stop and remove all services
bash clear.sh
```

## Additional Resources

For detailed deployment instructions, refer to:
- BookInfo: `tests/workload/bookinfo/README.md`
- Social Network: `tests/workload/socialnetwork/README.md`
