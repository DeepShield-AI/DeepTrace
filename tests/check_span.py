import json
import re

def parse_traceid(content):
    pattern = r"[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}"
    trace_id = re.findall(pattern, content)
    return trace_id[0] if trace_id else None

rps = 500
file_path = f"/home/ubuntu/smore/DeepTrace/agent/spans.json"
    # print(f"Parsing JSON file: {file_path}")
with open(file_path, "r", encoding="utf-8") as file:
    spans = json.load(file)

# 获取 spans 列表
# spans = data.get("spans", [])
# data = {}
cnt = 0
for span in spans:
    # 获取 span 的属性
    req_tracied = parse_traceid(span['req_content'])
    resp_tracied = parse_traceid(span['resp_content'])
    

    tgid = span['tgid']
    src_ip = span['src_ip']
    dst_ip = span['dst_ip']
    src_port = span['src_port']
    dst_port = span['dst_port']
    five_tuple = f"{src_ip}:{src_port} -> {dst_ip}:{dst_port}"
    if req_tracied != resp_tracied:
        cnt += 1
        # print(span['req']['payload'], span['resp']['payload'])
        print(f"Trace ID mismatch: {req_tracied} != {resp_tracied} {tgid} {five_tuple}")
print(f"{cnt} spans with trace ID mismatch")
print(f"Total spans: {len(spans)}")
print(f"Mismatch ratio: {cnt / len(spans):.2%}")
    # if tgid not in data:
    #     data[tgid] = {}
    # if five_tuple not in data[tgid]:
    #     data[tgid][five_tuple] = []
#     span['req']['type'] = 'req'
#     span['resp']['type'] = 'resp'
#     data[tgid][five_tuple].append(span['req'])
#     data[tgid][five_tuple].append(span['resp'])

# for tgid, five_tuple_map in data.items():
#     for five_tuple, req_resps in five_tuple_map.items():
#         req_resps = sorted(req_resps, key=lambda x: x['timestamp_ns'])
#         print(f"TGID: {tgid}, Five Tuple: {five_tuple}")
#         last_type = None
#         for req_resp in req_resps:
#             if last_type == req_resp['type']:
#                 print(f"{req_resp['type']}: {parse_traceid(req_resp['payload'])} {req_resp['timestamp_ns']} {'error *'*20}")
#             else:
#                 print(f"{req_resp['type']}: {parse_traceid(req_resp['payload'])} {req_resp['timestamp_ns']}")
#             last_type = req_resp['type']
#         print("-" * 50)