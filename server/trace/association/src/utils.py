
# 将span列表按照tgid、方向和IP进行分组，转化为字典形式
# 例如：{tgid: {direction: {ip: [span1, span2, ...]}}}
def span_grouping(spans):
    span_map = {}
    for span in spans:
        if span.protocol not in ['Thrift', 'HTTP1']:
            continue
        # print(span_obj)
        if span.tgid not in span_map:
            span_map[span.tgid] = {"incoming": {}, "outgoing": {}}
        if span.direction == "Ingress":
            if span.dst_ip not in span_map[span.tgid]["incoming"]:
                span_map[span.tgid]["incoming"][span.dst_ip] = []
            span_map[span.tgid]["incoming"][span.dst_ip].append(span)
        elif span.direction == "Egress":
            if span.dst_ip not in span_map[span.tgid]["outgoing"]:
                span_map[span.tgid]["outgoing"][span.dst_ip] = []
            span_map[span.tgid]["outgoing"][span.dst_ip].append(span)
        else:
            continue
    return span_map

# 将所有的span合并成一个列表
def span_merge(all_spans):
    spans = []
    for tgid, tgid_spans in all_spans.items():
        for direction, span_list in tgid_spans.items():
            for ip in span_list:
                for span in span_list[ip]:
                    spans.append(span.tojson())
    return spans

# 任意被调用者 A 收到的入请求与触发的A调用特定组件B的出请求之间的关联准确率
def pair_acc(spans):
    spanid2traceid = {}
    pair_acc = {}
    for tgid, span_list in spans.items():
        for direction, span_list in span_list.items():
            for ip in span_list:
                for span in span_list[ip]:
                    spanid2traceid[span.span_id] = span.trace_id
                    
    for tgid, tgid_spans in spans.items():
        if len(tgid_spans['incoming']) == 0 or len(tgid_spans['outgoing']) == 0:
            continue
        if tgid not in pair_acc:
            pair_acc[tgid] = {}
        outgoing_spans = tgid_spans['outgoing']
        for outgoing_ip, outgoing_span_list in outgoing_spans.items():
            if outgoing_ip not in pair_acc[tgid]:
                pair_acc[tgid][outgoing_ip] = 0
            for outgoing_span in outgoing_span_list:
                if outgoing_span.parent_id is not None:
                    if outgoing_span.parent_id in spanid2traceid:
                        if spanid2traceid[outgoing_span.parent_id] == outgoing_span.trace_id:
                            pair_acc[tgid][outgoing_ip] += 1
            pair_acc[tgid][outgoing_ip] = pair_acc[tgid][outgoing_ip] / len(outgoing_span_list)
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
        outgoing_spans = tgid_spans['outgoing']
        count = 0
        for outgoing_ip, outgoing_span_list in outgoing_spans.items():
            count += len(outgoing_span_list)
            for outgoing_span in outgoing_span_list:
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
        for outgoing_ip, outgoing_spans in tgid_spans['outgoing'].items():
            for outgoing_span in outgoing_spans:
                if outgoing_span.trace_id not in trace_acc:
                    trace_acc[outgoing_span.trace_id] = 1
                if outgoing_span.parent_id is None:
                    trace_acc[outgoing_span.trace_id] = 0
                    continue
                if outgoing_span.parent_id not in spanid2traceid:
                    trace_acc[outgoing_span.trace_id] = 0
                    continue
                if spanid2traceid[outgoing_span.parent_id] != outgoing_span.trace_id:
                    trace_acc[outgoing_span.trace_id] = 0
                    continue

    return sum(trace_acc.values()) / len(trace_acc)

def print_acc(span_dict):
    acc1 = pair_acc(span_dict)
    acc2 = service_acc(span_dict)
    acc3 = e2e_acc(span_dict)
    print("Pair Accuracy:")
    for tgid in acc1:
        for outgoing_ip in acc1[tgid]:
            print(f"    TGID: {tgid:<8} | IP: {outgoing_ip:<10} | Accuracy: {acc1[tgid][outgoing_ip]:.2f}")
    print("service Accuracy:")
    for tgid in acc2:
        print(f"    TGID: {tgid:<8} | Accuracy: {acc2[tgid]:.2f}")
    print(f"End-to-End Accuracy: {acc3:.2f}")