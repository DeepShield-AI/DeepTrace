



docker exec -it deeptrace_server python -m cli.src.cmd agent stop

# docker exec -it deeptrace_server python -m cli.src.cmd agent uninstall_app


cd ./server
# 停止并移除容器
sudo docker-compose down

# 清空数据、日志、插件目录
sudo rm -rf /user/share/es/data
sudo rm -rf /user/share/es/logs
sudo rm -rf /user/share/es/plugins

echo "Elasticsearch 相关目录已清空并重建。"
