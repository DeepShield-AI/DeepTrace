import numpy as np
from database.src.utils import write_callgraph, es_read_agent_span_list
from config.parse_config import load_agents
from callgraph.src.model import Graph

# Ingress: src_ip为组件ip; Egress: dst_ip为组件ip



def construct_graph():
    agents = load_agents()
    spans = es_read_agent_span_list(agents)
    
    ip2svc = {}
    graph = Graph()
    for span in spans:
        if span.direction == "Egress":
            continue
        service_name = span.docker_tag.get('container_name', 'UnknownService')[0]
        service_name = service_name.split('_')[-1].split('.')[0]  # 获取容器名称的最后一部分
        src_ip = span.src_ip if span.src_ip else 'UnknownSrcIP'
        ip2svc[src_ip] = service_name
        graph.add_node(service_name, tags={
            "ip": span.docker_tag.get('ip', 'UnknownIP'),
            "tgid": span.ebpf_tag.get('tgid', 'UnknownTgid'),
            "endpoint": span.endpoint if span.endpoint else 'UnknownEndpoint',
        })
    
    edge_metrics = {}
    for span in spans:
        if span.direction == "Egress":
            continue
        src_ip = span.src_ip if span.src_ip else 'UnknownSrcIP'
        dst_ip = span.dst_ip if span.dst_ip else 'UnknownDstIP'
        dst_service = ip2svc.get(src_ip, 'UnknownSrcService')
        src_service = ip2svc.get(dst_ip, 'UnknownDstService')
        edge_key = (src_service, dst_service)
        if edge_key not in edge_metrics:
            edge_metrics[edge_key] = {
                'durations': [],
                'req_sizes': [],
                'resp_sizes': [],
                'start_time': float('inf'),
                'end_time': float('-inf'),
                'count': 0
            }
        edge_metrics[edge_key]['durations'].append(span.duration)
        edge_metrics[edge_key]['req_sizes'].append(span.req_size)
        edge_metrics[edge_key]['resp_sizes'].append(span.resp_size)
        edge_metrics[edge_key]['start_time'] = min(edge_metrics[edge_key]['start_time'], span.start_time)
        edge_metrics[edge_key]['end_time'] = max(edge_metrics[edge_key]['end_time'], span.end_time)
        edge_metrics[edge_key]['count'] += 1
    
    for (src, dst), metrics in edge_metrics.items():
        if src not in graph.nodes or dst not in graph.nodes:
            # print(f"Skipping edge from {src} to {dst} due to missing nodes.")
            continue
        avg_duration = sum(metrics['durations']) / metrics['count'] / 1e6  # 转换为毫秒
        p99_duration = np.percentile(metrics['durations'], 99) / 1e6  # 转换为毫秒
        p95_duration = np.percentile(metrics['durations'], 95) / 1e6  # 转换为毫秒
        avg_req_size = sum(metrics['req_sizes']) / metrics['count'] if metrics['count'] > 0 else 0
        avg_resp_size = sum(metrics['resp_sizes']) / metrics['count'] if metrics['count'] > 0 else 0
        rps = metrics['count'] / ((metrics['end_time'] - metrics['start_time']) / 1e9) if (metrics['end_time'] - metrics['start_time']) > 0 else 0
        metrics = {
            'avg_duration': avg_duration,
            'p99_duration': p99_duration,
            'p95_duration': p95_duration,
            'avg_req_size': avg_req_size,
            'avg_resp_size': avg_resp_size,
            'RPS': rps
        }
        graph.add_edge(src, dst, metrics=metrics)
    # graph.print_graph()
    write_callgraph(graph)
        
    
        