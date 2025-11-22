
import copy
import toml

config_path = './config/config.toml'
default_path = './config/full.toml'

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

def _deep_merge(default: dict, override: dict) -> dict:
    """
    递归合并：override 中的值覆盖 default。list/primitive 类型直接替换。
    """
    result = copy.deepcopy(default) if default is not None else {}
    for k, v in (override or {}).items():
        if k in result and isinstance(result[k], dict) and isinstance(v, dict):
            result[k] = _deep_merge(result[k], v)
        else:
            result[k] = copy.deepcopy(v)
    return result


def load_default_configs():
    """
    加载默认配置，包括 agent_config、elastic_config 和 server_config
    """
    from controller.src.agent import Agent

    # 读取默认模板配置
    with open(default_path, 'r') as f:
        default_conf = toml.load(f)

    # 加载默认 server 配置
    server_config = default_conf.get('server', {})
    
    # 加载默认 elastic 配置
    elastic_config = default_conf.get('elastic', {})
    elastic_config['address'] = server_config.get('ip', 'localhost')
    elastic_config.setdefault('port', 9200)
    elastic_config.setdefault('username', 'elastic')
    elastic_config.setdefault('request_timeout', 10)
    elastic_config.setdefault('bulk_size', 10)

    default_agents = default_conf.get('agents', [])
    for default_item in default_agents:
        if 'agent' not in default_item:
            raise KeyError(f"每个 [[agents]] 项必须包含 [agents.agent] 区块，问题项：{default_item}")

        raw_agent = default_item.get('agent', {})
        agent_name = raw_agent.get('agent_name') or raw_agent.get('name')
        host_password = raw_agent.get('host_password')
        host_ip = raw_agent.get('host_ip')

        # 必填字段检查
        if not agent_name:
            raise KeyError(f"agents 配置缺少必填字段 agent_name/name: {raw_agent}")
        if not host_password:
            raise KeyError(f"agents 配置缺少必填字段 host_password: {raw_agent}")
        if not host_ip:
            raise KeyError(f"agents 配置缺少必填字段 host_ip: {raw_agent}")

        # 构造 agent_info
        agent_info = {
            'agent_name': agent_name,
            'host_password': host_password,
            'host_ip': host_ip,
        }
        if 'user_name' in raw_agent:
            agent_info['user_name'] = raw_agent['user_name']
        if 'ssh_port' in raw_agent:
            try:
                agent_info['ssh_port'] = int(raw_agent['ssh_port'])
            except Exception:
                agent_info['ssh_port'] = raw_agent['ssh_port']

        # 组装 agent_config
        agent_config = {'agent_info': agent_info}
        for key in ('metric', 'sender', 'trace', 'ebpf', 'api', 'span'):
            if key in default_item:
                agent_config[key] = default_item[key]

        break  # 只加载第一个 agent 作为默认配置

    return agent_config, elastic_config, server_config

def load_agents():
    from controller.src.agent import Agent

    # 读取默认模板和用户配置
    default_conf = {}

    with open(default_path, 'r') as f:
        default_conf = toml.load(f)

    with open(config_path, 'r') as f:
        user_conf = toml.load(f)


    server_config = user_conf.get('server', {})

        # elastic defaults (保留 elastic 部分默认项)
    elastic_config = user_conf.get('elastic', {})
    elastic_config['address'] = server_config.get('ip', 'localhost')
    elastic_config.setdefault('port', 9200)
    elastic_config.setdefault('username', 'elastic')
    elastic_config.setdefault('request_timeout', 10)
    elastic_config.setdefault('bulk_size', 10)

    agent_dict = {}

    # 选择默认模板中的第一个 agents 项作为模板（如果存在）
    default_agents = default_conf.get('agents', [])
    default_template = default_agents[0] if default_agents else {}

    for user_item in user_conf.get('agents', []):
        # 用户项必须包含 agent 区块
        if 'agent' not in user_item:
            raise KeyError(f"每个 [[agents]] 项必须包含 [agents.agent] 区块，问题项：{user_item}")

        # 合并：基于默认模板合并用户项（用户覆盖默认）
        merged_item = _deep_merge(default_template, user_item)

        raw_agent = merged_item.get('agent', {})
        # 名称支持 agent_name 或 name
        agent_name = raw_agent.get('agent_name') or raw_agent.get('name')
        host_password = raw_agent.get('host_password')
        host_ip = raw_agent.get('host_ip')

        # 必填字段检查
        if not agent_name:
            raise KeyError(f"agents 配置缺少必填字段 agent_name/name: {raw_agent}")
        if not host_password:
            raise KeyError(f"agents 配置缺少必填字段 host_password: {raw_agent}")
        if not host_ip:
            raise KeyError(f"agents 配置缺少必填字段 host_ip: {raw_agent}")

        # 构造 agent_info（使用合并后的值）
        agent_info = {
            'agent_name': agent_name,
            'host_password': host_password,
            'host_ip': host_ip,
        }
        if 'user_name' in raw_agent:
            agent_info['user_name'] = raw_agent['user_name']
        if 'ssh_port' in raw_agent:
            try:
                agent_info['ssh_port'] = int(raw_agent['ssh_port'])
            except Exception:
                agent_info['ssh_port'] = raw_agent['ssh_port']

        # 组装 agent_config：从合并后的 item 中保留存在的区块（metric/sender/trace/ebpf/span/api）
        agent_config = {'agent_info': agent_info}
        for key in ('metric', 'sender', 'trace', 'ebpf', 'api', 'span'):
            if key in merged_item:
                agent_config[key] = merged_item[key]

        agent_dict[agent_name] = Agent(agent_config, elastic_config, server_config)

    return agent_dict

def get_server_mode():
    with open(config_path, 'r') as f:
        config = toml.load(f)
    server_config = config.get('server', {})
    return server_config.get('mode', 'automatic')