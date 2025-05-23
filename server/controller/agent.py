import json
import paramiko
import os
from utils import *
from scp import SCPClient
import tarfile  # 用于压缩文件
from elasticsearch import Elasticsearch
import requests


agents = {}

# 管理agent状态：alive、处理延迟、条目、
# 用户管理：agent属于哪个用户
# 数据管理：数据属于哪个agent （重点
# 分析引擎管理：关联、基于trace做分析、tag、沈总的ProfileMap （重点
# 分成两组：trace、ProfileMap、志衡的
# api管理：web
# tag管理

class Agent:
    def __init__(self, agent_config, elastic_config):
        self.ssh_client = None
        self.agent_info = agent_config['agent_info']
        self.sender = agent_config['sender']
        self.ebpf = agent_config['ebpf']
        self.elastic_config = elastic_config
        self.api = agent_config['api']

        self.agent_name = self.agent_info['agent_name']
        self.host_ip = self.agent_info['host_ip']
        self.ssh_port = self.agent_info['ssh_port']
        self.host_password = self.agent_info['host_password']
        self.user_name = self.agent_info['user_name']
        self.code_path = self.expand_path(self.agent_info['code_path'])
        self.es_write_config()
        

    def es_write_config(self):
        try:
            # 准备要写入的数据
            agent_data = {
                "agent_info": self.agent_info,
                "sender": self.sender,
                "ebpf": self.ebpf,
                "api": self.api,
                "elastic_config": self.elastic_config,
            }

            # 初始化 Elasticsearch 客户端
            es_client = Elasticsearch(
                hosts=[f"http://0.0.0.0:{self.elastic_config['port']}"],
                basic_auth=("elastic", self.elastic_config['elastic_password'])
            )

            # 索引名称
            index_name = "agent_list"

            # 检查索引是否存在
            if not es_client.indices.exists(index=index_name):
                # 如果索引不存在，则创建索引
                es_client.indices.create(index=index_name)
                print(f"{self.agent_name}: 索引 {index_name} 已创建")

            # 查询是否存在指定 agent_name 的条目
            query = {
                "query": {
                    "term": {
                        "agent_info.agent_name.keyword": self.agent_name
                    }
                }
            }
            search_response = es_client.search(index=index_name, body=query)

            if search_response['hits']['total']['value'] > 0:
                # 如果存在，获取文档 ID 并更新
                doc_id = search_response['hits']['hits'][0]['_id']
                response = es_client.update(index=index_name, id=doc_id, body={"doc": agent_data})
                print(f"{self.agent_name}: 配置更新到 Elasticsearch")
            else:
                # 如果不存在，则插入新文档
                response = es_client.index(index=index_name, document=agent_data)
                print(f"{self.agent_name}: 配置插入到 Elasticsearch")

        except Exception as e:
            print(f"{self.agent_name}: 写入 Elasticsearch 失败 - {str(e)}")

    def connect(self):
        if not self.ssh_client:
            self.ssh_client = paramiko.SSHClient()
            self.ssh_client.set_missing_host_key_policy(paramiko.AutoAddPolicy())
            self.ssh_client.connect(
                hostname=self.host_ip, port=self.ssh_port, username=self.user_name, password=self.host_password
            )

    def disconnect(self):
        if self.ssh_client:
            self.ssh_client.close()
            self.ssh_client = None

    def execute_command(self, command):
        try:
            # print(f"{self.agent_name} execute: {command}")
            self.connect()
            stdin, stdout, stderr = self.ssh_client.exec_command(command)
            output = stdout.read().decode()
            error = stderr.read().decode()
            return output, error
        except Exception as e:
            return None, str(e)

    def expand_path(self, path):
        # 处理路径中的 ~ 符号
        if path.startswith('~'):
            output, error = self.execute_command(f"cd {path} && pwd")
        return output.strip() if output else path

    def sync_config(self):
        toml_content = f"""
[agent]
workers = {self.agent_info['workers']}
# channel_size = 4096

[api]
address = "{self.api['address']}"
port = {self.api['port']}
workers = {self.api['workers']}
ident = "{self.api['ident']}"

[sender]
batch_size = {self.sender['batch_size']}

[sender.flat_file]
mem_buffer_size = {self.sender['mem_buffer_size']}
file_buffer_size = {self.sender['file_buffer_size']}
file_size_limit = {self.sender['file_size_limit']}

[sender.elastic]
node_url = "http://{self.elastic_config['address']}:{self.elastic_config['port']}"
username = "elastic"
password = "{self.elastic_config['elastic_password']}"
request_timeout = {self.elastic_config['request_timeout']}
index_name = "{self.sender['index_name']}"
bulk_size = {self.elastic_config['bulk_size']}

[ebpf]
pids = {self.ebpf['pids']}
"""
        # 目标文件路径
        remote_file_path = f"{self.code_path}/DeepTrace/agent/config/default.toml"

        try:
            # 将 toml_content 写入到远程主机的目标文件
            self.connect()
            sftp = self.ssh_client.open_sftp()
            with sftp.file(remote_file_path, 'w') as remote_file:
                remote_file.write(toml_content.strip())
            sftp.close()

            print(f"{self.agent_name}: 配置文件已同步 {remote_file_path}")
        except Exception as e:
            print(f"{self.agent_name}: 同步配置文件失败 - {str(e)}")
        


    def clone_code(self):
        try:
            # 清除老代码
            command = f"cd {self.code_path} && [ -d DeepTrace ] && rm -rf DeepTrace"
            self.execute_command(command)

            # 检查目标路径是否存在，不存在则创建
            repo_url = 'https://gitee.com/gytlll/DeepTrace.git'
            command = f"mkdir -p {self.code_path} && cd {self.code_path} && GIT_LFS_SKIP_SMUDGE=1 git clone {repo_url}"
            # print(f"在远程主机执行命令: {command}")
            
            # 执行命令
            output, error = self.execute_command(command)

            if error and "should have been pointers, but weren't" not in error:
                raise Exception(f"克隆代码失败: {error}")

            print(f"代码仓库已克隆到 {self.agent_name} 的 {self.code_path}/DeepTrace")
        except Exception as e:
            print(f"克隆代码到 {self.agent_name} 失败: {str(e)}")
    
    def install_dependencies(self):
        command = f"cd {self.code_path}/DeepTrace/scripts && echo {self.host_password} | sudo -S bash install.sh"
        output, error = self.execute_command(command)
        if error and 'Updating crates.io index' not in error:
            raise Exception(f"{self.agent_name}: 安装依赖失败 {error}")
        else:
            print(f'{self.agent_name}: 安装依赖成功')

    def compile_code(self):
        command = f"cd {self.code_path}/DeepTrace/scripts && echo {self.host_password} | sudo -S bash compile.sh"
        output, error = self.execute_command(command)
        if error and 'profile [optimized] target(s)' not in error:
            raise Exception(f"{self.agent_name}: 编译失败 {error}")
        else:
            print(f'{self.agent_name}: 编译成功')

    def get_pids(self):
        command = f"cd {self.code_path}/DeepTrace && echo {self.host_password} | sudo -S bash scripts/docker_pids.sh"
        output, error = self.execute_command(command)
        if error and 'error' in error:
            raise Exception(f"{self.agent_name}: 获取进程失败 {error}")
        else:
            print(f'{self.agent_name}: 获取进程成功')
            self.ebpf['pids'] = output.strip().split('\n')
            print(f'{self.agent_name}: 进程列表 {self.ebpf["pids"]}')
            return output

    def run(self):
        # RUST_LOG=info cargo run --release --config 'target."cfg(all())".runner="sudo -E"' -- &
        # command = f"cd {self.code_path}/DeepTrace && bash scripts/run.sh"
        command = f"cd {self.code_path}/DeepTrace && echo {self.host_password} | sudo -S bash scripts/run.sh"
        output, error = self.execute_command(command)
        if error and 'error' in error:
            raise Exception(f"{self.agent_name}: 启动失败 {error}")
        else:
            print(f'{self.agent_name}: 启动成功 {output}')

    
    def stop(self):
        # pkill -f 'target/release/agent'
        command = f"echo {self.host_password} | sudo -S pkill -f 'target/release/agent'"
        output, error = self.execute_command(command)
        if error and 'error' in error:
            raise Exception(f"{self.agent_name}: 停止失败 {error}")
        else:
            print(f'{self.agent_name}: 停止成功 {output}')

    def update_config(self):
        
        self.sync_config()
        # 准备要发送的 JSON 数据
        config_data = {
            "agent": {
                "workers": self.agent_info['workers']
            },
            "sender": {
                "batch_size": self.sender['batch_size'],
                "flat_file": {
                    "mem_buffer_size": self.sender['mem_buffer_size'],
                    "file_buffer_size": self.sender['file_buffer_size'],
                    "file_size_limit": self.sender['file_size_limit']
                },
                "elastic": {
                    "node_url": f"http://{self.elastic_config['address']}:{self.elastic_config['port']}",
                    "username": "elastic",
                    "password": self.elastic_config['elastic_password'],
                    "request_timeout": self.elastic_config['request_timeout'],
                    "index_name": self.sender['index_name'],
                    "bulk_size": self.elastic_config['bulk_size']
                }
            },
            "ebpf": {
                "pids": self.ebpf['pids']
            },
            "api": {
                "address": self.api['address'],
                "port": self.api['port'],
                "workers": self.api['workers'],
                "ident": self.api['ident']
            }
        }

        # 目标 URL
        url = f"http://{self.host_ip}:{self.api['port']}/api/config/update"

        # 发送 POST 请求
        headers = {"Content-Type": "application/json"}
        response = requests.post(url, json=config_data, headers=headers)

        if response.status_code == 200:
            print(f"{self.agent_name}: 配置更新成功")
        else:
            print(f"{self.agent_name}: 配置更新失败，状态码: {response.status_code}, 响应: {response.text}")


