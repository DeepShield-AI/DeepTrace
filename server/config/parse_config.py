import toml

config_path = './config/config.toml'

def read_db_config():
    with open(config_path, "r") as f:
        config = toml.load(f)
    try:
        elastic_pwd = config.get("elastic", {}).get("elastic_password")
        kibana_pwd = config.get("elastic", {}).get("elastic_password")
        server_ip = config.get("server", {}).get("ip")
    except KeyError as e:
        raise KeyError(f"请设置elastic_password和kibana_password: {e}")
    
    return elastic_pwd, server_ip

def load_agents():
    from controller.src.agent import Agent
    with open(config_path, 'r') as f:
        config = toml.load(f)
    elastic_config = config.get('elastic', {})
    if 'port' not in elastic_config:
        elastic_config['port'] = 9200
    if 'username' not in elastic_config:
        elastic_config['username'] = 'elastic'
    if 'request_timeout' not in elastic_config:
        elastic_config['request_timeout'] = 10
    if 'bulk_size' not in elastic_config:
        elastic_config['bulk_size'] = 1024
    if 'agent_status_index' not in elastic_config:
        elastic_config['agent_status_index'] = 'agent_status'
    server_config = config.get('server', {})
    agent_dict = {}
    if 'path' not in server_config:
        server_config['path'] = 'deeptrace/ws'
    if 'port' not in server_config:
        server_config['port'] = 7901
    for agent_config in config.get('agents', []):
        if 'deeptrace_port' not in agent_config['agent_info']:
            agent_config['agent_info']['deeptrace_port'] = 52001
        if 'workers' not in agent_config['agent_info']:
            agent_config['agent_info']['workers'] = 16
        if 'span' not in agent_config:
            agent_config['span'] = {'batch_size': 1024}
        if 'sender' not in agent_config:
            agent_config['sender'] = {'index_name': f"spans_{agent_config['agent_info']['agent_name']}",
                                      'mem_buffer_size': 16,
                                      'file_buffer_size': 32,
                                      'file_size_limit': 1024,
                                      'batch_size': 1024}
        else:
            if 'index_name' not in agent_config['sender']:
                agent_config['sender']['index_name'] = f"spans_{agent_config['agent_info']['agent_name']}"
            if 'mem_buffer_size' not in agent_config['sender']:
                agent_config['sender']['mem_buffer_size'] = 16
            if 'file_buffer_size' not in agent_config['sender']:
                agent_config['sender']['file_buffer_size'] = 32
            if 'file_size_limit' not in agent_config['sender']:
                agent_config['sender']['file_size_limit'] = 1024
            if 'batch_size' not in agent_config['sender']:
                agent_config['sender']['batch_size'] = 1024
        if 'trace' not in agent_config:
            agent_config['trace'] = {'pids': []}
        if 'api' not in agent_config:
            agent_config['api'] = {'port': 7899, 'address': '0.0.0.0', 'workers': 1, 'ident': 'deeptrace'}
        else:
            if 'port' not in agent_config['api']:
                agent_config['api']['port'] = 7899
            if 'address' not in agent_config['api']:
                agent_config['api']['address'] = '0.0.0'
            if 'workers' not in agent_config['api']:
                agent_config['api']['workers'] = 1
            if 'ident' not in agent_config['api']:
                agent_config['api']['ident'] = 'deeptrace'
        agent_dict[agent_config['agent_info']['agent_name']] = Agent(agent_config, elastic_config, server_config)
    return agent_dict

def get_server_mode():
    with open(config_path, 'r') as f:
        config = toml.load(f)
    server_config = config.get('server', {})
    return server_config.get('mode', 'automatic')
