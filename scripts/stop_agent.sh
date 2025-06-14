#!/bin/bash

# 查找包含 ./target/release/agent 的进程并杀掉
pids=$(pgrep -f "./target/release/agent")
if [ -n "$pids" ]; then
    echo "Killing agent process(es): $pids"
    sudo kill -9 $pids
else
    echo "No agent process found."
fi