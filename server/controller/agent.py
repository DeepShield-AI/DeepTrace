import json
import paramiko
import os
from utils import *
from scp import SCPClient
import tarfile  # 用于压缩文件

agents = {}

# 管理agent状态：alive、处理延迟、条目、
# 用户管理：agent属于哪个用户
# 数据管理：数据属于哪个agent （重点
# 分析引擎管理：关联、基于trace做分析、tag、沈总的ProfileMap （重点
    # 分成两组：trace、ProfileMap、志衡的
# api管理：web
# tag管理

class Agent:
    def __init__(self, info):
        self.agent_name = info['agent_name']
        self.ip = info['ip']
        self.user_name = info['user_name']
        self.password = info['password']
        self.ssh_port = info.get('ssh_port', 22)
        self.deeptrace_port = info.get('deeptrace_port', 13701)
        self.ssh_client = None
        self.code_path = info.get('code_path', '~/')

    def connect(self):
        if not self.ssh_client:
            self.ssh_client = paramiko.SSHClient()
            self.ssh_client.set_missing_host_key_policy(paramiko.AutoAddPolicy())
            self.ssh_client.connect(
                hostname=self.ip, port=self.ssh_port, username=self.user_name, password=self.password
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

    def sync_config(self):
        pass


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

            print(f"代码仓库已克隆到 {self.agent_name} 的 {self.code_path}DeepTrace")
        except Exception as e:
            print(f"克隆代码到 {self.agent_name} 失败: {str(e)}")
    
    def install_dependencies(self):
        command = f"cd {self.code_path}DeepTrace/deployment && bash install.sh {self.password}"
        output, error = self.execute_command(command)
        if error and 'Updating crates.io index' not in error:
            raise Exception(f"{self.agent_name}: 安装依赖失败 {error}")
        else:
            print(f'{self.agent_name}: 安装依赖成功')

    def compile_code(self):
        command = f"cd {self.code_path}DeepTrace/deployment && bash compile.sh"
        output, error = self.execute_command(command)
        if error and 'profile [optimized] target(s)' not in error:
            raise Exception(f"{self.agent_name}: 编译失败 {error}")
        else:
            print(f'{self.agent_name}: 编译成功')

    def run_agent(self):
        command = f"cd {self.code_path}DeepTrace/deployment && bash run.sh"
        output, error = self.execute_command(command)
        if error and 'error' in error:
            raise Exception(f"{self.agent_name}: 启动失败 {error}")
        else:
            print(f'{self.agent_name}: 启动成功')
    
    def stop_agent(self):
        pass

