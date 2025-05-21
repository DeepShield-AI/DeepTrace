import paramiko
from scp import SCPClient

def scp_copy_directory(host, port, username, password, local_path, remote_path):
    """
    使用 SCP 将本地目录递归拷贝到远程服务器，并打印进度
    :param host: 远程服务器地址
    :param port: SSH 端口号
    :param username: SSH 用户名
    :param password: SSH 密码
    :param local_path: 本地目录路径
    :param remote_path: 远程目录路径
    """
    try:
        # 创建 SSH 客户端
        ssh = paramiko.SSHClient()
        ssh.set_missing_host_key_policy(paramiko.AutoAddPolicy())
        
        # 连接远程服务器
        ssh.connect(hostname=host, port=port, username=username, password=password)
        
        # 创建 SCP 客户端
        with SCPClient(ssh.get_transport(), progress=progress_callback) as scp:
            # 递归拷贝目录
            scp.put(local_path, remote_path, recursive=True)
        
        print(f"成功将目录 {local_path} 拷贝到 {host}:{remote_path}")
    except Exception as e:
        print(f"发生错误: {e}")
    finally:
        ssh.close()

def progress_callback(filename, size, sent):
    """
    进度回调函数，每拷贝 10 个文件打印一次
    :param filename: 当前文件名
    :param size: 文件总大小
    :param sent: 已发送的字节数
    """
    if sent == size:  # 文件传输完成
        progress_callback.file_count += 1
        if progress_callback.file_count % 10 == 0:
            print(f"已拷贝 {progress_callback.file_count} 个文件...")
progress_callback.file_count = 0  # 初始化文件计数器

# 示例用法
if __name__ == "__main__":
    host = "202.112.237.33"  # 远程服务器地址
    port = 22               # SSH 端口号
    username = "ubuntu"       # SSH 用户名
    password = "netsys204"   # SSH 密码
    local_path = "/root/gyt-file/DeepTrace"  # 本地目录路径
    remote_path = "/home/ubuntu/"  # 远程目录路径

    scp_copy_directory(host, port, username, password, local_path, remote_path)