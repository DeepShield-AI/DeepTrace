import json
import time
from elasticsearch import Elasticsearch
from elasticsearch.helpers import bulk
import toml

# 从外部配置文件读取密码
with open("../../../config/config.toml", "r") as f:
    config = toml.load(f)

ES_USERNAME = "elastic"
ES_PASSWORD = config.get("elastic", {}).get("elastic_password") 
SERVER_IP = "0.0.0.0"



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
        self.src_ip = span_json.get("src_ip")
        self.dst_ip = span_json.get("dst_ip")
        self.src_port = span_json.get("src_port")
        self.dst_port = span_json.get("dst_port")
        self.direction = span_json.get("direction")
        self.duration = span_json.get("duration")
        self.req_size = span_json.get("req_size")
        self.resp_size = span_json.get("resp_size")
        self.span_id = span_json.get('span_id')
        self.endpoint = span_json.get('endpoint')
        self.parent_id = None



    def __str__(self):
        """
        返回 Span 的字符串表示
        """
        return (f"Span(start_time={self.start_time}, end_time={self.end_time}, "
                f"tgid={self.tgid}, pid={self.pid}, protocol={self.protocol}, component_name={self.component_name}), "
                f"trace_id={self.trace_id}, duration={self.duration}, direction={self.direction}) ")
    def tojson(self):
        """
        将 Span 的所有成员变量动态转换为 JSON 格式
        """
        return {attr: getattr(self, attr) for attr in self.__dict__}



# 任意被调用者 A 收到的入请求与触发的A调用特定组件B的出请求之间的关联准确率
def pair_acc(spans):
    spanid2traceid = {}
    pair_acc = {}
    for tgid, span_list in spans.items():
        for direction, span_list in span_list.items():
            for ip in span_list:
                for span in span_list[ip]:
                    spanid2traceid[span.span_id] = span.trace_id
                    
    for tgid, tgid_spans in spans.items():
        if len(tgid_spans['incoming']) == 0 or len(tgid_spans['outgoing']) == 0:
            continue
        if tgid not in pair_acc:
            pair_acc[tgid] = {}
        outgoing_spans = tgid_spans['outgoing']
        for outgoing_ip, outgoing_span_list in outgoing_spans.items():
            if outgoing_ip not in pair_acc[tgid]:
                pair_acc[tgid][outgoing_ip] = 0
            for outgoing_span in outgoing_span_list:
                if outgoing_span.parent_id is not None:
                    if outgoing_span.parent_id in spanid2traceid:
                        if spanid2traceid[outgoing_span.parent_id] == outgoing_span.trace_id:
                            pair_acc[tgid][outgoing_ip] += 1
            pair_acc[tgid][outgoing_ip] = pair_acc[tgid][outgoing_ip] / len(outgoing_span_list)
    return pair_acc

# 计算每个组件所有的父子span的关联准确率
def service_acc(spans):
    spanid2traceid = {}
    svc_acc = {}
    for tgid, span_list in spans.items():
        for direction, span_list in span_list.items():
            for ip in span_list:
                for span in span_list[ip]:
                    spanid2traceid[span.span_id] = span.trace_id
    for tgid, tgid_spans in spans.items():
        if len(tgid_spans['incoming']) == 0 or len(tgid_spans['outgoing']) == 0:
            continue
        if tgid not in svc_acc:
            svc_acc[tgid] = 0
        outgoing_spans = tgid_spans['outgoing']
        count = 0
        for outgoing_ip, outgoing_span_list in outgoing_spans.items():
            count += len(outgoing_span_list)
            for outgoing_span in outgoing_span_list:
                if outgoing_span.parent_id is not None:
                    if outgoing_span.parent_id in spanid2traceid:
                        if spanid2traceid[outgoing_span.parent_id] == outgoing_span.trace_id:
                            svc_acc[tgid] += 1
        svc_acc[tgid] = svc_acc[tgid] / count
    return svc_acc 



def e2e_acc(spans):

    trace_acc = {}
    spanid2traceid = {}
    for tgid, span_list in spans.items():
        for direction, span_list in span_list.items():
            for ip in span_list:
                for span in span_list[ip]:
                    spanid2traceid[span.span_id] = span.trace_id
    for tgid, tgid_spans in spans.items():
        if len(tgid_spans['incoming']) == 0 or len(tgid_spans['outgoing']) == 0:
            continue
        outgoing_spans = tgid_spans['outgoing']
        for outgoing_ip, outgoing_spans in tgid_spans['outgoing'].items():
            for outgoing_span in outgoing_spans:
                if outgoing_span.trace_id not in trace_acc:
                    trace_acc[outgoing_span.trace_id] = 1
                if outgoing_span.parent_id is None:
                    trace_acc[outgoing_span.trace_id] = 0
                    continue
                if outgoing_span.parent_id not in spanid2traceid:
                    trace_acc[outgoing_span.trace_id] = 0
                    continue
                if spanid2traceid[outgoing_span.parent_id] != outgoing_span.trace_id:
                    trace_acc[outgoing_span.trace_id] = 0
                    continue

    return sum(trace_acc.values()) / len(trace_acc)
    

def es_read_spans(index_name):
    """
    从 Elasticsearch 中读取指定索引的所有 span 数据
    """
    t1 = time.time()
    # 连接到 Elasticsearch
    es = Elasticsearch(hosts=[f"http://{SERVER_IP}:9200"], basic_auth=(ES_USERNAME, ES_PASSWORD))

    # 查询所有文档
    query = {
        "query": {
            "match_all": {}  # 匹配所有文档
        },
        "size": 10000  # 设置返回的文档数量
    }

    # 执行查询
    response = es.search(index=index_name, body=query)

    t2 = time.time()
    print(f"Read all spans from {index_name} took {t2 - t1:.2f} seconds")

    # 提取 span 数据
    spans = []
    for hit in response["hits"]["hits"]:
        spans.append(hit["_source"])  # _source 字段包含实际的文档数据
    span_class_list = []
    for span in spans:
        span_obj = Span(span)
        span_class_list.append(span_obj)
    return span_class_list


# 在执行组件内关联之前对database中读到的span进行预处理
def intra_preprocess(spans):

    span_map = {}
    for span in spans:
        if span.protocol not in ['Thrift', 'HTTP1']:
            continue
        # print(span_obj)
        if span.tgid not in span_map:
            span_map[span.tgid] = {"incoming": {}, "outgoing": {}}
        if span.direction == "Ingress":
            if span.dst_ip not in span_map[span.tgid]["incoming"]:
                span_map[span.tgid]["incoming"][span.dst_ip] = []
            span_map[span.tgid]["incoming"][span.dst_ip].append(span)
        elif span.direction == "Egress":
            if span.dst_ip not in span_map[span.tgid]["outgoing"]:
                span_map[span.tgid]["outgoing"][span.dst_ip] = []
            span_map[span.tgid]["outgoing"][span.dst_ip].append(span)
        else:
            continue
    return span_map
    # span_map_copy = {}
    # for tgid, span_tgid in span_map.items():
    #     if len(span_tgid['incoming']) == 0 or len(span_tgid['outgoing']) == 0:
    #         continue
        
    #     span_map_copy[tgid] = {"incoming": {}, "outgoing": {}}
    #     trace_id_count = {}
    #     for direction, span_list in span_tgid.items():
    #         for ip in span_list:
    #             span_map_copy[tgid][direction][ip] = []
    #             for span in span_list[ip]:
    #                 if span.trace_id not in trace_id_count:
    #                     trace_id_count[span.trace_id] = 1
    #                 else:
    #                     trace_id_count[span.trace_id] += 1
    #     for direction, span_list in span_tgid.items():
    #         for ip in span_list:
    #             for span in span_list[ip]:
    #                 if trace_id_count[span.trace_id] == 3:
    #                     span_map_copy[tgid][direction][ip].append(span)

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
    # return span_map_copy



# 这里的all_spans是列表: tgid -> direction -> ip -> span_list
def es_write_spans(index_name, all_spans):
    """
    将 spans 写入 Elasticsearch，使用 bulk API 实现批量传输
    """
    es = Elasticsearch(
        hosts=[f"http://{SERVER_IP}:9200"],
        basic_auth=(ES_USERNAME, ES_PASSWORD)  # 添加用户名和密码
    )

    spans = []
    for tgid, tgid_spans in all_spans.items():
        for direction, span_list in tgid_spans.items():
            for ip in span_list:
                for span in span_list[ip]:
                    spans.append(span)

    # 准备 bulk 数据
    actions = [
        {
            "_index": index_name,
            "_source": span.tojson()
        }
        for span in spans
    ]

    # 执行批量写入
    success, _ = bulk(es, actions)
