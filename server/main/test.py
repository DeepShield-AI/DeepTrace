import os
import argparse

# 定义 API 地址
BASE_URL = "http://127.0.0.1:59002"

# 定义请求命令
register_agent_cmd = """curl -X POST http://127.0.0.1:59002/register_agent -H "Content-Type: application/json" -d '{
  "host_ip": "118.229.43.254",
  "user_name": "ubuntu",
  "host_password": "netsys204",
  "user_id": "user123",
  "ssh_port": 6114,
  "agent_name": "agent1"
}'"""

start_agent_cmd = """curl -X POST http://127.0.0.1:59002/start_agent -H "Content-Type: application/json" -d '{
  "host_ip": "118.229.43.254",
  "user_name": "ubuntu",
  "host_password": "netsys204",
  "user_id": "user123",
  "ssh_port": 6114,
  "agent_name": "agent1"
}'"""

stop_agent_cmd = """curl -X POST http://127.0.0.1:59002/stop_agent -H "Content-Type: application/json" -d '{
  "host_ip": "118.229.43.254",
  "user_name": "ubuntu",
  "host_password": "netsys204",
  "user_id": "user123",
  "ssh_port": 6114,
  "agent_name": "agent1"
}'"""

query_agent_config_cmd = """
curl -X POST http://127.0.0.1:59002/query_agent_config -H "Content-Type: application/json" -d '{
  "host_ip": "118.229.43.254",
  "user_name": "ubuntu",
  "host_password": "netsys204",
  "user_id": "user123",
  "ssh_port": 6114,
  "agent_name": "agent1"
}'
"""

sync_agent_config_cmd = """curl -X POST http://127.0.0.1:59002/sync_agent_config -H "Content-Type: application/json" -d '{
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
}'"""

# 定义主函数
def main():
    parser = argparse.ArgumentParser(description="Agent Management CLI")
    parser.add_argument("command", choices=["register", "start", "stop", "query", "sync"], help="Command to execute")
    args = parser.parse_args()

    if args.command == "register":
        print("Registering Agent...")
        os.system(register_agent_cmd)

    elif args.command == "start":
        print("Starting Agent...")
        os.system(start_agent_cmd)

    elif args.command == "stop":
        print("Stopping Agent...")
        os.system(stop_agent_cmd)

    elif args.command == "query":
        print("Querying Agent Config...")
        os.system(query_agent_config_cmd)

    elif args.command == "sync":
        print("Syncing Agent Config...")
        os.system(sync_agent_config_cmd)

# 调用主函数
if __name__ == "__main__":
    main()