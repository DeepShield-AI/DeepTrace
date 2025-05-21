# 跨组件的span关联

# 在每个连接内，每个ingress span和最近的内容相同的egress span关联
def inter_association(spans):
    sorted_spans = sorted(spans, key=lambda x: x.start_time)
    for i, span in enumerate(sorted_spans):
        if span.direction == 'Ingress':
            for j in range(0, i)[::-1]:
                if sorted_spans[j].direction == 'Egress' and sorted_spans[j].src_ip == span.dst_ip and \
                        sorted_spans[j].src_port == span.dst_port and sorted_spans[j].dst_ip == span.src_ip and \
                        sorted_spans[j].dst_port == span.src_port:
                    span.parent_id = sorted_spans[j].span_id
                    # print(f"Associated Ingress span {span.span_id} with Egress span {sorted_spans[j].span_id}")
                    break
    return sorted_spans


