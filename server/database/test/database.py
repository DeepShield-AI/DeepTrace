
import json
import re
from elasticsearch import Elasticsearch

import json

# 从外部配置文件读取密码
with open("../../config/config.json", "r") as f:
    config = json.load(f)

ES_USERNAME = "elastic"
ES_PASSWORD = config.get("elastic_password")  


def parse_traceid(protocol, content):
    """
    解析 trace_id
    """
    if protocol == "Thrift":
        if 'uber-trace-id' not in content:
            return
        parts = content.split("uber-trace-id")[-1]
        parts = parts.split(":")
        return parts[0].strip()
    if protocol == "HTTP1":
        if 'X-Request-ID' not in content and 'X-Request-Id' not in content:
            return
        
        pattern = r"X-Request-ID:\s*([a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12})"
        match = re.search(pattern, content)
        if match:
            request_id = match.group(1)
            return request_id
        pattern = r"X-Request-Id:\s*([a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12})"
        match = re.search(pattern, content)
        if match:
            request_id = match.group(1)
            return request_id
        return ''

def parse_json(file_path):
    """
    解析 JSON 文件并返回 span_list
    """
    with open(file_path, "r", encoding="utf-8") as file:
        data = json.load(file)

    spans = data.get("spans", [])
    span_list = []

    for span in spans:
        req = span.get("req", {})
        resp = span.get("resp", {})
        if req['protocol'] not in ['Thrift', 'HTTP1']:
            continue
        tgid = req.get("tgid")
        pid = req.get("pid")
        protocol = req.get("protocol")
        start_time = req.get("timestamp_ns")
        end_time = resp.get("timestamp_ns")
        component_name = req.get("comm").split("\\")[0]
        req_content = req.get("payload")
        resp_content = resp.get("payload")
        dstip = req['quintuple']['dst_addr']
        srcip = req['quintuple']['src_addr']
        src_port = req['quintuple']['src_port']
        dst_port = req['quintuple']['dst_port']
        direction = req.get("direction")
        trace_id = parse_traceid(protocol, req_content)
        span_list.append({
            "tgid": tgid,
            "pid": pid,
            "component_name": component_name,
            "protocol": protocol,
            "direction": direction,
            "srcip": srcip,
            "src_port": src_port,
            "dstip": dstip,
            "dst_port": dst_port,
            "start_time": start_time,
            "end_time": end_time,
            "req_content": req_content,
            "resp_content": resp_content,
            "trace_id": trace_id
        })
    return span_list

def write_to_elasticsearch(index_name, span_list):
    """
    将 span_list 写入 Elasticsearch
    """
    es = Elasticsearch(
        hosts=["http://127.0.0.1:9200"],
        basic_auth=(ES_USERNAME, ES_PASSWORD)  # 添加用户名和密码
    )

    for span in span_list:
        es.index(index=index_name, document=span)

def write_all():
    for rps in [100, 200, 300, 400, 500]:
    # for rps in [100]:
        file_path = f"./spans/rps{rps}-spans.json"
        spans = parse_json(file_path)
        write_to_elasticsearch(f'rps-{rps}-spans', spans)

def clear_all():
    es = Elasticsearch(
        hosts=["http://127.0.0.1:9200"],
        basic_auth=(ES_USERNAME, ES_PASSWORD)  # 添加用户名和密码
    )
    for index in es.indices.get_alias(index="*"):
        if index.startswith("rps-"):
            es.indices.delete(index=index)

if __name__ == "__main__":
    clear_all()
    write_all()



