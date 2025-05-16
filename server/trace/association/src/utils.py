import json
import re
import random
import copy
from elasticsearch import Elasticsearch

# 从外部配置文件读取密码
with open("../../../config/config.json", "r") as f:
    config = json.load(f)

ES_USERNAME = "elastic"
ES_PASSWORD = config.get("elastic_password")  





def random_64bit_id():
    # 生成 16 个十六进制字符（64 位）
    return ''.join(random.choice('0123456789abcdef') for _ in range(16))


class Span:
    def __init__(self, span_json):
        self.start_time = span_json.get("start_time")
        self.end_time = span_json.get("end_time")
        self.tgid = span_json.get("tgid")
        self.pid = span_json.get("pid")
        self.protocol = span_json.get("protocol")
        self.component_name = span_json.get("component_name")
        self.req_content = span_json.get("req_content")
        self.resp_content = span_json.get("resp_content")
        self.trace_id = span_json.get("trace_id")
        self.direction = span_json.get("direction")
        self.association_tracid = None
        self.src_ip = span_json.get("src_ip")
        self.dst_ip = span_json.get("dst_ip")
        self.src_port = span_json.get("src_port")
        self.dst_port = span_json.get("dst_port")
        self.direction = span_json.get("direction")
        self.duration = self.end_time - self.start_time

    def parse_traceid(self):
        """
        解析 trace_id
        """
        if self.protocol == "Thrift":
            if 'uber-trace-id' not in self.content:
                return
            parts = self.content.split("uber-trace-id")[-1]
            parts = parts.split(":")
            return parts[0].strip()
        if self.protocol == "HTTP1":
            if 'X-Request-ID' not in self.content:
                return
            
            pattern = r"X-Request-ID:\s*([a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12})"
            match = re.search(pattern, self.content)
            if match:
                request_id = match.group(1)
                return request_id
    def __str__(self):
        """
        返回 Span 的字符串表示
        """
        return (f"Span(start_time={self.start_time}, end_time={self.end_time}, "
                f"tgid={self.tgid}, pid={self.pid}, protocol={self.protocol}, component_name={self.component_name}), "
                f"trace_id={self.trace_id}, duration={self.duration}, direction={self.direction}) ")


def parse_json(file_path):

    # print(f"Parsing JSON file: {file_path}")
    with open(file_path, "r", encoding="utf-8") as file:
        data = json.load(file)

    # 获取 spans 列表
    spans = data.get("spans", [])

    span_map = {} # component:{incoming:[], outgoing:[]}

    # 遍历并打印每个 span 的请求和响应信息
    count = 0
    for span in spans:
        count += 1
        # if count > 10000:
        #     break
        req = span.get("req", {})
        resp = span.get("resp", {})
        if req['protocol'] not in ['Thrift', 'HTTP1']:
            continue
        start_time = req.get("timestamp_ns")
        end_time = resp.get("timestamp_ns")
        tgid = req.get("tgid")
        pid = req.get("pid")
        protocol = req.get("protocol")
        component_name = req.get("comm").split("\\")[0]
        content = req.get("payload")
        dstip = req['quintuple']['dst_addr']
        direction = req.get("direction")
        span = Span(start_time, end_time, tgid, pid, protocol, component_name, content, direction)
        if tgid not in span_map:
            span_map[tgid] = {"incoming": {}, "outgoing": {}}
        if req.get("direction") == "Ingress":
            if dstip not in span_map[tgid]["incoming"]:
                span_map[tgid]["incoming"][dstip] = []
            span_map[tgid]["incoming"][dstip].append(span)
        elif req.get("direction") == "Egress":
            if dstip not in span_map[tgid]["outgoing"]:
                span_map[tgid]["outgoing"][dstip] = []
            span_map[tgid]["outgoing"][dstip].append(span)
        else:
            print(f"Unknown direction: {req.get('direction')}")
            continue
    return span_map
    
def service_acc(spans):
    service_acc = {}
    for tgid, span_list in spans.items():
        right_count = 0
        for span in span_list:
            if span.association_tracid is not None:
                if span.association_tracid == span.trace_id:
                    right_count += 1

        service_acc[tgid] = right_count / len(span_list)
    return service_acc 



def e2e_acc(spans):

    trace_acc = {}
    for tgid, span_list in spans.items():
        for span in span_list:
            if span.trace_id not in trace_acc:
                trace_acc[span.trace_id] = 1
            if span.association_tracid is None:
                trace_acc[span.trace_id] = 0
            if span.association_tracid != span.trace_id:
                trace_acc[span.trace_id] = 0
    
    return sum(trace_acc.values()) / len(trace_acc)
    

def read_all_spans(index_name):
    """
    从 Elasticsearch 中读取指定索引的所有 span 数据
    """
    # 连接到 Elasticsearch
    es = Elasticsearch(hosts=["http://127.0.0.1:9200"], basic_auth=(ES_USERNAME, ES_PASSWORD))

    # 查询所有文档
    query = {
        "query": {
            "match_all": {}  # 匹配所有文档
        },
        "size": 10000  # 设置返回的文档数量
    }

    # 执行查询
    response = es.search(index=index_name, body=query)

    # 提取 span 数据
    spans = []
    for hit in response["hits"]["hits"]:
        spans.append(hit["_source"])  # _source 字段包含实际的文档数据
    span_map = {}
    for span in spans:
    
        span_obj = Span(span)
        if span_obj.protocol not in ['Thrift', 'HTTP1']:
            continue
        # print(span_obj)
        if span_obj.tgid not in span_map:
            span_map[span_obj.tgid] = {"incoming": {}, "outgoing": {}}
        if span_obj.direction == "Ingress":
            if span_obj.dst_ip not in span_map[span_obj.tgid]["incoming"]:
                span_map[span_obj.tgid]["incoming"][span_obj.dst_ip] = []
            span_map[span_obj.tgid]["incoming"][span_obj.dst_ip].append(span_obj)
        elif span_obj.direction == "Egress":
            if span_obj.dst_ip not in span_map[span_obj.tgid]["outgoing"]:
                span_map[span_obj.tgid]["outgoing"][span_obj.dst_ip] = []
            span_map[span_obj.tgid]["outgoing"][span_obj.dst_ip].append(span_obj)
        else:
            continue
    
    span_map_copy = {}
    for tgid, span_tgid in span_map.items():
        if len(span_tgid['incoming']) == 0 or len(span_tgid['outgoing']) == 0:
            continue
        
        span_map_copy[tgid] = {"incoming": {}, "outgoing": {}}
        trace_id_count = {}
        for direction, span_list in span_tgid.items():
            for ip in span_list:
                span_map_copy[tgid][direction][ip] = []
                for span in span_list[ip]:
                    if span.trace_id not in trace_id_count:
                        trace_id_count[span.trace_id] = 1
                    else:
                        trace_id_count[span.trace_id] += 1
        for direction, span_list in span_tgid.items():
            for ip in span_list:
                for span in span_list[ip]:
                    if trace_id_count[span.trace_id] == 3:
                        span_map_copy[tgid][direction][ip].append(span)

    # 预处理掉 trace_id_count != 3 的 span
    # print(trace_id_count)
    # span_map_copy = copy.deepcopy(span_map)
    # for tgid, span in span_map.items():
    #     if len(span['incoming']) == 0 or len(span['outgoing']) == 0:
    #         continue
    #     for direction, span_list in span.items():
    #         for ip in span_list:
    #             for span in span_list[ip]:
    #                 if trace_id_count[span.trace_id] != 3:
    #                     span_map_copy[tgid][direction][ip].remove(span)

    return span_map_copy


