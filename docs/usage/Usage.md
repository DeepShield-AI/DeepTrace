# DeepTrace Usage Guide

## Requirements
- Docker and Docker Compose
- Installation: `apt install docker.io docker-compose`

## Step 1: Fill in the Configuration File
- **Server-related configurations that must be filled in the [configuration file](../../server/config/config.toml):**
  - `server.ip` # The IP address of the server running the Elastic database
  - `elastic.elastic_password` # Password for Elastic
  - `elastic.kibana_password` # Password for Kibana

- **Agent-related configurations that must be filled in the configuration file:**
  - `agents.agent_info.agent_name` # Name of the agent, which uniquely identifies each agent instance
  - `agents.agent_info.user_name` # Username for logging into the agent host (e.g., SSH username)
  - `agents.agent_info.host_ip` # IP address of the agent host
  - `agents.agent_info.ssh_port` # SSH port of the agent host (usually 22)
  - `agents.agent_info.host_password` # Password for logging into the agent host
  - `agents.sender.index_name` # Index name in Elasticsearch where the agent writes collected spans


## Step 2: Deploy Server and Database Containers
- `bash scripts/deploy_server.sh`
- You can access the database frontend via the web at `http://ip:5601`
  - Username: `elastic`  
  - Password: `elastic_password`

## Step 3: Start Agents
- Commands

```
docker exec -it deeptrace_server /bin/bash # Enter the server container, the following commands are run within the deeptrace_server container
python3 -m cli.src.cmd agent install # This command will automatically connect to the remote host, clone the code, and compile. 
python3 -m cli.src.cmd agent run # Run the agent, by default it will automatically collect spans from all Docker containers and store them in the server's Elastic database. 
```

**Note**: Traffic must be generated on the host where the agent is located to produce spans.

## Step 4: Build Traces

```
docker exec -it deeptrace_server /bin/bash 
python -m cli.src.cmd asso algo <algorithm>
```

- `<algorithm>`: Choose from `fifo`, `deeptrace`, `vpath`, `wap5`, `traceweaver_v1`, `deepflow` to infer parent-child relationships between spans.

```
python -m cli.src.cmd assemble
```

- Assemble spans from the database into traces.

## Step 5: Clear Agents and Server
Stop all running agents:

```
docker exec -it deeptrace_server /bin/bash
python -m cli.src.cmd agent stop
```

Exit the container and then execute:

```
bash scripts/clear_server.sh
```

- This will remove all containers on the server.

