#!/bin/bash
set -euo pipefail

DOCKER_CONFIG="/etc/docker/daemon.json"
TARGET_REGISTRY="47.97.67.233:5000"
TEMP_FILE=$(mktemp)

sudo apt-get update
sudo apt-get install -y jq docker.io docker-compose

if [ ! -f "$DOCKER_CONFIG" ] || [ ! -s "$DOCKER_CONFIG" ]; then
    echo "{}" | sudo tee "$DOCKER_CONFIG" >/dev/null
    echo "Created new $DOCKER_CONFIG"
fi

if ! jq empty "$DOCKER_CONFIG" &>/dev/null; then
    echo "Error: Invalid JSON in $DOCKER_CONFIG" >&2
    exit 1
fi

if jq -e --arg registry "$TARGET_REGISTRY" '
    .["insecure-registries"] // [] | index($registry)
' "$DOCKER_CONFIG" >/dev/null; then
    echo "Registry $TARGET_REGISTRY is already configured in $DOCKER_CONFIG"
    echo "No Docker restart needed."
else
    echo "Registry $TARGET_REGISTRY is not present, adding..."
    jq --arg registry "$TARGET_REGISTRY" '
        .["insecure-registries"] = (
            (.["insecure-registries"] // []) + [$registry] | unique
        )
    ' "$DOCKER_CONFIG" > "$TEMP_FILE" && sudo mv "$TEMP_FILE" "$DOCKER_CONFIG"
    echo "Reloading and restarting Docker due to config changes..."
    sudo systemctl daemon-reload
    sudo systemctl restart docker
    echo "Docker restarted."
fi

# 读取 elastic 和 kibana 密码
CONFIG_FILE="./server/config/config.toml"
ELASTIC_PWD=$(grep 'elastic_password' $CONFIG_FILE | head -n 1 | sed 's/.*= *"\(.*\)".*/\1/')
KIBANA_PWD=$(grep 'kibana_password' $CONFIG_FILE | head -n 1 | sed 's/.*= *"\(.*\)".*/\1/')

COMPOSE_FILE="./server/docker-compose.yaml"

# 创建目录并设置权限
sudo mkdir -p /user/share/es/data
sudo mkdir -p /user/share/es/config
sudo mkdir -p /user/share/es/plugins
chmod 777 -R ./


# 替换 docker-compose.yaml 中 ELASTICSEARCH_PASSWORD
sed -i "s/ELASTICSEARCH_PASSWORD=.*/ELASTICSEARCH_PASSWORD=${ELASTIC_PWD}/" $COMPOSE_FILE

# 启动服务
cd ./server
docker-compose up -d

# 等待 ES 启动
echo "等待 Elasticsearch 启动..."
sleep 20

# 自动生成 elastic 密码，循环直到获取成功
max_retry=60
retry_interval=5
elastic_pass=""
for ((i=1;i<=max_retry;i++)); do
    echo "第${i}次尝试获取 elastic 用户的自动生成密码..."
    set +e
    output=$(yes | sudo docker exec -i es /usr/share/elasticsearch/bin/elasticsearch-reset-password -u elastic --auto 2>&1)
    elastic_pass=$(echo "$output" | grep "New value:" | awk -F'New value:' '{print $2}' | xargs)
    if [[ -n "$elastic_pass" ]]; then
        echo "debug $elastic_pass"
        break
    fi
    echo "未能获取 elastic 用户的自动生成密码，第${i}次重试，等待${retry_interval}秒..."
    sleep $retry_interval
done

if [[ -z "$elastic_pass" ]]; then
    echo "未能获取 elastic 用户的自动生成密码，已重试多次仍失败"
    exit 1
fi
echo "获取到的 elastic 用户密码: $elastic_pass"

# 用新密码修改为你想要的密码
curl -u elastic:"$elastic_pass" -X POST "http://localhost:9200/_security/user/elastic/_password" \
    -H "Content-Type: application/json" \
    -d "{\"password\":\"${ELASTIC_PWD}\"}"

echo "elasticsearch 成功设置密码为: $ELASTIC_PWD"

# 自动生成 kibana_system 密码
kibana_pass=""
for ((i=1;i<=max_retry;i++)); do
    output=$(yes | sudo docker exec -i es /usr/share/elasticsearch/bin/elasticsearch-reset-password -u kibana --auto 2>&1)
    kibana_pass=$(echo "$output" | grep "New value:" | awk -F'New value:' '{print $2}' | xargs)
    if [[ -n "$kibana_pass" ]]; then
        break
    fi
    echo "未能获取 kibana 用户的自动生成密码，第${i}次重试，等待${retry_interval}秒..."
    sleep $retry_interval
done

if [[ -z "$kibana_pass" ]]; then
    echo "未能获取 kibana 用户的自动生成密码，已重试多次仍失败"
    exit 1
fi

# 用新密码修改为你想要的密码
curl -u kibana:"$kibana_pass" -X POST "http://localhost:9200/_security/user/kibana/_password" \
    -H "Content-Type: application/json" \
    -d "{\"password\":\"${KIBANA_PWD}\"}"

echo "Elasticsearch 和 Kibana 密码已自动设置完成！"