
from utils import Span
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
        traces.append({'spans': [span_dict[span_id].tojson() for span_id in span_list], 'trace_id': span.trace_id})
    return traces