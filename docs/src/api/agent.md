# Agent API Reference

The DeepTrace Agent provides a RESTful API for monitoring, configuration, and control operations. This API allows external tools and scripts to interact with the agent programmatically.

## Base URL

```
http://localhost:7899
```

## Authentication

Currently, the Agent API does not require authentication. In production environments, consider implementing network-level security controls.

## Endpoints

### Health and Status

#### GET /health

Returns the health status of the agent.

**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2024-01-15T10:30:00Z",
  "uptime": 3600,
  "version": "0.1.0"
}
```

#### GET /status

Returns detailed status information about the agent.

**Response:**
```json
{
  "agent": {
    "status": "running",
    "pid": 12345,
    "start_time": "2024-01-15T09:30:00Z",
    "config_file": "/etc/deeptrace/agent.toml"
  },
  "ebpf": {
    "programs_loaded": 8,
    "maps_active": 12,
    "kernel_version": "6.8.0"
  },
  "collection": {
    "spans_collected": 15420,
    "spans_per_second": 125.5,
    "last_span_time": "2024-01-15T10:29:58Z"
  },
  "resources": {
    "cpu_usage": 2.5,
    "memory_usage": 45.2,
    "disk_usage": 1.8
  }
}
```

### Process Management

#### GET /processes

Lists all monitored processes.

**Response:**
```json
{
  "processes": [
    {
      "pid": 1234,
      "name": "nginx",
      "command": "/usr/sbin/nginx -g daemon off;",
      "status": "monitored",
      "spans_collected": 1250
    },
    {
      "pid": 5678,
      "name": "redis-server",
      "command": "redis-server *:6379",
      "status": "monitored",
      "spans_collected": 890
    }
  ]
}
```

#### POST /processes

Add a process to monitoring.

**Request Body:**
```json
{
  "pid": 9012,
  "name": "app-server"
}
```

**Response:**
```json
{
  "success": true,
  "message": "Process 9012 added to monitoring",
  "process": {
    "pid": 9012,
    "name": "app-server",
    "status": "monitored"
  }
}
```

#### DELETE /processes/{pid}

Remove a process from monitoring.

**Response:**
```json
{
  "success": true,
  "message": "Process 9012 removed from monitoring"
}
```

### Span Collection

#### GET /spans

Retrieve collected spans with optional filtering.

**Query Parameters:**
- `limit` (int): Maximum number of spans to return (default: 100)
- `offset` (int): Number of spans to skip (default: 0)
- `service` (string): Filter by service name
- `operation` (string): Filter by operation name
- `start_time` (ISO 8601): Start time filter
- `end_time` (ISO 8601): End time filter

**Example:**
```
GET /spans?service=user-service&limit=50&start_time=2024-01-15T10:00:00Z
```

**Response:**
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
      }
    }
  ],
  "total": 1,
  "limit": 50,
  "offset": 0
}
```

#### GET /spans/stats

Get span collection statistics.

**Response:**
```json
{
  "total_spans": 15420,
  "spans_per_second": 125.5,
  "services": {
    "user-service": 5200,
    "order-service": 3800,
    "payment-service": 2100
  },
  "operations": {
    "GET /api/users": 2500,
    "POST /api/orders": 1800,
    "GET /api/health": 950
  },
  "time_range": {
    "start": "2024-01-15T09:30:00Z",
    "end": "2024-01-15T10:30:00Z"
  }
}
```

### Configuration

#### GET /config

Retrieve current agent configuration.

**Response:**
```json
{
  "agents": {
    "trace": {
      "batch_size": 1024,
      "flush_interval": 5000,
      "sampling_rate": 1.0
    },
    "capture": {
      "max_payload_size": 1024,
      "enable_compression": true
    },
    "sender": {
      "server_url": "http://localhost:7901",
      "timeout": 30000,
      "retry_count": 3
    }
  }
}
```

#### PUT /config

Update agent configuration.

**Request Body:**
```json
{
  "agents": {
    "trace": {
      "batch_size": 2048,
      "sampling_rate": 0.5
    }
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

#### POST /config/reload

Reload configuration from file.

**Response:**
```json
{
  "success": true,
  "message": "Configuration reloaded successfully",
  "config_file": "/etc/deeptrace/agent.toml"
}
```

### eBPF Management

#### GET /ebpf/programs

List loaded eBPF programs.

**Response:**
```json
{
  "programs": [
    {
      "id": 123,
      "name": "tcp_connect",
      "type": "kprobe",
      "attach_point": "tcp_connect",
      "loaded": true,
      "instructions": 245
    },
    {
      "id": 124,
      "name": "tcp_close",
      "type": "kprobe", 
      "attach_point": "tcp_close",
      "loaded": true,
      "instructions": 189
    }
  ]
}
```

#### GET /ebpf/maps

List active eBPF maps.

**Response:**
```json
{
  "maps": [
    {
      "id": 45,
      "name": "span_buffer",
      "type": "ringbuf",
      "key_size": 0,
      "value_size": 0,
      "max_entries": 262144,
      "current_entries": 1250
    },
    {
      "id": 46,
      "name": "process_map",
      "type": "hash",
      "key_size": 4,
      "value_size": 64,
      "max_entries": 10240,
      "current_entries": 45
    }
  ]
}
```

### Control Operations

#### POST /start

Start span collection.

**Response:**
```json
{
  "success": true,
  "message": "Span collection started"
}
```

#### POST /stop

Stop span collection.

**Response:**
```json
{
  "success": true,
  "message": "Span collection stopped"
}
```

#### POST /restart

Restart the agent.

**Response:**
```json
{
  "success": true,
  "message": "Agent restart initiated"
}
```

## Error Responses

All endpoints may return error responses in the following format:

```json
{
  "error": {
    "code": "INVALID_REQUEST",
    "message": "Invalid process ID provided",
    "details": {
      "field": "pid",
      "value": "invalid"
    }
  }
}
```

### Common Error Codes

- `INVALID_REQUEST`: Malformed request
- `NOT_FOUND`: Resource not found
- `INTERNAL_ERROR`: Internal server error
- `PERMISSION_DENIED`: Insufficient permissions
- `SERVICE_UNAVAILABLE`: Service temporarily unavailable

## Rate Limiting

The API implements basic rate limiting:
- 100 requests per minute per IP address
- Burst limit of 20 requests

## Examples

### Python Client Example

```python
import requests
import json

class DeepTraceAgentClient:
    def __init__(self, base_url="http://localhost:7899"):
        self.base_url = base_url
    
    def get_status(self):
        response = requests.get(f"{self.base_url}/status")
        return response.json()
    
    def get_spans(self, service=None, limit=100):
        params = {"limit": limit}
        if service:
            params["service"] = service
        
        response = requests.get(f"{self.base_url}/spans", params=params)
        return response.json()
    
    def add_process(self, pid, name):
        data = {"pid": pid, "name": name}
        response = requests.post(f"{self.base_url}/processes", json=data)
        return response.json()

# Usage
client = DeepTraceAgentClient()
status = client.get_status()
print(f"Agent status: {status['agent']['status']}")

spans = client.get_spans(service="user-service", limit=10)
print(f"Found {len(spans['spans'])} spans")
```

### Bash Script Example

```bash
#!/bin/bash

AGENT_URL="http://localhost:7899"

# Check agent health
curl -s "$AGENT_URL/health" | jq '.status'

# Get span statistics
curl -s "$AGENT_URL/spans/stats" | jq '.total_spans'

# Add process to monitoring
curl -X POST "$AGENT_URL/processes" \
  -H "Content-Type: application/json" \
  -d '{"pid": 12345, "name": "my-app"}'

# Get recent spans
curl -s "$AGENT_URL/spans?limit=5" | jq '.spans[].operation_name'
```
