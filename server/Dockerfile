# 使用官方 Python 基础镜像
FROM docker.1ms.run/python:3.12-slim

# 设置工作目录
WORKDIR /app

# 复制项目文件到容器
COPY . /app

# 安装依赖
RUN pip install --upgrade pip -i https://pypi.tuna.tsinghua.edu.cn/simple \
    && pip install -r requirements.txt -i https://pypi.tuna.tsinghua.edu.cn/simple



# 默认启动命令（可根据实际情况修改）
CMD ["tail", "-f", "/dev/null"]