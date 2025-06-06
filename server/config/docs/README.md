# DeepTrace Configuration Guide

## Overview
This configuration file sets up Elasticsearch integration and defines parameters for the DeepTrace agent (`agent1`) to collect and send trace data using eBPF. Key components include:

1. **Elasticsearch settings** for data storage
2. **Agent configuration** for data collection and transmission
3. **API settings** for agent management

## Elasticsearch Configuration
| Parameter              | Default Value       | Description                              |
|------------------------|---------------------|------------------------------------------|
| `elastic_password`     | `""` (Empty)        | Password for Elasticsearch user          |
| `port`                 | `9200`              | Elasticsearch service port               |
| `address`              | `114.215.254.187`   | Elasticsearch server IP address          |
| `kibana_password`      | `""` (Empty)        | Kibana user password                     |
| `bulk_size`            | `1024`              | Bulk write size (in documents)           |
| `request_timeout`      | `10`                | Request timeout in seconds               |
| `agent_status_index`   | `"agent_status"`    | Index name for agent status monitoring   |

---

## Agent Configuration (`agent1`)
### Agent Information
| Parameter          | Default Value | Description                        |
|--------------------|---------------|------------------------------------|
| `agent_name`       | `"agent1"`    | Unique identifier for the agent    |
| `user_name`        | `"root"`      | SSH username for host access       |
| `host_ip`          | `""` (Empty)  | IP address of the monitored host   |
| `ssh_port`         | `22`          | SSH connection port               |
| `host_password`    | `""` (Empty)  | SSH password for host access       |
| `deeptrace_port`   | `52001`       | DeepTrace service port             |
| `code_path`        | `"/root"`     | Path to agent code on host         |
| `workers`          | `16`          | Number of worker threads           |

### Data Sender Settings
| Parameter           | Default Value        | Description                              |
|---------------------|----------------------|------------------------------------------|
| `index_name`        | `"spans_agent1"`     | Elasticsearch index for trace data       |
| `mem_buffer_size`   | `16` (MB)            | In-memory buffer size                    |
| `file_buffer_size`  | `32` (MB)            | File buffer size                         |
| `file_size_limit`   | `1024` (MB)          | Maximum file size for buffering          |
| `batch_size`        | `1024`               | Batch size for data transmission         |

### eBPF Trace Settings
| Parameter | Default Value         | Description                          |
|----------|-----------------------|--------------------------------------|
| `pids`   | `[3056354, 3056217, 3056210]` | Process IDs to trace with eBPF |

### API Service
| Parameter   | Default Value   | Description                        |
|-------------|-----------------|------------------------------------|
| `port`      | `7899`          | API service port                   |
| `address`   | `"0.0.0.0"`     | Network interface to bind to       |
| `workers`   | `1`             | API worker threads                 |
| `ident`     | `"deeptrace"`   | Service identifier                 |

---

## Critical Security Notes
1. **Passwords must be configured**:
   - Set `elastic_password` and `kibana_password` for secure Elasticsearch access
   - Provide `host_password` for SSH authentication
2. **Host IP requirement**:
   - `host_ip` cannot be empty - specify the target host's IP address
3. **Firewall rules**:
   - Ensure ports `9200` (ES), `7899` (API), and `52001` (DeepTrace) are accessible

> **Warning**: Never commit passwords to version control. Use environment variables or secret management tools for production deployments.
