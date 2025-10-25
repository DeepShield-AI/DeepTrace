
from log.utils import log
from elasticsearch import Elasticsearch, helpers
import time
from config.parse_config import read_db_config
from trace.model.span import Span
import json

ES_USERNAME = "elastic"

def add_k8stag_to_span(index_name):
    """
    从 Elasticsearch 中读取指定索引的所有 span 数据，
    根据 span 的 tag.ebpf_tag.tgid 在 k8s_tag_map 中查找 k8s_tag，
    并更新 span 的 tag.k8s_tag 字段，最后写回数据库。
    """
    k8s_tag_map = json.load(open("./config/k8s_tag_map.json", "r"))
    ES_PASSWORD, SERVER_IP = read_db_config()
    t1 = time.time()
    es = Elasticsearch(hosts=[f"http://{SERVER_IP}:9200"], basic_auth=(ES_USERNAME, ES_PASSWORD))

    query = {
        "query": {
            "match_all": {}
        },
        "size": 10000
    }

    response = es.search(index=index_name, body=query)
    t2 = time.time()
    print(f"Read all spans from {index_name} took {t2 - t1:.2f} seconds")

    spans = []
    actions = []
    for hit in response["hits"]["hits"]:
        span = hit["_source"]
        tgid = None
        # 获取 tgid
        try:
            tgid = span.get("tag", {}).get("ebpf_tag", {}).get("tgid")
        except Exception:
            pass
        # 查找 k8s_tag
        k8s_tag = k8s_tag_map.get(str(tgid), {}) if tgid else {}
        # 更新 span 的 tag.k8s_tag 字段
        if "tag" not in span:
            span["tag"] = {}
        span["tag"]["k8s_tag"] = k8s_tag
        spans.append(span)
        # 构造更新操作
        actions.append({
            "_op_type": "update",
            "_index": index_name,
            "_id": hit["_id"],
            "doc": {"tag": span["tag"]}
        })

    # 批量更新到 Elasticsearch
    if actions:
        helpers.bulk(es, actions)
        print(f"Updated {len(actions)} spans with k8s_tag.")

    return spans

if __name__ == "__main__":
    add_k8stag_to_span("spans_agent1")