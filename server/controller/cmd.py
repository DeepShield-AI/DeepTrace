import json
import paramiko
from utils import *
from agent import Agent
import os

agents = {}


def start_agent():
    # 同步代码

    for agent_name, agent in agents.items():
        # agent.clone_code()
        # agent.install_dependencies()
        agent.compile_code()


def test():

    print("Test start")
    command = "echo 'Test Success'"  # 要在代理上执行的命令
    # 遍历所有代理并执行命令
    print(agents)
    for agent_name, agent in agents.items():
        output, error = agent.execute_command(command)

        if output:
            print(f"{agent_name} output: {output}")
        if error:
            print(f"{agent_name} error: {error}")



# 主函数
def main():
    global agents  # 声明使用全局变量
    config_path = '../config/config.json'  # 配置文件路径
    agents = load_agents(config_path)  # 更新全局变量
    start_agent()



if __name__ == '__main__':
    main()