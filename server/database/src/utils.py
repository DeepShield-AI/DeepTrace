import json
import re
from elasticsearch import Elasticsearch, helpers
import time
import toml
import random
import string
from config.parse_config import read_db_config
from trace.model.span import Span


ES_USERNAME = "elastic"
ES_PASSWORD, _ = read_db_config()
SERVER_IP = "es"



def es_write_span_list(index_name, span_list):
    """
    批量写入 span_list 到 Elasticsearch
    """
    es = Elasticsearch(
        hosts=[f"http://{SERVER_IP}:9200"],
        basic_auth=(ES_USERNAME, ES_PASSWORD)
    )
    actions = [
        {
            "_index": index_name,
            "_source": span
        }
        for span in span_list
    ]
    helpers.bulk(es, actions)

def es_clear_index(index_name):
    t1 = time.time()
    es = Elasticsearch(
        hosts=[f"http://{SERVER_IP}:9200"],
        basic_auth=(ES_USERNAME, ES_PASSWORD)  # 添加用户名和密码
    )
    for index in es.indices.get_alias(index="*"):
        if index == index_name:
            es.indices.delete(index=index)
    t2 = time.time()

def es_read_span_list(index_name):
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


def es_read_agent_span_list(agents):
    """
    从 Elasticsearch 中读取指定索引的所有 span 数据
    """
    t1 = time.time()
    all_spans = []
    # 连接到 Elasticsearch
    for agent_name, agent in agents.items():
        index_name = agent.sender['index_name']
        spans = es_read_span_list(index_name)
        all_spans.extend(spans)
    t2 = time.time()
    print(f"Read all spans from agents took {t2 - t1:.2f} seconds")
    return all_spans


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
    success, _ = helpers.bulk(es, actions)