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

## Optional: Deploy docker swarm clusters and microservices applications on hosts
```bash
sudo docker exec -it deeptrace_server python -m cli.src.cmd agent install_app
```

## Step 3: Start Agents
- Commands

```bash
sudo docker exec -it deeptrace_server /bin/bash # Enter the server container, the following commands are run within the deeptrace_server container
python3 -m cli.src.cmd agent install # This command will automatically connect to the remote host, clone the code, and compile. 
python3 -m cli.src.cmd agent run # Run the agent, by default it will automatically collect spans from all Docker containers and store them in the server's Elastic database. 
```

**Note**: Traffic must be generated on the host where the agent is located to produce spans. You can look up the container id of the wrk2 container via `docker ps | grep wrk2`, then enter the container via `docker exec -it container_id /bin/bash` and run the command `cd root; ./wrk -D exp -t 6 -c 6 -d 3 -L -s ./wrk2/scripts/social-network/compose-post.lua http://nginx-web-server:8080/wrk2-api/post/compose -R 100` to send the package afterward.

## Step 4: Build Traces

### `automatic` mode. 

```bash
sudo docker exec -it deeptrace_server python -m trace.main
```

### `manual` mode.

- `<algorithm>`: Choose from `fifo`, `deeptrace`, `vpath`, `wap5`, `traceweaver_v1`, `deepflow` to infer parent-child relationships between spans.

```bash
sudo docker exec -it deeptrace_server /bin/bash 
python -m cli.src.cmd asso algo <algorithm>
```

- Assemble spans from the database into traces.

```bash
python -m cli.src.cmd assemble
```


## Step 5: Clear Agents and Server
Stop all running agents:

```bash
sudo docker exec -it deeptrace_server python -m cli.src.cmd agent stop
```

Clean up the microservices application and container swarm clusters.

```bash
sudo docker exec -it deeptrace_server python -m cli.src.cmd agent uninstall_app
```

Exit the container and then execute:

```bash
bash scripts/clear_server.sh
```

- This will remove all containers on the server.

