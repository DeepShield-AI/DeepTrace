# DeepTrace All-in-One Mode Usage Guide

## Step 1: Clone the Code

In **all-in-one** mode, the DeepTrace **server** and **agent** run on the **same host**.  
Clone the repository to any directory **except `/etc`**—the agent will automatically populate `/etc` with its own files later.

```bash
git clone https://github.com/DeepShield-AI/DeepTrace.git
```

## Step 2: Fill in the Configuration File

- **To deploy DeepTrace, you must fill in the following fields in the [configuration file](../../server/config/config.toml)(DeepTrace/server/config/config.toml) in order to run it.** These required fields are presented in the configuration file in the format of **xxx**. In all-in-one mode, the `server.ip` and `agents.agent_info.host_ip` values are identical.  

| Configuration Item | Description |
| --- | --- |
| `server.ip` | The external IP address of the host running the DeepTrace server and the Elastic database |
| `elastic.elastic_password` | Password for Elastic |
| `agents.agent_info.agent_name` | Name of the agent, which uniquely identifies each agent instance |
| `agents.agent_info.user_name` | The username for logging into the host where the agent is located via SSH |
| `agents.agent_info.host_ip` | IP address of the agent host |
| `agents.agent_info.ssh_port` | SSH port of the agent host (usually 22) |
| `agents.agent_info.host_password` | The password for logging into the host where the agent is located via SSH |



## Step 3: Deploy Server

- Change into the DeepTrace root directory and run:
  ```bash
  bash scripts/deploy_server.sh
  ```
- Access the web UI at `http://<server.ip>:8000`.

## Step 4: Deploy a Microservice Application

To generate traces, deploy a microservice application on the host:

- Bookinfo — see [documentation 1](../../tests/workload/bookinfo/README.md)  
- Social Network — see [documentation 2](../../tests/workload/socialnetwork/README.md)


## Step 5: Deploy Agents

- **After that, use the command-line tool of the server to install and start the agent**

```bash
sudo docker exec -it deeptrace_server python -m cli.src.cmd agent install # This command will automatically connect to the remote host, clone the code, and compile. 
sudo docker exec -it deeptrace_server python -m cli.src.cmd agent run # Run the agent, by default it will automatically collect spans from all Docker containers and store them in the server's Elastic database.  
```


## Step 6: Build Traces

After the agent starts, send requests to the microservice application.  
Refer to the following documents for sending requests:

- Bookinfo — see [documentation 1](../../tests/workload/bookinfo/README.md)  
- Social Network — see [documentation 2](../../tests/workload/socialnetwork/README.md)

Once requests are complete, build traces with:

```bash
sudo docker exec -it deeptrace_server python -m cli.src.cmd asso algo deeptrace # Perform span correlation
sudo docker exec -it deeptrace_server python -m cli.src.cmd assemble # Perform trace assembling
```


## Step 7: Clear Agents and Server

The following command will delete the installed DeepTrace agent and server:

```bash
sudo bash scripts/clear.sh
```


