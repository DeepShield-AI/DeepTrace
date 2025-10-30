import ipaddress
import json
import re

def get_endpoint(content):
    match = re.search(r'([A-Z][A-Za-z0-9_]+)[\x00\u0000]', content)
    if match:
        method_name = match.group(1)
        return method_name
    return 'UnknownEndpoint'

def parse_traceid(content):
    pattern = r'uber-trace-id\x00{3}([0-9a-f]+:[0-9a-f]+:[0-9a-f]+:[0-9a-f]+)\x00'
    
    match = re.search(pattern, content)
    if match:
        return match.group(1).split(":")[0][1:]
    return 'UnknownTraceID'

agent1_span = open("agent/spans.txt").readlines()
agent1_span = [line.strip() for line in agent1_span if line.strip()]

spans = []

for line in agent1_span:
    span = json.loads(line)
    span["agent"] = "agent1"
    span['trace_id'] = parse_traceid(span['content']['req_content'])
    span['endpoint'] = get_endpoint(span['content']['req_content'])
    spans.append(span)

traces = {}

for span in spans:
    if span['tag']['ebpf_tag']['protocol'] != "Thrift":
        continue
    if span['trace_id'] == "UnknownTraceID":
        print(f"Span with UnknownTraceID: {span}")
        continue
    if span['trace_id'] not in traces:
        traces[span['trace_id']] = []
    traces[span['trace_id']].append(span)

print(f"Total spans: {len([span for span in spans if span['tag']['ebpf_tag']['protocol'] == 'Thrift'])}")
right = 0

max = max([len(trace_spans) for trace_spans in traces.values()])
min = min([len(trace_spans) for trace_spans in traces.values()])
for trace_id, trace_spans in traces.items():
    trace_spans.sort(key=lambda x: x['metric']['start_time'])
    if len(trace_spans) == 21:
        right += 1
    print(f"Trace ID: {trace_id} has {len(trace_spans)} spans")
    for span in trace_spans:
        five_tuple = f'{ipaddress.ip_address(span['tag']['ebpf_tag']['src_ip'])}:{span['tag']['ebpf_tag']['src_port']} -> {ipaddress.ip_address(span['tag']['ebpf_tag']['dst_ip'])}:{span['tag']['ebpf_tag']['dst_port']}'
        req = span['content']['req_content'].encode('utf-8').hex()
        resp = span['content']['resp_content'].encode('utf-8').hex()
        # print(f"{span.trace_id} {span.direction} {span.endpoint} {span.pid} {span.protocol} {five_tuple} {req} {resp}")
        print(f"{span['trace_id']} {span['tag']['ebpf_tag']['direction']} {span['tag']['ebpf_tag']['req_seq']} {span['tag']['ebpf_tag']['resp_seq']} {span['endpoint']} {span['tag']['ebpf_tag']['pid']} {five_tuple}")
print(f"{right / len(traces) * 100}% of traces({right}) have 21 spans")
print(sum([len(trace_spans) for trace_spans in traces.values()]) / len(traces), "is the average span count in a trace")
print(len(traces), "is the total trace count")

print(max, "is the max span count in a trace")
print(min, "is the min span count in a trace")