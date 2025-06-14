import toml

config_path = './config/config.toml'

def read_db_config():
    with open(config_path, "r") as f:
        config = toml.load(f)
    try:
        elastic_pwd = config.get("elastic", {}).get("elastic_password")
        kibana_pwd = config.get("elastic", {}).get("kibana_password")
    except KeyError as e:
        raise KeyError(f"请设置elastic_password和kibana_password: {e}")
    
    return elastic_pwd, kibana_pwd

def load_agents():
    from controller.src.agent import Agent
    with open(config_path, 'r') as f:
        config = toml.load(f)
    elastic_config = config.get('elastic', {})
    server_config = config.get('server', {})
    agent_dict = {agent_config['agent_info']['agent_name']: Agent(agent_config, elastic_config, server_config) for agent_config in config['agents']}
    return agent_dict