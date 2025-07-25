
# 将span列表按照tgid、方向和IP进行分组，转化为字典形式
# 例如：{tgid: {direction: {ip: [span1, span2, ...]}}}

debug = open('./trace/logs/acc.txt', 'w')
spans_fp = open('./trace/logs/spans.txt', 'w')

def span_grouping(spans):
    span_map = {}
    traceid_stats = {}
    for span in spans:
        spans_fp.write(f"{span.trace_id} {span.span_id} {span.direction}  {span.protocol} {span.endpoint}\n")
        if span.trace_id not in traceid_stats:
            traceid_stats[span.trace_id] = 0
        traceid_stats[span.trace_id] += 1
        # print(span_obj)
        if span.tgid not in span_map:
            span_map[span.tgid] = {"incoming": {}, "outgoing": {}}
        if span.direction == "Ingress":
            if span.endpoint not in span_map[span.tgid]["incoming"]:
                span_map[span.tgid]["incoming"][span.endpoint] = []
            span_map[span.tgid]["incoming"][span.endpoint].append(span)
        elif span.direction == "Egress":
            if span.endpoint not in span_map[span.tgid]["outgoing"]:
                span_map[span.tgid]["outgoing"][span.endpoint] = []
            span_map[span.tgid]["outgoing"][span.endpoint].append(span)
        else:
            continue
    spans_fp.write(f"Total trace IDs: {len(traceid_stats)}\n")
    for trace_id, count in traceid_stats.items():
        spans_fp.write(f"Trace ID: {trace_id}, Count: {count}\n")
    return span_map

# 将所有的span合并成一个列表
def span_merge(all_spans):
    spans = []
    for tgid, tgid_spans in all_spans.items():
        for direction, span_list in tgid_spans.items():
            for ip in span_list:
                for span in span_list[ip]:
                    spans.append(span)
    return spans

# 任意被调用者 A 收到的入请求与触发的A调用特定组件B的出请求之间的关联准确率
def pair_acc(spans):
    spanid2traceid = {}
    pair_acc = {}
    for tgid, span_list in spans.items():
        for direction, span_list in span_list.items():
            for endpoint in span_list:
                for span in span_list[endpoint]:
                    spanid2traceid[span.span_id] = span.trace_id
                    
    for tgid, tgid_spans in spans.items():
        if len(tgid_spans['incoming']) == 0 or len(tgid_spans['outgoing']) == 0:
            continue
        if tgid not in pair_acc:
            pair_acc[tgid] = {}
        outgoing_spans = tgid_spans['outgoing']
        ingress_trace_ids = []
        for incoming_endpoint, incoming_span_list in tgid_spans['incoming'].items():
            for incoming_span in incoming_span_list:
                ingress_trace_ids.append(incoming_span.trace_id)
        for outgoing_endpoint, outgoing_span_list in outgoing_spans.items():
            if outgoing_endpoint not in pair_acc[tgid]:
                pair_acc[tgid][outgoing_endpoint] = 0
            for outgoing_span in outgoing_span_list:
                if outgoing_span.trace_id not in ingress_trace_ids:
                    debug.write(f"Warning: {outgoing_span.trace_id} {outgoing_span.endpoint} has no available inspan\n")
                    pair_acc[tgid][outgoing_endpoint] += 1
                    continue
                if outgoing_span.parent_id is not None:
                    if outgoing_span.parent_id in spanid2traceid:
                        if spanid2traceid[outgoing_span.parent_id] == outgoing_span.trace_id:
                            pair_acc[tgid][outgoing_endpoint] += 1
                        else:
                            debug.write(f"Error: {outgoing_span.trace_id} -> {spanid2traceid[outgoing_span.parent_id]} | {outgoing_span.endpoint}\n")
                else:
                    debug.write(f"Error: {outgoing_span.trace_id} has no parent_id | {outgoing_span.endpoint}\n")
                    
            pair_acc[tgid][outgoing_endpoint] = pair_acc[tgid][outgoing_endpoint] / len(outgoing_span_list)
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
        ingress_trace_ids = []
        for incoming_endpoint, incoming_span_list in tgid_spans['incoming'].items():
            for incoming_span in incoming_span_list:
                ingress_trace_ids.append(incoming_span.trace_id)

        outgoing_spans = tgid_spans['outgoing']
        count = 0
        for outgoing_ip, outgoing_span_list in outgoing_spans.items():
            count += len(outgoing_span_list)
            for outgoing_span in outgoing_span_list:
                if outgoing_span.trace_id not in ingress_trace_ids:
                    debug.write(f"Warning: {outgoing_span.trace_id} {outgoing_span.endpoint} has no available inspan\n")
                    svc_acc[tgid] += 1
                    continue
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
        ingress_trace_ids = []
        for incoming_endpoint, incoming_span_list in tgid_spans['incoming'].items():
            for incoming_span in incoming_span_list:
                ingress_trace_ids.append(incoming_span.trace_id)
        for outgoing_ip, outgoing_spans in tgid_spans['outgoing'].items():
            for outgoing_span in outgoing_spans:
                if outgoing_span.trace_id not in trace_acc:
                    trace_acc[outgoing_span.trace_id] = 1
                if outgoing_span.parent_id is None and outgoing_span.trace_id in ingress_trace_ids:
                    debug.write(f"e2e Error1: {outgoing_span.trace_id} {outgoing_span.endpoint} has no parent_id\n")
                    trace_acc[outgoing_span.trace_id] = 0
                    continue
                if outgoing_span.parent_id not in spanid2traceid:
                    debug.write(f"e2e Warning: {outgoing_span.trace_id} -> {outgoing_span.parent_id} | {outgoing_span.endpoint}\n")
                    continue
                if spanid2traceid[outgoing_span.parent_id] != outgoing_span.trace_id:
                    trace_acc[outgoing_span.trace_id] = 0
                    debug.write(f"e2e Error3: {outgoing_span.trace_id} -> {spanid2traceid[outgoing_span.parent_id]} | {outgoing_span.endpoint}\n")
                    continue
    print(f'error num: {len(trace_acc)-sum(trace_acc.values())}')
    return sum(trace_acc.values()) / len(trace_acc)

def print_acc(span_dict):
    acc1 = pair_acc(span_dict)
    acc2 = service_acc(span_dict)
    acc3 = e2e_acc(span_dict)
    print("Pair Accuracy:")
    for tgid in acc1:
        print(f"    TGID: {tgid:<8}")
        for outgoing_ip in acc1[tgid]:
            print(f"        Endpoint: {outgoing_ip:<10} | Accuracy: {100 * acc1[tgid][outgoing_ip]:.2f}%")
    print("service Accuracy:")
    for tgid in acc2:
        print(f"    TGID: {tgid:<8} | Accuracy: {100 * acc2[tgid]:.2f}%")
    print(f"End-to-End Accuracy: {100 * acc3:.2f}%")