import time
def log(msg, level="INFO"):
    """
    简单日志函数，写入本地 deeptrace_server.log 文件
    """
    now = time.strftime("%Y-%m-%d %H:%M:%S", time.localtime())
    with open("deeptrace_server.log", "a") as f:
        f.write(f"{now} [{level}] {msg}\n")