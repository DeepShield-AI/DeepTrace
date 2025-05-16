from utils import Span, parse_json, service_acc, e2e_acc
import copy

# Client server对接，周五前
# 配置、网络传输
# server的一部分，控制器，发配置
# 实现tag的机制
# 
def fifo(spans):
    processed_spans = {}
    Allocated = {} # 记录入span是否分配了某个下游调用者的出span
    for tgid, span in spans.items():
        if len(span['incoming']) == 0 or len(span['outgoing']) == 0:
            continue
        
        processed_spans[tgid] = []
        for direction, span_list in span.items():
            for ip in span_list:
                # print(f'tgid: {tgid}, direction: {direction}, ip: {ip}')
                span_list[ip] = sorted(span_list[ip], key=lambda x: x.start_time)
        caller = list(span['incoming'].keys())[0]
        incmoing_spans = span['incoming'][caller]
        for outgoing_ip, outgoing_spans in span['outgoing'].items(): # 不同的下游被调用服务
            Allocated[outgoing_ip] = {}
            for outgoing_span in outgoing_spans:
                for incoming_span in incmoing_spans:
                    # 时间范围约束+入span不能有两个相同的下游调用者的出span约束
                    if outgoing_span.start_time >= incoming_span.start_time and outgoing_span.end_time <= incoming_span.end_time \
                        and incoming_span.trace_id not in Allocated[outgoing_ip]:
                        outgoing_span.association_tracid = incoming_span.trace_id
                        processed_spans[tgid].append(outgoing_span)
                        Allocated[outgoing_ip][incoming_span.trace_id] = True
                        break
                else:
                    continue

            
    return processed_spans

