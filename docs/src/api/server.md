# Server API Reference

The DeepTrace Server provides a comprehensive API for trace correlation, analysis, and management. This API serves as the central hub for processing spans collected by agents and providing analytical insights.

## Base URL

```
http://localhost:7901
```

## Authentication

The Server API supports optional authentication via API keys. Configure authentication in the server configuration file.

```toml
[api]
enable_auth = true
api_keys = ["your-api-key-here"]
```

When authentication is enabled, include the API key in requests:

```
Authorization: Bearer your-api-key-here
```

## Endpoints

### Health and Status

#### GET /health

Returns the health status of the server.

**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2024-01-15T10:30:00Z",
  "uptime": 7200,
  "version": "0.1.0",
  "components": {
    "elasticsearch": "healthy",
    "correlation_engine": "healthy",
    "api_server": "healthy"
  }
}
```

#### GET /status

Returns detailed status information about the server.

**Response:**
```json
{
  "server": {
    "status": "running",
    "pid": 23456,
    "start_time": "2024-01-15T08:30:00Z",
    "config_file": "/etc/deeptrace/server.toml"
  },
  "elasticsearch": {
    "status": "connected",
    "cluster_name": "deeptrace",
    "nodes": 1,
    "indices": 5,
    "documents": 125420
  },
  "correlation": {
    "algorithm": "deeptrace",
    "spans_processed": 98750,
    "traces_generated": 12340,
    "correlation_rate": 89.5
  },
  "performance": {
    "cpu_usage": 15.2,
    "memory_usage": 256.8,
    "disk_usage": 12.4,
    "network_io": 45.6
  }
}
```

### Span Management

#### POST /spans

Receive spans from agents (internal endpoint).

**Request Body:**
```json
{
  "spans": [
    {
      "trace_id": "abc123def456",
      "span_id": "span789",
      "parent_span_id": "parent456",
      "service_name": "user-service",
      "operation_name": "GET /api/users",
      "start_time": "2024-01-15T10:15:30.123Z",
      "end_time": "2024-01-15T10:15:30.456Z",
      "duration": 333,
      "tags": {
        "http.method": "GET",
        "http.url": "/api/users",
        "http.status_code": 200
      },
      "process": {
        "pid": 1234,
        "hostname": "web-server-01"
      }
    }
  ]
}
```

**Response:**
```json
{
  "success": true,
  "spans_received": 1,
  "spans_processed": 1,
  "processing_time": 15
}
```

#### GET /spans/search

Search for spans with advanced filtering.

**Query Parameters:**
- `q` (string): Full-text search query
- `service` (string): Filter by service name
- `operation` (string): Filter by operation name
- `trace_id` (string): Filter by trace ID
- `start_time` (ISO 8601): Start time filter
- `end_time` (ISO 8601): End time filter
- `min_duration` (int): Minimum duration in microseconds
- `max_duration` (int): Maximum duration in microseconds
- `tags` (string): Tag filters (format: key:value)
- `limit` (int): Maximum results (default: 100, max: 1000)
- `offset` (int): Pagination offset

**Example:**
```
GET /spans/search?service=user-service&min_duration=1000&tags=http.status_code:500&limit=50
```

**Response:**
```json
{
  "spans": [
    {
      "trace_id": "abc123def456",
      "span_id": "span789",
      "service_name": "user-service",
      "operation_name": "GET /api/users",
      "start_time": "2024-01-15T10:15:30.123Z",
      "duration": 1500,
      "tags": {
        "http.method": "GET",
        "http.status_code": 500,
        "error": true
      }
    }
  ],
  "total": 1,
  "limit": 50,
  "offset": 0,
  "query_time": 25
}
```

### Trace Management

#### GET /traces

Retrieve assembled traces.

**Query Parameters:**
- `service` (string): Filter by service name
- `operation` (string): Filter by root operation
- `start_time` (ISO 8601): Start time filter
- `end_time` (ISO 8601): End time filter
- `min_duration` (int): Minimum trace duration
- `max_duration` (int): Maximum trace duration
- `min_spans` (int): Minimum number of spans
- `max_spans` (int): Maximum number of spans
- `has_errors` (bool): Filter traces with errors
- `limit` (int): Maximum results (default: 50)
- `offset` (int): Pagination offset

**Response:**
```json
{
  "traces": [
    {
      "trace_id": "abc123def456",
      "root_span": {
        "span_id": "root123",
        "service_name": "api-gateway",
        "operation_name": "GET /api/users"
      },
      "start_time": "2024-01-15T10:15:30.123Z",
      "end_time": "2024-01-15T10:15:31.456Z",
      "duration": 1333,
      "span_count": 8,
      "service_count": 4,
      "has_errors": false,
      "services": ["api-gateway", "user-service", "database", "cache"]
    }
  ],
  "total": 1,
  "limit": 50,
  "offset": 0
}
```

#### GET /traces/{trace_id}

Get detailed information about a specific trace.

**Response:**
```json
{
  "trace_id": "abc123def456",
  "spans": [
    {
      "span_id": "root123",
      "parent_span_id": null,
      "service_name": "api-gateway",
      "operation_name": "GET /api/users",
      "start_time": "2024-01-15T10:15:30.123Z",
      "end_time": "2024-01-15T10:15:31.456Z",
      "duration": 1333,
      "tags": {
        "http.method": "GET",
        "http.url": "/api/users",
        "http.status_code": 200
      },
      "children": ["span456", "span789"]
    },
    {
      "span_id": "span456",
      "parent_span_id": "root123",
      "service_name": "user-service",
      "operation_name": "query_users",
      "start_time": "2024-01-15T10:15:30.200Z",
      "end_time": "2024-01-15T10:15:30.800Z",
      "duration": 600,
      "tags": {
        "db.statement": "SELECT * FROM users",
        "db.type": "postgresql"
      },
      "children": []
    }
  ],
  "metadata": {
    "total_spans": 8,
    "total_duration": 1333,
    "service_count": 4,
    "error_count": 0
  }
}
```

### Correlation Management

#### POST /correlation/run

Trigger span correlation process.

**Request Body:**
```json
{
  "algorithm": "deeptrace",
  "parameters": {
    "window_size": 1000,
    "similarity_threshold": 0.8,
    "max_iterations": 100
  },
  "time_range": {
    "start": "2024-01-15T10:00:00Z",
    "end": "2024-01-15T11:00:00Z"
  }
}
```

**Response:**
```json
{
  "job_id": "corr_job_123",
  "status": "started",
  "algorithm": "deeptrace",
  "estimated_duration": 120,
  "spans_to_process": 15420
}
```

#### GET /correlation/jobs/{job_id}

Get correlation job status.

**Response:**
```json
{
  "job_id": "corr_job_123",
  "status": "completed",
  "algorithm": "deeptrace",
  "start_time": "2024-01-15T10:30:00Z",
  "end_time": "2024-01-15T10:32:15Z",
  "duration": 135,
  "results": {
    "spans_processed": 15420,
    "traces_generated": 1890,
    "correlation_rate": 92.3,
    "errors": 0
  }
}
```

#### GET /correlation/algorithms

List available correlation algorithms.

**Response:**
```json
{
  "algorithms": [
    {
      "name": "deeptrace",
      "description": "Advanced transaction-based correlation",
      "parameters": {
        "window_size": {
          "type": "integer",
          "default": 1000,
          "description": "Correlation window in milliseconds"
        },
        "similarity_threshold": {
          "type": "float",
          "default": 0.8,
          "description": "Minimum similarity score"
        }
      }
    },
    {
      "name": "fifo",
      "description": "Simple first-in-first-out correlation",
      "parameters": {
        "batch_size": {
          "type": "integer",
          "default": 1000,
          "description": "Processing batch size"
        }
      }
    }
  ]
}
```

### Analytics

#### GET /analytics/services

Get service-level analytics.

**Query Parameters:**
- `start_time` (ISO 8601): Start time for analysis
- `end_time` (ISO 8601): End time for analysis
- `service` (string): Filter by specific service

**Response:**
```json
{
  "services": [
    {
      "name": "user-service",
      "request_count": 5420,
      "error_count": 12,
      "error_rate": 0.22,
      "avg_duration": 245.6,
      "p95_duration": 890.2,
      "p99_duration": 1250.8,
      "throughput": 90.3
    },
    {
      "name": "order-service",
      "request_count": 3200,
      "error_count": 8,
      "error_rate": 0.25,
      "avg_duration": 156.3,
      "p95_duration": 450.1,
      "p99_duration": 780.5,
      "throughput": 53.3
    }
  ],
  "time_range": {
    "start": "2024-01-15T10:00:00Z",
    "end": "2024-01-15T11:00:00Z"
  }
}
```

#### GET /analytics/operations

Get operation-level analytics.

**Response:**
```json
{
  "operations": [
    {
      "service": "user-service",
      "operation": "GET /api/users",
      "request_count": 2100,
      "error_count": 5,
      "error_rate": 0.24,
      "avg_duration": 189.4,
      "p95_duration": 567.8,
      "p99_duration": 890.2
    }
  ]
}
```

#### GET /analytics/topology

Get service dependency topology.

**Response:**
```json
{
  "nodes": [
    {
      "id": "api-gateway",
      "name": "API Gateway",
      "type": "service",
      "request_count": 8500,
      "error_rate": 0.15
    },
    {
      "id": "user-service",
      "name": "User Service",
      "type": "service",
      "request_count": 5420,
      "error_rate": 0.22
    }
  ],
  "edges": [
    {
      "source": "api-gateway",
      "target": "user-service",
      "request_count": 5420,
      "avg_duration": 245.6,
      "error_rate": 0.22
    }
  ]
}
```

### Configuration

#### GET /config

Get current server configuration.

**Response:**
```json
{
  "server": {
    "host": "0.0.0.0",
    "port": 7901,
    "workers": 4
  },
  "elasticsearch": {
    "hosts": ["http://localhost:9200"],
    "index_prefix": "deeptrace",
    "batch_size": 1000
  },
  "correlation": {
    "default_algorithm": "deeptrace",
    "auto_correlation": true,
    "correlation_interval": 60
  }
}
```

#### PUT /config

Update server configuration.

**Request Body:**
```json
{
  "correlation": {
    "default_algorithm": "fifo",
    "correlation_interval": 30
  }
}
```

**Response:**
```json
{
  "success": true,
  "message": "Configuration updated successfully",
  "restart_required": false
}
```

## WebSocket API

### Real-time Span Streaming

Connect to receive real-time span updates:

```
ws://localhost:7901/ws/spans
```

**Message Format:**
```json
{
  "type": "span",
  "data": {
    "trace_id": "abc123def456",
    "span_id": "span789",
    "service_name": "user-service",
    "operation_name": "GET /api/users",
    "timestamp": "2024-01-15T10:15:30.123Z"
  }
}
```

### Real-time Trace Updates

Connect to receive trace completion notifications:

```
ws://localhost:7901/ws/traces
```

**Message Format:**
```json
{
  "type": "trace_completed",
  "data": {
    "trace_id": "abc123def456",
    "span_count": 8,
    "duration": 1333,
    "services": ["api-gateway", "user-service", "database"]
  }
}
```

## Error Responses

Standard error response format:

```json
{
  "error": {
    "code": "CORRELATION_FAILED",
    "message": "Correlation algorithm failed to process spans",
    "details": {
      "algorithm": "deeptrace",
      "spans_processed": 1250,
      "error_details": "Insufficient memory for correlation matrix"
    }
  }
}
```

## Examples

### Python Client Example

```python
import requests
import json
from datetime import datetime, timedelta

class DeepTraceServerClient:
    def __init__(self, base_url="http://localhost:7901", api_key=None):
        self.base_url = base_url
        self.headers = {}
        if api_key:
            self.headers["Authorization"] = f"Bearer {api_key}"
    
    def search_spans(self, service=None, operation=None, limit=100):
        params = {"limit": limit}
        if service:
            params["service"] = service
        if operation:
            params["operation"] = operation
        
        response = requests.get(
            f"{self.base_url}/spans/search",
            params=params,
            headers=self.headers
        )
        return response.json()
    
    def get_trace(self, trace_id):
        response = requests.get(
            f"{self.base_url}/traces/{trace_id}",
            headers=self.headers
        )
        return response.json()
    
    def run_correlation(self, algorithm="deeptrace"):
        data = {
            "algorithm": algorithm,
            "time_range": {
                "start": (datetime.now() - timedelta(hours=1)).isoformat() + "Z",
                "end": datetime.now().isoformat() + "Z"
            }
        }
        response = requests.post(
            f"{self.base_url}/correlation/run",
            json=data,
            headers=self.headers
        )
        return response.json()
    
    def get_service_analytics(self):
        response = requests.get(
            f"{self.base_url}/analytics/services",
            headers=self.headers
        )
        return response.json()

# Usage
client = DeepTraceServerClient(api_key="your-api-key")

# Search for error spans
error_spans = client.search_spans(tags="error:true", limit=10)
print(f"Found {len(error_spans['spans'])} error spans")

# Get service analytics
analytics = client.get_service_analytics()
for service in analytics['services']:
    print(f"{service['name']}: {service['error_rate']:.2%} error rate")

# Run correlation
job = client.run_correlation()
print(f"Correlation job started: {job['job_id']}")
```
