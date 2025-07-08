# DeepTrace Usage Guide

## Step 1: Clone Code

- Clone the DeepTrace code on the controller of your cluster. Note that the code should not be placed under the `/etc` directory, because the agent will automatically clone the code to `/etc` later. If the server and agent are deployed on the same machine, a conflict will occur.

- Commands
 ```bash
 git clone https://github.com/DeepShield-AI/DeepTrace.git
 ```

## Step 2: Fill in the Configuration File

- **To deploy DeepTrace, you must fill in the following fields in the [configuration file](../../server/config/config.toml)(DeepTrace/server/config/config.toml) in order to run it.** These required fields are presented in the configuration file in the format of **xxx**.

| Configuration Item | Description |
| --- | --- |
| `server.ip` | The external IP address of the host running the DeepTrace server and the Elastic database |
| `elastic.elastic_password` | Password for Elastic |
| `agents.agent_info.agent_name` | Name of the agent, which uniquely identifies each agent instance |
| `agents.agent_info.user_name` | The username for logging into the host where the agent is located via SSH |
| `agents.agent_info.host_ip` | IP address of the agent host |
| `agents.agent_info.ssh_port` | SSH port of the agent host (usually 22) |
| `agents.agent_info.host_password` | The password for logging into the host where the agent is located via SSH |

- **In addition to the required fields, there are also some optional fields that will use default settings if you do not set them manually. A configuration file template that includes all the fields can be viewed under `DeepTrace/server/config/full.toml`. These parameters and their default settings are as follows:**

| Parameter | Description | Default Value |
|-----------|-------------|---------------|
| `elastic.port` | Elasticsearch service port | `9200` |
| `elastic.bulk_size` | Elasticsearch bulk write size | `1024` |
| `elastic.request_timeout` | Request timeout in seconds | `10` |
| `elastic.agent_status_index` | The name of the database table storing agent status | `agent_status` |
| `elastic.username` | Elasticsearch username | `elastic` |
| `server.port` | DeepTrace Server port | `7901` |
| `server.path` | DeepTrace Server path | `deeptrace/ws` |
| `deeptrace_port` | Deeptrace service port | `52001` |
| `agents.agent_info.workers` | Number of worker threads | `16` |
| `agents.span.batch_size` | Batch size for span transmission | `1024` |
| `agents.sender.mem_buffer_size` | Memory buffer size | `16` |
| `agents.sender.file_buffer_size` | File buffer size | `32` |
| `agents.sender.file_size_limit` | File size limit | `1024` |
| `agents.sender.batch_size` | Batch size for sending data | `1024` |
| `agents.trace.pids` | List of PIDs to monitor | `Default monitoring of all Docker container PIDs` |
| `agents.api.port` | API service port of agent | `7899` |
| `agents.api.address` | API service listening address | `0.0.0.0` |
| `agents.api.workers`  | Number of API service threads | `1` |
| `agents.api.ident` | Service identifier | `deeptrace` |

## Step 3: Deploy Server and Database Containers

- **Then, switch to the main directory of DeepTrace and run the following code on the server to deploy the DeepTrace server and the containers related to the database.**
- `bash scripts/deploy_server.sh`
- You can access the database frontend via the web at `http://server.ip:5601`
  - Username: `elastic`  
  - Password: `elastic.elastic_password`


## Step 3: Start Agents
- **After that, enter the DeepTrace server container and use the command-line tool of the server to install and start the agent.**

```bash
sudo docker exec -it deeptrace_server /bin/bash # Enter the server container, the following commands are run within the deeptrace_server container
python3 -m cli.src.cmd agent install # This command will automatically connect to the remote host, clone the code, and compile. 
python3 -m cli.src.cmd agent run # Run the agent, by default it will automatically collect spans from all Docker containers and store them in the server's Elastic database. 
```


## Step 4: Build Traces

### After entering the DeepTrace server container, run the following command to perform span correlation and trace assembly. DeepTrace has two modes: *manual* and *automatic*.

- `automatic` mode. 

```bash
sudo docker exec -it deeptrace_server python -m trace.main
```

- `manual` mode.

`<algorithm>`: Choose from `fifo`, `deeptrace`, `vpath`, `wap5`, `traceweaver_v1`, `deepflow` to infer parent-child relationships between spans.

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


Exit the container and then execute:

```bash
bash scripts/clear_server.sh
```

- This will remove all containers on the server.


## Optional: Deploy docker swarm clusters and microservices applications on hosts

- You must have a microservice application to observe the Trace built by DeepTrace. For the convenience of testing, we have provided a script that automates the deployment of a Docker Swarm distributed cluster and installs the SocialNetwork microservice application from Deathstarbench. If you do not have a microservice application in your cluster, you can follow the steps below to install it.

- After filling out the configuration file (**Step1**) and starting the DeepTrace server (**Step2**), you can run the following command to install the Swarm cluster and the SocialNetwork microservice.

```bash
sudo docker exec -it deeptrace_server python -m cli.src.cmd agent install_app
```

- Send requests to the application.
  - Traffic must be generated on the host where the agent is located to produce spans. You can look up the container id of the wrk2 container via `docker ps | grep wrk2`, then enter the container via `docker exec -it container_id /bin/bash` and run the command `cd root; ./wrk -D exp -t 6 -c 6 -d 3 -L -s ./wrk2/scripts/social-network/compose-post.lua http://nginx-web-server:8080/wrk2-api/post/compose -R 100` to send the package afterward.  

- Clean up the microservices application and container swarm clusters.

```bash
sudo docker exec -it deeptrace_server python -m cli.src.cmd agent uninstall_app
```
