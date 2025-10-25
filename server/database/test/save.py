from elasticsearch import Elasticsearch
import json

def export_es_index(index_name, output_file):
    # 连接 ES
    ES_USERNAME = "elastic"
    ES_PASSWORD = "netsys204"
    SERVER_IP = "202.112.237.37"
    # SERVER_IP = "114.215.254.187"
    es = Elasticsearch(
        hosts=[f"http://{SERVER_IP}:9200"],
        basic_auth=(ES_USERNAME, ES_PASSWORD)
    )



    # 查询所有文档
    query = {
        "query": {"match_all": {}},
        "size": 10000  # 可根据实际数据量调整
    }
    response = es.search(index=index_name, body=query)

    # 提取数据
    docs = [hit["_source"] for hit in response["hits"]["hits"]]

    # 保存到本地
    with open(output_file, "w", encoding="utf-8") as f:
        json.dump(docs, f, ensure_ascii=False, indent=2)

def import_json_to_es(index_name, input_file):
    # 连接 ES
    ES_USERNAME = "elastic"
    ES_PASSWORD = "deeptrace123"
    SERVER_IP = "202.112.237.37"
    # SERVER_IP = "114.215.254.187"
    es = Elasticsearch(
        hosts=[f"http://{SERVER_IP}:9200"],
        basic_auth=(ES_USERNAME, ES_PASSWORD)
    )

    mapping = {
        "mappings": {
            "properties": {
                "spans": {
                    "type": "nested"
                }
            }
        }
    }
    if not es.indices.exists(index=index_name):
        es.indices.create(index=index_name, body=mapping)
        
    # 读取本地 JSON 文件
    with open(input_file, "r", encoding="utf-8") as f:
        docs = json.load(f)

    # 批量写入 ES
    from elasticsearch import helpers
    actions = [
        {
            "_index": index_name,
            "_source": doc
        }
        for doc in docs
    ]
    helpers.bulk(es, actions)


if __name__ == "__main__":
    # export_es_index("traces", "bookinfo.json")
    # import_json_to_es("traces", "bookinfo.json")
    
    # export_es_index("traces", "social.json")
    import_json_to_es("traces2", "social.json")