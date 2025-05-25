# DeepTrace Usage Guide

## Requirements
- Docker and Docker Compose
- Installation: `apt install docker.io docker-compose`

## Step 1: Fill in the Configuration File
- **Server-related configurations that must be filled in the configuration file:**
  - `elastic.address` # The IP address of the server running the Elastic database
  - `elastic.elastic_password` # Password for Elastic
  - `elastic.kibana_password` # Password for Kibana

- **Agent-related configurations that must be filled in the configuration file:**
  - `agents.agent_info.agent_name` # Name of the agent, which uniquely identifies each agent instance
  - `agents.agent_info.user_name` # Username for logging into the agent host (e.g., SSH username)
  - `agents.agent_info.host_ip` # IP address of the agent host
  - `agents.agent_info.ssh_port` # SSH port of the agent host (usually 22)
  - `agents.agent_info.host_password` # Password for logging into the agent host
  - `agents.sender.index_name` # Index name in Elasticsearch where the agent writes collected spans

## Step 2: Install Python Virtual Environment on Server
- `bash scripts/install_server_evn.sh`

## Step 3: Start Elastic Database on Server
- `bash scripts/install_es.sh`
- You can access the database frontend via the web at `http://ip:5601`
  - Username: `elastic`  
  - Password: `elastic_password`

## Step 4: Remotely Start Agent on Server
- `cd server ; source venv/bin/activate; cd controller`
- `python3 cmd.py first_start`
  - This command will automatically connect to the remote host, clone the code, compile, and run the agent. By default, it will automatically collect spans from all Docker containers and store them in the server's Elastic database.

## Step 5: Perform Span Correlation on Server

## Step 6: Perform Trace Assembly on Server

## Step 7: Clear Agent and Server
- `bash scripts/clear.sh`
  - This command will automatically stop all running agents and delete the database on the server.
