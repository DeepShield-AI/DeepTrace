from log.utils import log
from elasticsearch import Elasticsearch, helpers
import time
from config.parse_config import read_db_config
from trace.model.span import Span


ES_USERNAME = "elastic"
# ES_PASSWORD, SERVER_IP = read_db_config()
# SERVER_IP = "es"



def es_write_agent_config(agent_config, elastic_config, server_config):
    ES_PASSWORD, SERVER_IP = read_db_config()
    try:
        # 准备要写入的数据
        # print(f"{agent_name}: 准备写入 Elasticsearch 配置")
        agent_name = agent_config['agent_info']['agent_name']
        agent_data = {
            "agent_config": agent_config,
            "elastic_config": elastic_config,
            "server_config": server_config
        }

        # 初始化 Elasticsearch 客户端
        es_client = Elasticsearch(
            hosts=[f"http://{SERVER_IP}:{elastic_config['port']}"],
            basic_auth=("elastic", elastic_config['elastic_password'])
        )

        # 索引名称
        index_name = "agent_list"

        # 检查索引是否存在
        if not es_client.indices.exists(index=index_name):
            # 如果索引不存在，则创建索引
            es_client.indices.create(index=index_name)
            # print(f"{agent_name}: 索引 {index_name} 已创建")

        # 查询是否存在指定 agent_name 的条目
        query = {
            "query": {
                "term": {
                    "agent_info.agent_name.keyword": agent_name
                }
            }
        }
        search_response = es_client.search(index=index_name, body=query)

        if search_response['hits']['total']['value'] > 0:
            # 如果存在，获取文档 ID 并更新
            doc_id = search_response['hits']['hits'][0]['_id']
            response = es_client.update(index=index_name, id=doc_id, body={"doc": agent_data})
            print(f"{agent_name}: 配置更新到 Elasticsearch")
        else:
            # 如果不存在，则插入新文档
            response = es_client.index(index=index_name, document=agent_data)
            print(f"{agent_name}: 配置插入到 Elasticsearch")

    except Exception as e:
        print(f"{agent_name}: 写入 Elasticsearch 失败 - {str(e)}")


def es_write_span_list(index_name, span_list):
    """
    批量写入 span_list 到 Elasticsearch
    """
    ES_PASSWORD, SERVER_IP = read_db_config()
    es = Elasticsearch(
        hosts=[f"http://{SERVER_IP}:9200"],
        basic_auth=(ES_USERNAME, ES_PASSWORD)
    )
    actions = [
        {
            "_index": index_name,
            "_source": span.tojson()
        }
        for span in span_list
    ]
    helpers.bulk(es, actions)

def es_clear_index(index_name):
    ES_PASSWORD, SERVER_IP = read_db_config()
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
    ES_PASSWORD, SERVER_IP = read_db_config()
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
        
        if span_obj.protocol not in ['Redis', 'HTTP1']:
            continue
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
    traces = sorted(traces, key=lambda trace: trace['start_time'])
    ES_PASSWORD, SERVER_IP = read_db_config()
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
    # print(f"Successfully wrote {success} traces to index {index_name}")


def es_clear_all():
    """
    清除所有 Elasticsearch 索引
    """
    ES_PASSWORD, SERVER_IP = read_db_config()
    es = Elasticsearch(
        hosts=[f"http://{SERVER_IP}:9200"],
        basic_auth=(ES_USERNAME, ES_PASSWORD)  # 添加用户名和密码
    )
    indices = [index for index in es.indices.get_alias(index="*") if not index.startswith('.')]
    for index in indices:
        print(f"Deleting index: {index}")
        es.indices.delete(index=index)



def es_read_new_spans(index_name, last_timestamp=None):
    """
    每次只读取比 last_timestamp 更新的 span 数据
    """
    ES_PASSWORD, SERVER_IP = read_db_config()
    es = Elasticsearch(hosts=[f"http://{SERVER_IP}:9200"], basic_auth=(ES_USERNAME, ES_PASSWORD))
    # 先判断索引是否存在
    if not es.indices.exists(index=index_name):
        print(f"Index {index_name} does not exist.")
        return [], last_timestamp
    
    query = {
        "size": 5000,
        "sort": [{"metric.start_time": "asc"}],
        "query": {
            "range": {
                "metric.start_time": {"gt": last_timestamp} if last_timestamp else {"gte": 0}
            }
        }
    }
    response = es.search(index=index_name, body=query)
    hits = response["hits"]["hits"]

    spans = []
    if hits:
        for hit in hits:
            doc = hit["_source"]
            span_obj = Span(doc)
            # if span_obj.protocol not in ['Thrift', 'HTTP1']:
            #     continue
            spans.append(span_obj)
            # 处理数据
        new_last_timestamp = hits[-1]["_source"]["metric"]["start_time"]
    else:
        new_last_timestamp = last_timestamp
    return spans, new_last_timestamp

def poll_agents_new_spans(agents, queue, poll_interval=5):
    """
    每隔 poll_interval 秒从所有 agent 读取未处理的 spans
    """
    last_timestamps = {agent_name: None for agent_name in agents}
    while True:
        count = 0
        for agent_name, agent in agents.items():
            index_name = agent.sender['index_name']
            last_ts = last_timestamps[agent_name]
            new_spans, new_last_ts = es_read_new_spans(index_name, last_ts)
            if new_spans:
                for span in new_spans:
                    count += 1
                    queue.put(span)
            last_timestamps[agent_name] = new_last_ts
        if count > 0:
            log(f"Polled {count} new spans from all agents.")
        # 这里可以对 all_new_spans 做统一处理或写入目标表
        time.sleep(poll_interval)


def check_db():
    ES_PASSWORD, SERVER_IP = read_db_config()

    while True:
        try:
            es = Elasticsearch(hosts=[f"http://{SERVER_IP}:9200"], basic_auth=(ES_USERNAME, ES_PASSWORD))
            if es.ping():
                log("Elasticsearch connected successfully!")
                break
            else:
                log("Elasticsearch not ready, retrying...")
        except Exception:
            # 不打印异常，不写入日志，只安静地重试
            pass
        time.sleep(5)
        
        
def write_service_metrics(metrics):
    """
    将服务指标写入 Elasticsearch
    """
    ES_PASSWORD, SERVER_IP = read_db_config()
    es = Elasticsearch(
        hosts=[f"http://{SERVER_IP}:9200"],
        basic_auth=(ES_USERNAME, ES_PASSWORD)  # 添加用户名和密码
    )
    
    index_name = "service_metrics"
    
    # 检查索引是否存在
    if not es.indices.exists(index=index_name):
        es.indices.create(index=index_name)
    
    actions = [
        {
            "_index": index_name,
            "_source": {
                "service_name": service_name,
                "metrics": metrics[service_name]
            }
        }
        for service_name in metrics
    ]
    
    helpers.bulk(es, actions)
    log("Service metrics written to Elasticsearch")
    
def write_callgraph(graph):
    """
    将调用图写入 Elasticsearch
    """
    ES_PASSWORD, SERVER_IP = read_db_config()
    es = Elasticsearch(
        hosts=[f"http://{SERVER_IP}:9200"],
        basic_auth=(ES_USERNAME, ES_PASSWORD)  # 添加用户名和密码
    )
    
    index_name = "call_graph"
    
    # 检查索引是否存在
    if not es.indices.exists(index=index_name):
        es.indices.create(index=index_name)
    
    actions = []
    
    for node in graph.nodes.values():
        actions.append({
            "_index": index_name,
            "_id": node.service_name,
            "_source": {
                "type": "node",
                "service_name": node.service_name,
                "tags": node.tags
            }
        })
    
    for edge in graph.edges.values():
        actions.append({
            "_index": index_name,
            "_id": f"{edge.src}-{edge.dst}",
            "_source": {
                "type": "edge",
                "src": edge.src,
                "dst": edge.dst,
                "metrics": edge.metrics
            }
        })
    
    helpers.bulk(es, actions)
    log("Call graph written to Elasticsearch")