import json
import time
from elasticsearch import Elasticsearch
from elasticsearch.helpers import bulk

# 从外部配置文件读取密码
with open("../../../config/config.json", "r") as f:
    config = json.load(f)

ES_USERNAME = "elastic"
ES_PASSWORD = config.get("elastic_password")  
SERVER_IP = config.get("server_ip") 



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
        self.duration = self.end_time - self.start_time
        self.span_id = span_json.get('span_id')
        self.parent_id = span_json.get('parent_id')



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



# 这里的all_spans是列表: tgid -> direction -> ip -> span_list
def es_write_traces(index_name, traces):
    es = Elasticsearch(
        hosts=[f"http://{SERVER_IP}:9200"],
        basic_auth=(ES_USERNAME, ES_PASSWORD)  # 添加用户名和密码
    )

    # 准备 bulk 数据
    actions = [
        {
            "_index": index_name,
            "_source": trace
        }
        for trace in traces
    ]

    # 执行批量写入
    success, _ = bulk(es, actions)