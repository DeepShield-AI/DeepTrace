import json
from collections import defaultdict
import re

def extract_thrift_method(packet):
    thrift_metadata = {}
    if "thrift" in packet["_source"]["layers"]:
        thrift_layer = packet["_source"]["layers"]["thrift"]
        for key in thrift_layer:
            if "method" in key:
                thrift_metadata['method'] = thrift_layer[key]['thrift.method']
            if 'CALL' in key:
                thrift_metadata['type'] = "CALL"
            elif 'REPLY' in key:
                thrift_metadata['type'] = "REPLY"
        if thrift_metadata['type'] == "CALL":
            trace_span_id = thrift_layer['Data']['thrift.map']['thrift.string']
            content = str(thrift_layer).replace(trace_span_id, "")
            trace_id = trace_span_id.split(":")[0]
            thrift_metadata['trace_id'] = trace_id
            thrift_metadata['content'] = content
        elif thrift_metadata['type'] == "REPLY":
            thrift_metadata['content'] = str(thrift_layer)
            thrift_metadata['trace_id'] = ''
    return thrift_metadata


def get_endpoint(content):
    match = re.search(r'([A-Z][A-Za-z0-9_]+)[\x00\u0000]', content)
    if match:
        method_name = match.group(1)
        return method_name
    return 'UnknownEndpoint'

def extract_message(packets, message_map):
    message_id = 0
    for packet in packets:
        if "ip" in packet["_source"]["layers"] and "tcp" in packet["_source"]["layers"] and "thrift" in packet["_source"]["layers"]:
            message = {}
            thrift_content = extract_thrift_method(packet)
            message['thrift_method'] = thrift_content['method']
            message['type'] = thrift_content['type']
            message['trace_id'] = thrift_content['trace_id']
            message['content'] = thrift_content['content']
            ip_layer = packet["_source"]["layers"]["ip"]
            tcp_layer = packet["_source"]["layers"]["tcp"]
            srcip = ip_layer["ip.src"]
            dstip = ip_layer["ip.dst"]
            srcport = tcp_layer["tcp.srcport"]
            dstport = tcp_layer["tcp.dstport"]
            message['length'] = tcp_layer["tcp.len"]
            message['seq'] = tcp_layer["tcp.seq"]
            message['timestamp'] = packet["_source"]["layers"]["frame"]["frame.time_epoch"]
            if message['type'] == "CALL":
                message['tuple'] = (srcip, srcport, dstip, dstport)
            elif message['type'] == "REPLY":
                message['tuple'] = (dstip, dstport, srcip, srcport)
            message['endpoint'] = get_endpoint(message['content'])

            message_map[message_id] = message
            message_id += 1



def extract_span(call, reply, span_id):
    span = {}
    span['span_id'] = str(span_id)
    span['method'] = call['thrift_method']
    span['start_time'] = float(call['timestamp'])
    span['end_time'] = float(reply['timestamp'])
    span['duration_us'] = 1e6 * (float(reply['timestamp']) - float(call['timestamp']))
    span['req_size'] = call['length']
    span['resp_size'] = reply['length']
    span['req_content'] = call['content']
    span['resp_content'] = reply['content']
    span['trace_id'] = call['trace_id']
    span['srcip'] = call['tuple'][0]
    span['dstip'] = call['tuple'][2]
    span['src_port'] = call['tuple'][1]
    span['dst_port'] = call['tuple'][3]
    span['endpoint'] = call['endpoint']
    return span


def construct_spans(spans_file, message_map):
    span_id = 0
    spans = {}
    calls = []
    replies = []

    # 将 messages 分为 calls 和 replies
    for key, message in message_map.items():
        if message["type"] == "CALL":
            calls.append(message)
        elif message["type"] == "REPLY":
            replies.append(message)


    for reply in replies:
        reply_five_tuple = reply["tuple"]
        reply_method = reply["thrift_method"]
        reply_timestamp = float(reply["timestamp"])

        # 找到时间最近且五元组相同的 call
        closest_call = None
        min_time_diff = float('inf')
        for call in calls:
            call_five_tuple = call["tuple"]
            call_method = call["thrift_method"]
            call_timestamp = float(call["timestamp"])

            if call_method == reply_method and call_five_tuple == reply_five_tuple:
                time_diff = reply_timestamp - call_timestamp
                if 0 <= time_diff < min_time_diff:
                    closest_call = call
                    min_time_diff = time_diff

        if closest_call:
            spans[span_id] = extract_span(closest_call, reply, span_id)
            span_id += 1
            calls.remove(closest_call)  # 确保一对一关系

        else:
            print(f"No matching CALL found for REPLY with method {reply_method} and tuple {reply_five_tuple}")

    if calls:
        print(f"Unmatched CALLs found: {calls}")

    print(f"Constructed {len(spans)} spans")
    right = 0
    traces = {}
    for span in spans.values():
        trace_id = span['trace_id']
        if trace_id not in traces:
            traces[trace_id] = []
        traces[trace_id].append(span)
    max_size = max([len(trace_spans) for trace_spans in traces.values()])
    min_size = min([len(trace_spans) for trace_spans in traces.values()])
    # trace_group_by_length = {i: [] for i in range(min, max + 1)}
    for trace_id, trace_spans in traces.items():
        trace_spans.sort(key=lambda x: x['start_time'])
        # trace_group_by_length[len(trace_spans)].append(trace_spans)
        if len(trace_spans) > 13:
            right += 1
        print(f"Trace ID: {trace_id} has {len(trace_spans)} spans")
        for span in trace_spans:
            five_tuple = f'{span['srcip']}:{span['src_port']} -> {span['dstip']}:{span['dst_port']}'
            req = span['req_content'].encode('utf-8').hex()
            resp = span['resp_content'].encode('utf-8').hex()
            # print(f"{span.trace_id} {span.direction} {span.endpoint} {span.pid} {span.protocol} {five_tuple} {req} {resp}")
            print(f"{span['trace_id']} {span['method']} {five_tuple}")
    print(f"{right / len(traces) * 100}% of traces({right}) have more than 13 spans")
    print(sum([len(trace_spans) for trace_spans in traces.values()]) / len(traces), "is the average span count in a trace")
    print(len(traces), "is the total trace count")

    print(max_size, "is the max span count in a trace")
    print(min_size, "is the min span count in a trace")
    # for i in range(min, max + 1):
    #     print(len(trace_group_by_length[i]), "traces have", i, "spans")

    # num = 0
    # for trace in traces.values():
    #     for span in trace:
    #         if span['endpoint'] == 'ComposePost':
    #             num += 1
    # print(f"{num} spans have endpoint 'ComposePost'")
    # print("Trace with 1 span:\n")
    # for trace in trace_group_by_length[1]:
    #     print(trace[0])
    # print("Trace with 2 span:\n")
    # for trace in trace_group_by_length[2]:
    #     # if trace[0]['endpoint'] != trace[1]['endpoint']:
    #     print(trace[0])
    #     print(trace[1])
    #     print()
    # print("Trace with 3 span:\n")
    # for trace in trace_group_by_length[3]:
    #     print(trace[0]['endpoint'])
    #     print(trace[1]['endpoint'])
    #     print(trace[2]['endpoint'])
    #     print()

    # # print("Trace with 4 span:\n")
    # # for trace in trace_group_by_length[4]:
    # #     print(trace[0]['endpoint'])
    # #     print(trace[1]['endpoint'])
    # #     print(trace[2]['endpoint'])
    # #     print(trace[3]['endpoint'])
    # #     print()

    # print("Trace with 5 span:\n")
    # for trace in trace_group_by_length[5]:
    #     print(trace[0]['endpoint'])
    #     print(trace[1]['endpoint'])
    #     print(trace[2]['endpoint'])
    #     print(trace[3]['endpoint'])
    #     print(trace[4]['endpoint'])
    #     print()
    #     # json.dump(spans, open(spans_file, 'w'), indent=4)

def json_parse_hook(lst):
    result = {}
    count = {}
    for key, val in lst:
        if key in count: count[key] += 1
        else: count[key] = 1
        if key == 'thrift.i64' and key in result:
            pass
            # if count[key] > 2 :
            #         result[key] = max(result[key], val)
            # else:
            #     result[key] = result[key]
        else:
            result[key] = val
    
    return result


def main(packets_file, spans_file):
    message_map = {}
    with open(packets_file, 'r') as f:
        packets = json.loads(f.read(), object_pairs_hook=json_parse_hook)
        extract_message(packets, message_map)
    # json.dump(message_map, open("data/thrift_messages.json", 'w'), indent=4)
    construct_spans(spans_file, message_map)

if __name__ == "__main__":

    packets_file = "tests/example.json"
    spans_file = "tests/tcpdump-spans.json"
    main(packets_file, spans_file)
