#!/bin/bash

cd ./server

# 创建 Python 虚拟环境
python3 -m venv venv

# 激活虚拟环境
source venv/bin/activate

# 升级 pip
pip install --upgrade pip

# 安装依赖包
pip install -r requirements.txt

echo "虚拟环境创建并依赖安装完成。"