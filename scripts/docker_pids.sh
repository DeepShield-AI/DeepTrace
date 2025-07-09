#!/bin/bash

# 检查是否安装了 Docker
if ! command -v docker &> /dev/null; then
    echo "Docker 未安装，请先安装 Docker。"
    exit 1
fi

# 获取所有运行中的容器 ID
container_ids=$(docker ps -q)

# 检查是否有运行中的容器
if [ -z "$container_ids" ]; then
    echo "当前没有运行中的容器。"
    exit 0
fi

# 遍历每个容器，获取其 PID
for container_id in $container_ids; do
    # 获取容器名称
    name=$(docker inspect --format '{{.Name}}' "$container_id" | sed 's|/||')
    # 如果名称包含 wrk、jaeger、deeptrace、elastic、kibana，则跳过
    if [[ "$name" == *wrk* ]] || [[ "$name" == *jaeger* ]] || [[ "$name" == *deeptrace* ]] || [[ "$name" == *elastic* ]] || [[ "$name" == *kibana* ]]; then
        continue
    fi
    # 获取容器的 PID
    pid=$(docker inspect --format '{{.State.Pid}}' "$container_id")
    echo "$pid"
done