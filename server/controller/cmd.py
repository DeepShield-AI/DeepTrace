import json
import paramiko
from utils import *
from agent import Agent
import os
import time
import argparse  # 用于解析命令行参数
import threading

agents = {}
config_path = '../config/config.toml' 

def first_start_agent():
    for agent_name, agent in agents.items():
        agent.clone_code()
        agent.install_dependencies()
        agent.compile_code()
        agent.get_pids()
        agent.sync_config()
        agent.run()

def install():
    def install_agent(agent_name, agent):
        agent.clone_code()
        agent.install_dependencies()
        # agent.compile_code()
        agent.get_pids()
        agent.sync_config()

    threads = []
    for agent_name, agent in agents.items():
        t = threading.Thread(target=install_agent, args=(agent_name, agent))
        t.start()
        threads.append(t)
    for t in threads:
        t.join()


def start():
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

def update_agent_config(): # 热加载
    for agent_name, agent in agents.items():
        agent.update_config()

def stop():
    for agent_name, agent in agents.items():
        agent.stop()

def test():
    print("Test start")
    command = "echo 'Test Success'"  # 要在代理上执行的命令
    # 遍历所有代理并执行命令
    for agent_name, agent in agents.items():
        output, error = agent.execute_command(command)

        if output:
            print(f"{agent_name} output: {output}")
        if error:
            print(f"{agent_name} error: {error}")


# 主函数
def main():
    global agents  # 声明使用全局变量
    # 加载代理配置
    agents = load_agents(config_path)

    # 创建参数解析器
    parser = argparse.ArgumentParser(description="Agent Controller")
    parser.add_argument("action", choices=["first_start", "start", "update", "stop", "test", "install"], 
                        help="Action to perform on agents")
    args = parser.parse_args()

    # 根据参数执行不同的操作
    if args.action == "first_start":
        first_start_agent()
    elif args.action == "start":
        start()
    elif args.action == "update":
        update_agent_config()
    elif args.action == "stop":
        stop()
    elif args.action == "test":
        test()
    elif args.action == "install":
        install()

if __name__ == '__main__':
    main()