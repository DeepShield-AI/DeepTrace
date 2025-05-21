import subprocess
import shutil
import os

# 停止并移除容器
subprocess.run("sudo docker-compose -f docker-compose.yaml down", shell=True, check=True)

# 清空数据、日志、插件目录
shutil.rmtree("/user/share/es/data", ignore_errors=True)
shutil.rmtree("/user/share/es/logs", ignore_errors=True)
shutil.rmtree("/user/share/es/plugins", ignore_errors=True)



print("Elasticsearch 相关目录已清空并重建。")