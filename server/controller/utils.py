import json
from agent import Agent


# 读取配置文件 实例化 Agent
def load_agents(config_path):
    with open(config_path, 'r') as f:
        config = json.load(f)
    elastic_config = config.get('elastic', {})
    agent_dict = {agent_config['agent_info']['agent_name']: Agent(agent_config, elastic_config) for agent_config in config['agents']}
    return agent_dict
