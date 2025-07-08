import json
import paramiko
import requests
from database.src.utils import es_write_agent_config
import time
import threading


# 管理agent状态：alive、处理延迟、条目、
# 用户管理：agent属于哪个用户
# 数据管理：数据属于哪个agent （重点
# 分析引擎管理：关联、基于trace做分析、tag、沈总的ProfileMap （重点
# 分成两组：trace、ProfileMap、志衡的
# api管理：web
# tag管理

class Agent:
    def __init__(self, agent_config, elastic_config, server_config):
        self.ssh_client = None
        self.server_config = server_config
        self.agent_config = agent_config
        self.agent_info = agent_config['agent_info']
        self.sender = agent_config['sender']
        self.trace = agent_config['trace']
        self.elastic_config = elastic_config
        self.api = agent_config['api']

        self.agent_name = self.agent_info['agent_name']
        self.host_ip = self.agent_info['host_ip']
        self.ssh_port = self.agent_info['ssh_port']
        self.host_password = self.agent_info['host_password']
        self.user_name = self.agent_info['user_name']
        # self.code_path = self.expand_path(self.agent_info['code_path'])
        self.code_path = '/etc'
        es_write_agent_config(self.agent_config, self.elastic_config, self.server_config)
        


    def connect(self):
        if not self.ssh_client:
            self.ssh_client = paramiko.SSHClient()
            self.ssh_client.set_missing_host_key_policy(paramiko.AutoAddPolicy())
            self.ssh_client.connect(
                hostname=self.host_ip, port=self.ssh_port, username=self.user_name, password=self.host_password
            )
        # print(f"{self.agent_name} 已连接到 {self.host_ip}:{self.ssh_port}")


    def disconnect(self):
        if self.ssh_client:
            self.ssh_client.close()
            self.ssh_client = None

    def execute_command(self, command):
        try:
            # print(f"{self.agent_name} execute: {command}")
            self.connect()
            stdin, stdout, stderr = self.ssh_client.exec_command(command, get_pty=True)
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
name = "{self.agent_name}"
workers = {self.agent_info['workers']}
state_index = "{self.elastic_config['agent_status_index']}"
# channel_size = 4096

[server]
ip  = "{self.server_config['ip']}"
port = {self.server_config['port']}
path = "{self.server_config['path']}"

[api]
address = "{self.api['address']}"
port = {self.api['port']}
workers = {self.api['workers']}
ident = "{self.api['ident']}"

[sender]
batch_size = {self.sender['batch_size']}

[span]
batch_size = {self.agent_config['span']['batch_size']}

[sender.flat_file]
mem_buffer_size = {self.sender['mem_buffer_size']}
file_buffer_size = {self.sender['file_buffer_size']}
file_size_limit = {self.sender['file_size_limit']}

[sender.elastic]
node_url = "http://{self.server_config['ip']}:{self.elastic_config['port']}"
username = "{self.elastic_config['username']}"
password = "{self.elastic_config['elastic_password']}"
request_timeout = {self.elastic_config['request_timeout']}
index_name = "{self.sender['index_name']}"
bulk_size = {self.elastic_config['bulk_size']}

[trace]
pids = {self.trace['pids']}

[provenance]
pids = []
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
        


    def clone_code(self, progress_dict):
        try:
            # 清除老代码
            command = f"cd {self.code_path} ; ls"
            output, err = self.execute_command(command)
            if "DeepTrace" in output:
                command = f"cd {self.code_path} && echo {self.host_password} | sudo -S rm -rf DeepTrace"
                output, error = self.execute_command(command)
                if error:
                    raise Exception(f"清除老代码失败: {error}")
                else:
                    progress_dict[self.agent_name] = "清除老代码成功"
                   

            # 检查目标路径是否存在，不存在则创建
            repo_url = 'https://gitee.com/gytlll/DeepTrace.git'
            command = f"mkdir -p {self.code_path} && cd {self.code_path} && GIT_LFS_SKIP_SMUDGE=1 git clone {repo_url}"
            # print(f"在远程主机执行命令: {command}")
            
            # 执行命令
            output, error = self.execute_command(command)

                        # 只在真正的git错误时才抛出异常
            if error and any(x in error.lower() for x in ["fatal", "error", "failed"]):
                raise Exception(f"克隆代码失败: {error}")

            progress_dict[self.agent_name] = f"代码已克隆到 {self.code_path}/DeepTrace"

        except Exception as e:
            print(f"克隆代码到 {self.agent_name} 失败: {str(e)}")
    

    def install(self, progress_dict):
        t1 = time.time()
        check_command = f"cd {self.code_path}/DeepTrace/agent && [ -d target ] && rm -rf target"
        self.execute_command(check_command)
        progress_dict[self.agent_name] = "开始安装..."

        stop_event = threading.Event()

        def run_install():
            command = f"cd {self.code_path}/DeepTrace ; echo {self.host_password} | sudo -S bash scripts/install_agent.sh > agent.log 2>&1"
            self.execute_command(command)
            stop_event.set()  # 安装完成后通知日志线程退出

        def tail_log():
            last_line = ""
            while not stop_event.is_set():
                tail_cmd = f"cd {self.code_path}/DeepTrace && tail -n 1 agent.log"
                output, error = self.execute_command(tail_cmd)
                if output and output.strip() != last_line:
                    last_line = output.strip()
                    progress_dict[self.agent_name] = last_line
                time.sleep(1)

        t_install = threading.Thread(target=run_install)
        t_log = threading.Thread(target=tail_log)
        t_log.start()
        t_install.start()
        t_install.join()
        stop_event.set()
        t_log.join()

        t2 = time.time()
        progress_dict[self.agent_name] = f"耗时 {t2 - t1:.2f} 秒"
        

    def get_pids(self):
        command = f"cd {self.code_path}/DeepTrace && echo {self.host_password} | sudo -S bash scripts/docker_pids.sh"
        output, error = self.execute_command(command)
        if error and 'error' in error:
            raise Exception(f"{self.agent_name}: 获取进程失败 {error}")
        else:
            print(f'{self.agent_name}: 获取进程成功')
            self.trace['pids'] = [
                int(pid) for pid in output.strip().split('\n')
                if pid.strip().isdigit()
            ]
            print(f'{self.agent_name}: 进程列表 {self.trace["pids"]}')
            return output

    def run(self):
        # RUST_LOG=info cargo run --release --config 'target."cfg(all())".runner="sudo -E"' -- &
        # command = f"cd {self.code_path}/DeepTrace && bash scripts/run.sh"
        command = f"cd {self.code_path}/DeepTrace && echo {self.host_password} | sudo -S bash scripts/run_agent.sh"
        output, error = self.execute_command(command)
        if error and 'error' in error:
            raise Exception(f"{self.agent_name}: 启动失败 {error}")
        else:
            print(f'{self.agent_name}: 启动成功 {output}')

    
    def stop(self):
        # pkill -f 'target/release/agent'
        command = f"cd {self.code_path}/DeepTrace && echo {self.host_password} | sudo -S bash scripts/stop_agent.sh"
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
                "workers": self.agent_info['workers'],
                "state_index": self.elastic_config['agent_status_index'],
                "name": self.agent_name
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
            "trace": {
                "pids": self.trace['pids']
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


