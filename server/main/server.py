from flask import Flask, request, jsonify
from config.parse_config import load_default_configs
from controller.src.agent import Agent
import copy

app = Flask(__name__)

# 模拟存储 agent 信息的字典
agent_config, elastic_config, server_config = load_default_configs()
elastic_config['elastic_password'] = 'deeptrace123'
elastic_config['address'] = '202.112.237.37'
server_config['ip'] = '202.112.237.37'

def parse_agent(request):
    """
    处理注册请求，解析请求数据并创建 Agent 实例
    """
    data = request.json
    print(f"Received registration request: {data}")  # Print request info

    # 必要参数检查
    required_params = ['host_ip', 'user_name', 'host_password', 'user_id', 'ssh_port', 'agent_name']
    if not all(param in data for param in required_params):
        return jsonify({'error': 'Missing required parameters'}), 400
    
    _agent_config = copy.deepcopy(agent_config)

    # 更新 agent_config
    _agent_config['agent_info']['host_ip'] = data['host_ip']
    _agent_config['agent_info']['user_name'] = data['user_name']
    _agent_config['agent_info']['host_password'] = data['host_password']
    _agent_config['agent_info']['user_id'] = data['user_id']
    _agent_config['agent_info']['ssh_port'] = data['ssh_port']
    _agent_config['agent_info']['agent_name'] = data['agent_name']

    # 创建 Agent 实例
    agent = Agent(_agent_config, elastic_config, server_config)
    return agent


@app.route('/register_agent', methods=['POST'])
def register_agent():
    """
    Register agent
    """
    agent = parse_agent(request)
    log_dict = {}
    print(f"Starting installation for agent: {agent.agent_name}")
    agent.clone_code(log_dict)
    print(f"Cloning code for agent: {agent.agent_name} completed")
    agent.install(log_dict, print_log=True)
    print(f"Installation for agent: {agent.agent_name} completed")
    
    return jsonify({'message': f'Agent {agent.agent_name} registered successfully'}), 200

@app.route('/start_agent', methods=['POST'])
def start_agent():
    """
    Start agent
    """
    agent = parse_agent(request)
    print(f"Starting agent: {agent.agent_name}")
    agent.get_pids()
    print(f"Agent {agent.agent_name} PIDs retrieved")
    agent.sync_config()
    print(f"Agent {agent.agent_name} configuration synchronized")
    agent.run()
    print(f"Agent {agent.agent_name} started")
    return jsonify({'message': f'Agent {agent.agent_name} started successfully'}), 200

@app.route('/stop_agent', methods=['POST'])
def stop_agent():
    """
    Stop agent
    """
    agent = parse_agent(request)
    print(f"Stopping agent: {agent.agent_name}")
    agent.stop()
    print(f"Agent {agent.agent_name} stopped")
    return jsonify({'message': f'Agent {agent.agent_name} stopped successfully'}), 200

@app.route('/delete_agent', methods=['POST'])
def delete_agent():
    """
    Delete agent
    """
    agent = parse_agent(request)
    return jsonify({'message': f'Agent {agent.agent_name} deleted successfully'}), 200


@app.route('/sync_agent_config', methods=['POST'])
def config_agent():
    """
    下发配置到指定的 Agent
    """
    data = request.json
    print(f"Received configuration request: {data}")  # 打印接收到的配置请求

    # 检查是否包含必要的字段
    if 'agent_info' not in data or 'agent_name' not in data['agent_info']:
        return jsonify({'error': 'Missing required agent_info or agent_name'}), 400

    agent_name = data['agent_info']['agent_name']
    
    _agent_config = copy.deepcopy(agent_config)

    try:
        # 遍历 JSON 数据并更新 Agent 的配置
        for key, value in data.items():
            if key in _agent_config:
                _agent_config[key] = value
            else:
                _agent_config[key] = value  # 添加新的配置字段
                
        agent = Agent(_agent_config, elastic_config, server_config)
        agent.sync_config()
        print(f"Configuration updated for agent: {agent_name}")
        print(f"New configuration: {_agent_config}")
        return jsonify({'message': f'Configuration updated for agent {agent_name}'}), 200
    except Exception as e:
        print(f"Error updating configuration for agent {agent_name}: {e}")
        return jsonify({'error': f'Failed to update configuration for agent {agent_name}'}), 500

@app.route('/query_agent_config', methods=['POST'])
def query_agent_config():
    """
    查询指定 Agent 的配置，并调整返回的结构
    """
    agent = parse_agent(request)
    config_data = agent.query_config()
    if config_data:
        print(f"Configuration file content retrieved successfully for agent: {agent.agent_name}")

        # 转换配置结构
        transformed_config = {
            'agent_info': {
                'agent_name': config_data['agent']['name'],
                'host_password': agent.agent_info['host_password'],
                'host_ip': agent.agent_info['host_ip'],
                'user_name': agent.agent_info['user_name'],
                'ssh_port': agent.agent_info['ssh_port']
            },
            'metric': config_data.get('metric', {}),
            'sender': config_data.get('sender', {}),
            'trace': config_data.get('trace', {}),
            'ebpf': config_data.get('ebpf', {})
        }

        # 打印转换后的配置
        print(f"Transformed configuration for agent: {agent.agent_name}")
        print(transformed_config)

        return jsonify(transformed_config), 200
    else:
        print(f"Failed to retrieve configuration file for agent: {agent.agent_name}")
        return jsonify({'error': 'Failed to retrieve configuration file'}), 500

if __name__ == '__main__':
    app.run(host='0.0.0.0', port=59002)
    print(agent_config)
    print(elastic_config)
    print(server_config)
    
"""
curl -X POST http://127.0.0.1:59002/register_agent -H "Content-Type: application/json" -d '{
  "host_ip": "118.229.43.254",
  "user_name": "ubuntu",
  "host_password": "netsys204",
  "user_id": "user123",
  "ssh_port": 6114,
  "agent_name": "agent1"
}'

curl -X POST http://127.0.0.1:59002/start_agent -H "Content-Type: application/json" -d '{
  "host_ip": "118.229.43.254",
  "user_name": "ubuntu",
  "host_password": "netsys204",
  "user_id": "user123",
  "ssh_port": 6114,
  "agent_name": "agent1"
}'

curl -X POST http://127.0.0.1:59002/stop_agent -H "Content-Type: application/json" -d '{
  "host_ip": "118.229.43.254",
  "user_name": "ubuntu",
  "host_password": "netsys204",
  "user_id": "user123",
  "ssh_port": 6114,
  "agent_name": "agent1"
}'

curl -X POST http://127.0.0.1:59002/query_agent_config -H "Content-Type: application/json" -d '{
  "host_ip": "118.229.43.254",
  "user_name": "ubuntu",
  "host_password": "netsys204",
  "user_id": "user123",
  "ssh_port": 6114,
  "agent_name": "agent1"
}'


curl -X POST http://127.0.0.1:59002/sync_agent_config -H "Content-Type: application/json" -d '{
  "agent_info": {
    "agent_name": "agent1",
    "host_password": "netsys204",
    "host_ip": "118.229.43.254",
    "user_name": "ubuntu",
    "ssh_port": 6114,
    "user_id": "user123"
  },
  "metric": {
    "interval": 10,
    "sender": "metric"
  },
  "sender": {
    "elastic": {
      "trace": {
        "node_url": "http://localhost:9200",
        "username": "elastic",
        "password": "new_password",
        "request_timeout": 10,
        "index_name": "agent1",
        "bulk_size": 64
      }
    },
    "file": {
      "metric": {
        "path": "metrics.csv",
        "rotate": true,
        "max_size": 512,
        "max_age": 6,
        "rotate_time": 11,
        "data_format": "%Y%m%d"
      }
    }
  },
  "trace": {
    "ebpf": "trace",
    "sender": "trace",
    "span": {
      "cleanup_interval": 30,
      "max_sockets": 1024
    }
  },
  "ebpf": {
    "trace": {
      "log_level": 1,
      "pids": [523094],
      "max_buffered_events": 128,
      "enabled_probes": [
        "sys_enter_read",
        "sys_exit_read",
        "sys_enter_readv",
        "sys_exit_readv",
        "sys_enter_recvfrom",
        "sys_exit_recvfrom",
        "sys_enter_recvmsg",
        "sys_exit_recvmsg",
        "sys_enter_recvmmsg",
        "sys_exit_recvmmsg",
        "sys_enter_write",
        "sys_exit_write",
        "sys_enter_writev",
        "sys_exit_writev",
        "sys_enter_sendto",
        "sys_exit_sendto",
        "sys_enter_sendmsg",
        "sys_exit_sendmsg",
        "sys_enter_sendmmsg",
        "sys_exit_sendmmsg",
        "sys_exit_socket",
        "sys_enter_close"
      ]
    }
  }
}'
"""