import json
from agent import Agent


# 读取配置文件 实例化 Agent
def load_agents(config_path):
    with open(config_path, 'r') as f:
        config = json.load(f)
    agent_dict = {info['agent_name']: Agent(info) for info in config['agents']}
    return agent_dict
