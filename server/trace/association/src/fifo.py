from utils import Span, pair_acc, service_acc, e2e_acc, intra_preprocess
import copy


def fifo(spans):
    spans_dict = intra_preprocess(spans)
    Allocated = {} # 记录入span是否分配了某个下游调用者的出span
    for tgid, tgid_spans in spans_dict.items():
        if len(tgid_spans['incoming']) == 0 or len(tgid_spans['outgoing']) == 0:
            continue

        for direction, span_list in tgid_spans.items():
            for ip in span_list:
                # print(f'tgid: {tgid}, direction: {direction}, ip: {ip}')
                span_list[ip] = sorted(span_list[ip], key=lambda x: x.start_time)
        caller = list(tgid_spans['incoming'].keys())[0]

                
        incmoing_spans = tgid_spans['incoming'][caller]
        for outgoing_ip, outgoing_spans in tgid_spans['outgoing'].items(): # 不同的下游被调用服务
            Allocated[outgoing_ip] = {}
            for outgoing_span in outgoing_spans:
                for incoming_span in incmoing_spans:
                    # 时间范围约束+入span不能有两个相同的下游调用者的出span约束
                    # if incoming_span.pid != outgoing_span.pid:
                    #     continue
                    if outgoing_span.start_time >= incoming_span.start_time and outgoing_span.end_time <= incoming_span.end_time \
                        and incoming_span.span_id not in Allocated[outgoing_ip]:
                        outgoing_span.parent_id = incoming_span.span_id
                        Allocated[outgoing_ip][incoming_span.span_id] = True
                        break
                    else:
                        continue    
                    
    return spans_dict

