
import threading
import re


def install_agents(agents):
    def install_agent(agent_name, agent):
        agent.clone_code()
        agent.install()

    threads = []
    for agent_name, agent in agents.items():
        t = threading.Thread(target=install_agent, args=(agent_name, agent))
        t.start()
        threads.append(t)
    for t in threads:
        t.join()


def start_agents(agents):
    def start_agent(agent_name, agent):
        agent.get_pids()
        agent.sync_config()
        agent.run()
    threads = []
    for agent_name, agent in agents.items():
        t = threading.Thread(target=start_agent, args=(agent_name, agent))
        t.start()
        threads.append(t)
    for t in threads:
        t.join()

def update_agent_config(agents): # 热加载
    for agent_name, agent in agents.items():
        agent.update_config()

def sync_agent_config(agents): # 冷加载
    for agent_name, agent in agents.items():
        agent.get_pids()
        agent.sync_config()

def stop_agents(agents):
    for agent_name, agent in agents.items():
        agent.stop()

def test_agents(agents):
    print("Test start")
    command = "echo 'Test Success'"  # 要在代理上执行的命令
    # 遍历所有代理并执行命令
    for agent_name, agent in agents.items():
        output, error = agent.execute_command(command)

        if output:
            print(f"{agent_name} output: {output}")
        if error:
            print(f"{agent_name} error: {error}")



def install_workload(agents):
    clone_command = "git clone https://gitee.com/gytlll/DeathStar.git"
    for agent_name, agent in agents.items():
        code_path = agent.code_path
        check_command = f"cd {code_path} && [ -d DeathStar ] && rm -rf DeathStar"
        agent.execute_command(check_command)
        command = f"cd {code_path} && {clone_command}"
        output, error = agent.execute_command(command)
        if error:
            print(f"{agent_name} 克隆工作负载失败: {error}")
        else:
            print(f"{agent_name} 工作负载克隆成功")

    # 安装swarm集群
    master_agent = list(agents.values())[0]
    init_command = f'echo {master_agent.host_password} | sudo -S docker swarm init'
    output, error = master_agent.execute_command(init_command)
    match = re.search(r'docker swarm join [^\n]+', output)
    if match:
        join_command = match.group(0).strip()
        print("Join command:", join_command)
        print(f"{master_agent.agent_name} 初始化swarm集群成功")
    else:
        print("No join command found.")
    for agent_name, agent in agents.items():
        if agent_name == master_agent.agent_name:
            continue
        command = f"echo {agent.host_password} | sudo -S {join_command}"
        output, error = agent.execute_command(command)
        if error:
            print(f"{agent_name} 加入swarm集群失败: {error}")
        else:
            print(f"{agent_name} 加入swarm集群成功")
    check_swarm_command = f'echo {master_agent.host_password} | sudo -S docker node ls'
    output, error = master_agent.execute_command(check_swarm_command)
    if error:
        print(f"{master_agent.agent_name} 检查swarm集群失败: {error}")
    else:
        print(f"{master_agent.agent_name} 检查swarm集群成功")
        print(output)

    # 启动workload
    command = f"cd {master_agent.code_path}/DeathStar/socialNetwork/ && echo {master_agent.host_password} | sudo -S docker stack deploy -c docker-compose-swarm.yml soaicl"
    output, error = master_agent.execute_command(command)
    if error:
        print(f"{master_agent.agent_name} 启动工作负载失败: {error}")
    else:
        print(f"{master_agent.agent_name} 启动工作负载成功")
        check_workload_command = f"echo {master_agent.host_password} | sudo -S docker service ls"
        output, error = master_agent.execute_command(check_workload_command)
        if error:
            print(f"{master_agent.agent_name} 检查工作负载失败: {error}")
        else:
            print(f"{master_agent.agent_name} 检查工作负载成功")
            print(output)




def uninstall_workload(agents):
    master_agent = list(agents.values())[0]
    command = f"echo {master_agent.host_password} | sudo -S docker stack rm soaicl"
    output, error = master_agent.execute_command(command)
    if error:
        print(f"{master_agent.agent_name} 卸载工作负载失败: {error}")
    else:
        print("卸载工作负载成功")
    
    # 清除swarm
    for agent_name, agent in agents.items():
        command = f"echo {agent.host_password} | sudo -S docker swarm leave --force"
        output, error = agent.execute_command(command)
        if error:
            print(f"{agent_name} 离开swarm集群失败: {error}")
        else:
            print(f"{agent_name} 离开swarm集群成功")

