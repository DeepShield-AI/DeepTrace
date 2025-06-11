import json
import re
from elasticsearch import Elasticsearch, helpers
import time
import toml
import random
import string
from database.src.utils import *
import os



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
    
def parse_endpoint(protocol, content):
    if protocol == "HTTP1":
        pattern = r"(?<=Host:\s)([^\r\n]+)"
        match = re.search(pattern, content)
        if match:
            host = match.group(1)
            return host
        return ''
    return ''

def generate_spanid():
    """
    生成随机的64位ID
    """
    return ''.join(random.choices(string.ascii_letters + string.digits, k=64))

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
            "src_ip": srcip,
            "src_port": src_port,
            "dst_ip": dstip,
            "dst_port": dst_port,
            "start_time": start_time,
            "end_time": end_time,
            "req_content": req_content,
            "resp_content": resp_content,
            "trace_id": trace_id,
            "span_id": generate_spanid(),
            "parent_id": None,
            "duration": end_time - start_time,
            "req_size": len(req_content) if req_content else 0,
            "resp_size": len(resp_content) if resp_content else 0,
            "endpoint": parse_endpoint(protocol, req_content),
        })
    return span_list



def write_all():
    for rps in [100, 200, 300, 400, 500]:
    # for rps in [100]:
        t1 = time.time()
        span_dir = os.path.dirname(os.path.abspath(__file__))
        span_file = os.path.join(span_dir, f"spans/rps{rps}-spans.json")
        spans = parse_json(span_file)
        es_write_span_list(f'test-rps-{rps}-spans', spans)
        t2 = time.time()
        print(f"rps {rps} write done time: {t2-t1}")

def clear_all():
    for rps in [100, 200, 300, 400, 500]:
        es_clear_index(f'test-rps-{rps}-spans')
        es_clear_index(f'test-rps-{rps}-mappings')
        es_clear_index(f'test-rps-{rps}-traces')
        print(f"rps {rps} clear done")

def es_init_test_data():
    clear_all()
    write_all()

if __name__ == "__main__":
    clear_all()
    write_all()



