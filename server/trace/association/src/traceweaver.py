from utils import Span, parse_json, service_acc, e2e_acc
import copy


# spans[tgid][direction][dst_ip] = span_list
def vpath(spans):
    processed_spans = {}

    for tgid, tgid_spans in spans.items():
    # find candidates by using call graph, dependency order and time constraints
        for dst_ip, incoming_spans in tgid_spans['incoming']:
    # batching

    # construct delay distributions

    # ranking candidate mappings

    # joint optimization