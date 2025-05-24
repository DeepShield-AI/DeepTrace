import subprocess
import time
import re
import json
import toml


with open("../config/config.toml", "r") as f:
    config = toml.load(f)
try:
    elastic_pwd = config.get("elastic", {}).get("elastic_password")
    kibana_pwd = config.get("elastic", {}).get("kibana_password")
except KeyError as e:
    raise KeyError(f"请设置elastic_password和kibana_password: {e}")

def run(cmd, capture_output=False):
    print(f"运行命令: {cmd}")
    result = subprocess.run(cmd, shell=True, capture_output=capture_output, text=True)
    if capture_output:
        return result.stdout.strip()
    return None

# 创建目录并设置权限
run("sudo mkdir -p /user/share/es/data")
run("sudo mkdir -p /user/share/es/config")
run("sudo mkdir -p /user/share/es/plugins")
run("chmod 777 -R ./")

# 安装 elasticsearch Python 客户端
run("pip3 install elasticsearch==8.7.0")

# 拉取镜像
run("sudo docker pull docker.elastic.co/elasticsearch/elasticsearch:8.7.0")
run("sudo docker pull docker.elastic.co/kibana/kibana:8.7.0")



# 加载镜像
# run("sudo docker load -i ./images/elasticsearch817.tar")
# run("sudo docker load -i ./images/kibana817.tar")

# 替换密码

compose_file = "docker-compose.yaml"
with open(compose_file, "r", encoding="utf-8") as f:
    content = f.read()

# 替换 ELASTICSEARCH_PASSWORD 的值
content_new = re.sub(
    r'(ELASTICSEARCH_PASSWORD=)[^\n]+',
    rf'\1{kibana_pwd}',
    content
)

with open(compose_file, "w", encoding="utf-8") as f:
    f.write(content_new)

# 启动服务
run("sudo docker-compose -f docker-compose.yaml up -d")

# 等待 ES 启动
print("等待 Elasticsearch 启动...")
time.sleep(20)

# 自动生成 elastic 密码
cmd = "yes | sudo docker exec -i es /usr/share/elasticsearch/bin/elasticsearch-reset-password -u elastic --auto"
output = run(cmd, capture_output=True)
elastic_pass = ""
for line in output.splitlines():
    if "New value:" in line:
        elastic_pass = line.split("New value:")[-1].strip()
        break

if not elastic_pass:
    raise RuntimeError("未能获取 elastic 用户的自动生成密码")

# 用新密码修改为你想要的密码
run(f"""curl -u elastic:{elastic_pass} -X POST "http://localhost:9200/_security/user/elastic/_password" \
  -H "Content-Type: application/json" \
  -d '{{"password":"{elastic_pwd}"}}'""")

# 自动生成 kibana_system 密码
cmd = "yes | sudo docker exec -i es /usr/share/elasticsearch/bin/elasticsearch-reset-password -u kibana --auto"
output = run(cmd, capture_output=True)
kibana_pass = ""
for line in output.splitlines():
    if "New value:" in line:
        kibana_pass = line.split("New value:")[-1].strip()
        break

if not kibana_pass:
    raise RuntimeError("未能获取 kibana 用户的自动生成密码")

# 用新密码修改为你想要的密码
run(f"""curl -u kibana:{kibana_pass} -X POST "http://localhost:9200/_security/user/kibana/_password" \
  -H "Content-Type: application/json" \
  -d '{{"password":"{kibana_pwd}"}}'""")

print("Elasticsearch 和 Kibana 密码已自动设置完成！")