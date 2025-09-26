
from trace.assemble.src.nodes import construct_nodes
from trace.assemble.src.edges import construct_edges



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
    else:
        return "200"


def add_childs(span, paret_childs):
    span_json = span.tojson()
    span_json['context']['child_ids'] = paret_childs[span.span_id]['childs']
    span_json['metric']['start_time'] = time_conversion(span.start_time)
    span_json['metric']['end_time'] = time_conversion(span.end_time)
    return span_json

def construct_topology(spans):
    """
    构建拓扑结构
    """

    def dict_to_tree(edges):
        all_children = {c for v in edges.values() for c in v}
        roots = [k for k in edges if k not in all_children]
        def build(node):
            children = edges.get(node, [])
            if not children:
                return None
            return {child: build(child) for child in children}
        return {roots[0]: build(roots[0])}

    tgid_edges = {}
    for span in spans:
        # print(span)
        if span['tag']['ebpf_tag']['direction'] == "Ingress" and span['context']['parent_id'] is not None:
            parent_id = span['context']['parent_id']
            parent_span = next((s for s in spans if s['context']['span_id'] == parent_id), None)
            parent_tgid = parent_span['tag']['ebpf_tag']['tgid']
            if parent_tgid not in tgid_edges:
                tgid_edges[parent_tgid] = []
            tgid_edges[parent_tgid].append(span['tag']['ebpf_tag']['tgid'])
    topo = dict_to_tree(tgid_edges)
    # topo = {}
    components = {}
    for span in spans:
        if span['tag']['ebpf_tag']['tgid'] not in components:
            if span['tag']['docker_tag'] is None:
                print(f"Warning: span {span['context']['span_id']} has no docker_tag")
                continue
            components[span['tag']['ebpf_tag']['tgid']] = {
                'name': span['tag']['docker_tag']['container_name'],
                'ip': span['tag']['docker_tag']['ip'],
                'endpoint': span['tag']['ebpf_tag']['endpoint'],
                'protocol': span['tag']['ebpf_tag']['protocol']
            }
            
    
    return topo, components


def time_conversion(ktime_ns):
    boot_time = int(open('/proc/stat').read().split('btime ')[1].split('\n')[0])
    event_time = boot_time * 1000 + int(ktime_ns / 1e3)
    return event_time


def get_aggre_tags(span_list):
    aggre_tags = {
        "endpoints": set(),
        "component_names": set(),
        "protocols": set(),
        "ips": set(),
        "status_codes": set()
    }
    for span in span_list:
        aggre_tags["endpoints"].add(span['tag']['ebpf_tag']['endpoint'])
        aggre_tags["component_names"].add(span['tag']['docker_tag']['container_name'])
        aggre_tags["protocols"].add(span['tag']['ebpf_tag']['protocol'])
        aggre_tags["ips"].add(span['tag']['docker_tag']['ip'])
        status_code = "200" # TODO
        if status_code is not None:
            aggre_tags["status_codes"].add(status_code)
    trace_tags = {}
    for key, value in aggre_tags.items():
        trace_tags[key] = list(value)
    return trace_tags



def assemble_trace(spans):
    """
    根据 span_id 和 parent_id 将 spans 分组为 trace 列表
    """
    span_dict = {span.span_id: span for span in spans}
    paret_childs = {}
    ip2nodeid = {}
    for span in spans:
        if span.direction == "Egress":
            ip = span.dst_ip
            node_id = span.tgid
            if ip not in ip2nodeid:
                ip2nodeid[ip] = node_id
        if span.direction == "Ingress":
            ip = span.src_ip
            node_id = span.tgid
            if ip not in ip2nodeid:
                ip2nodeid[ip] = node_id
        if span.span_id not in paret_childs:
            paret_childs[span.span_id] = {'parent': None, 'childs': []}
        if span.parent_id is not None:
            if span.parent_id not in paret_childs:
                paret_childs[span.parent_id] = {'parent': None, 'childs': []}
            paret_childs[span.parent_id]['childs'].append(span.span_id)
            paret_childs[span.span_id]['parent'] = span.parent_id
    traces = []
    all_nodes = []
    all_edges = []
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
        span_list = [add_childs(span_dict[span_id], paret_childs) for span_id in valid_span_ids]
        topo, components = construct_topology(span_list)
        aggretags = get_aggre_tags(span_list)
        nodes, fullnodes = construct_nodes(span_list, aggretags)
        edges, fulledges = construct_edges(span_list, ip2nodeid, aggretags)
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
                        'start_time': time_conversion(trace_start_time),
                        'end_time': time_conversion(trace_end_time),
                        'topology': topo,
                        'components': components,
                        'spans': span_list,
                        'nodes': nodes,
                        'edges': edges})
        all_nodes.extend(fullnodes)
        all_edges.extend(fulledges)
    # print(all_nodes[:10])
    return traces, all_nodes, all_edges
