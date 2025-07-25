

def search_span(span, paret_childs):
    current_spans = [span.span_id, span.parent_id]
    visited = set()
    spanid_list = []
    while current_spans:
        current_span_id = current_spans.pop(0)
        if current_span_id in visited:
            continue
        visited.add(current_span_id)
        spanid_list.append(current_span_id)

        if len(paret_childs[current_span_id]['childs']):
            current_spans.extend(paret_childs[current_span_id]['childs'])
        if paret_childs[current_span_id]['parent'] is not None:
            current_spans.append(paret_childs[current_span_id]['parent']) 
    return spanid_list

def check_finish(spans, visted):
    for span in spans:
        if span.parent_id is not None and span.span_id not in visted:
            return False
    return True

def get_status_code(span):
    if span.protocol == 'HTTP1':
        return span.resp_content.split()[1]


def add_childs(span, paret_childs):
    span_json = span.tojson()
    span_json['context']['child_ids'] = paret_childs[span.span_id]['childs']
    return span_json

def assemble_trace(spans):
    """
    根据 span_id 和 parent_id 将 spans 分组为 trace 列表
    """
    span_dict = {span.span_id: span for span in spans}
    paret_childs = {}
    for span in spans:
        if span.span_id not in paret_childs:
            paret_childs[span.span_id] = {'parent': None, 'childs': []}
        if span.parent_id is not None:
            if span.parent_id not in paret_childs:
                paret_childs[span.parent_id] = {'parent': None, 'childs': []}
            paret_childs[span.parent_id]['childs'].append(span.span_id)
            paret_childs[span.span_id]['parent'] = span.parent_id
    traces = []
    visited = set()
    while check_finish(spans, visited) is False:
        span = spans.pop(0)
        if span.parent_id is None:
            continue
        if span.span_id in visited:
            continue
        span_list = search_span(span, paret_childs)
        for span_id in span_list:
            visited.add(span_id)
        valid_span_ids = [sid for sid in span_list if sid in span_dict]
        root_span_id = max(valid_span_ids, key=lambda sid: span_dict[sid].duration)
        e2e_dutaion = span_dict[root_span_id].duration
        ingress_endpoint = span_dict[root_span_id].endpoint
        ingress_component_name = span_dict[root_span_id].component_name
        protocol = span_dict[root_span_id].protocol
        status_code = get_status_code(span_dict[root_span_id])
        if len(span_list) < 5:
            continue
        trace_start_time = min(span_dict[span_id].start_time for span_id in valid_span_ids)
        trace_end_time = max(span_dict[span_id].end_time for span_id in valid_span_ids)
        traces.append({ 'trace_id': span.trace_id,
                        'span_num': len(span_list),
                        'e2e_duration': e2e_dutaion,
                        'endpoint': ingress_endpoint,
                        'component_name': ingress_component_name,
                        'server_ip': span_dict[root_span_id].src_ip,
                        'server_port': span_dict[root_span_id].src_port,
                        'client_ip': span_dict[root_span_id].dst_ip,
                        'client_port': span_dict[root_span_id].dst_port,
                        'protocol': protocol,
                        'status_code': status_code,
                        'start_time': trace_start_time,
                        'end_time': trace_end_time,
                        'spans': [add_childs(span_dict[span_id], paret_childs) for span_id in valid_span_ids
                      ]})
    return traces
