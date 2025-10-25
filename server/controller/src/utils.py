
import threading
import re
import os
import time
import sys
from config.parse_config import load_agents
import json


def install_agents(agents):
    progress_dict = {}
    def install_agent(agent_name, agent):
        print("clone code")
        agent.clone_code(progress_dict)
        agent.install(progress_dict)

    threads = []
    for agent_name, agent in agents.items():
        t = threading.Thread(target=install_agent, args=(agent_name, agent))
        t.start()
        threads.append(t)

    # 主线程定期刷新终端
    try:
        while any(t.is_alive() for t in threads):
            os.system('clear')  # 或 'cls' for Windows
            for agent_name in agents:
                print(f"{agent_name}: {progress_dict.get(agent_name, '等待中...')}")
            time.sleep(0.2)
    except KeyboardInterrupt:
        pass

    for t in threads:
        t.join()


def start_agents(agents):
    get_all_k8s_tags()
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



def all_services_ready(output):
    lines = output.strip().split('\n')
    if len(lines) <= 1:
        return False
    for line in lines[1:]:
        parts = line.split()
        if len(parts) < 4:
            continue
        service_name = parts[1]
        if service_name in ['social_cassandra-schema', 'social_wrk2']:
            continue
        replicas = parts[3]
        if "/" in replicas:
            cur, total = replicas.split("/")
            if cur != total or cur == "0":
                return False
    return True

def install_workload(agents):
    
    
    clone_command = "git clone https://gitee.com/gytlll/DeathStar.git"

    image_list = [
        'jaeger-cassandra-schema:latest',
        'jaeger-query:latest',
        'jaeger-collector:latest',
        'redis:latest',
        'memcached:latest',
        'jaeger-agent:latest',
        'social-network-microservices:latest',
        'mongo:4.4.6',
        'openresty-thrift:xenial',
        'media-frontend:xenial',
        'cassandra:3.9'
    ]

    def worker(agent_name, agent):
        print(f"{agent_name}: 开始克隆工作负载")
        code_path = agent.code_path
        check_command = f"cd {code_path} && [ -d DeathStar ] && rm -rf DeathStar"
        agent.execute_command(check_command)
        command = f"cd {code_path} && {clone_command}"
        output, error = agent.execute_command(command)
        if error:
            print(f"{agent_name} 克隆工作负载失败: {error}")
        else:
            print(f"{agent_name} 工作负载克隆成功")

        switch_source_cmd = f'cd {agent.code_path}/DeathStar; echo {agent.host_password} | sudo -S bash switch_source.sh'
        agent.execute_command(switch_source_cmd)
        # 检查并拉取镜像
        for image in image_list:
            check_image_cmd = f"docker images -q 47.97.67.233:5000/workload/{image}"
            output, _ = agent.execute_command(check_image_cmd)
            if not output.strip():
                print(f"{agent_name} 未找到镜像 {image}，开始拉取...")
                pull_cmd = f"docker pull 47.97.67.233:5000/workload/{image}"
                pull_output, pull_error = agent.execute_command(pull_cmd)
                if pull_error:
                    print(f"{agent_name} 拉取镜像 {image} 失败: {pull_error}")
                else:
                    print(f"{agent_name} 镜像 {image} 拉取成功")
            else:
                print(f"{agent_name} 已有镜像 {image}")


    threads = []
    for agent_name, agent in agents.items():
        t = threading.Thread(target=worker, args=(agent_name, agent))
        t.start()
        threads.append(t)
    for t in threads:
        t.join()

    # 先让所有节点都离开swarm，防止历史残留
    for agent_name, agent in agents.items():
        leave_command = f"echo {agent.host_password} | sudo -S docker swarm leave --force"
        output, error = agent.execute_command(leave_command)

    # 初始化swarm集群
    print("开始初始化swarm集群")
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
        sys.exit(1)

    # 其他节点加入swarm
    for agent_name, agent in agents.items():
        if agent_name == master_agent.agent_name:
            continue
        command = f"echo {agent.host_password} | sudo -S {join_command}"
        output, error = agent.execute_command(command)
        if error:
            print(f"{agent_name} 加入swarm集群失败: {error}")
            sys.exit(1)
        else:
            print(f"{agent_name} 加入swarm集群成功")

    # 检查swarm集群节点数量
    check_swarm_command = f'echo {master_agent.host_password} | sudo -S docker node ls'
    output, error = master_agent.execute_command(check_swarm_command)
    if error:
        print(f"{master_agent.agent_name} 检查swarm集群失败: {error}")
        sys.exit(1)
    else:
        print(f"{master_agent.agent_name} 检查swarm集群成功")
        print(output)
        # 检查节点数量和agent数量是否一致
        node_count = len([line for line in output.strip().split('\n')[1:] if line.strip()])
        agent_count = len(agents)
        if node_count == agent_count:
            print(f"节点数量({node_count})与agent数量({agent_count})一致, swarm集群正常")
        else:
            print(f"节点数量({node_count})与agent数量({agent_count})不一致，请检查！")
            
    # 启动workload
    print("开始启动工作负载")
    command = f"cd {master_agent.code_path}/DeathStar/socialNetwork/ && echo {master_agent.host_password} | sudo -S docker stack deploy -c docker-compose-swarm.yml social"
    output, error = master_agent.execute_command(command)
    if error:
        print(f"启动工作负载失败: {error}")
        sys.exit(1)
    else:
        print(f"等待工作负载启动...")
        
    while True:
        check_workload_command = f"echo {master_agent.host_password} | sudo -S docker service ls"
        output, error = master_agent.execute_command(check_workload_command)
        if error:
            print(f"{master_agent.agent_name} 检查工作负载失败: {error}")
            sys.exit(1)
        if all_services_ready(output):
            print(f"{master_agent.agent_name} 所有服务已启动成功")
            print(output)
            break
        else:
            print(f"{master_agent.agent_name} 服务尚未全部就绪，等待3秒后重试...")
            time.sleep(3)




def uninstall_workload(agents):
    master_agent = list(agents.values())[0]
    command = f"echo {master_agent.host_password} | sudo -S docker stack rm social"
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



def get_all_k8s_tags():
    agents = load_agents()
    all_tags = []
    for agent_name, agent in agents.items():
        tags = agent.get_k8s_tags()
        if tags:
            all_tags.extend(tags)

    tag_map = {}
    for tag in all_tags:
        if tag['type'] == 0:
            if tag['tgid'] not in tag_map:
                tag_map[tag['tgid']] = {
                    'namespace': tag['namespace'],
                    'pod_name': tag['pod_name'],
                    'uuid': tag['uuid'],
                    'hostname': '',
                    'ip': '',
                }
        if tag['type'] == 1:
            for key, value in tag_map.items():
                if tag['uuid'] == value['uuid']:
                    value['hostname'] = tag['hostname']
                    value['ip'] = tag['ip']
    
    with open("./config/k8s_tag_map.json", "w") as f:
        json.dump(tag_map, f, ensure_ascii=False, indent=2)