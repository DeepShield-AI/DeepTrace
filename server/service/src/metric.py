
import numpy as np
from database.src.utils import write_service_metrics, es_read_agent_span_list
from config.parse_config import load_agents
def service_metrics():
    agents = load_agents()
    spans = es_read_agent_span_list(agents)
    metrics = {}
    for span in spans:
        service_name = span.docker_tag.get('container_name', 'UnknownService')[0]
        service_name = service_name.split('_')[-1].split('.')[0]  # 获取容器名称的最后一部分
        if service_name not in metrics:
            metrics[service_name] = {
                'Ingress':{
                    'count': 0,
                    'durations': [],
                    'req_sizes': [],
                    'resp_sizes': [],
                    'start_time': float('inf'),
                    'end_time': float('-inf')
                },
                'Egress':{
                    'count': 0,
                    'durations': [],
                    'req_sizes': [],
                    'resp_sizes': [],
                    'start_time': float('inf'),
                    'end_time': float('-inf')
                },
                "Tags": {
                    "ip": span.docker_tag.get('ip', 'UnknownIP'),
                    "tgid": span.ebpf_tag.get('tgid', 'UnknownTgid'),
                }
            }
        direction = span.direction
        metrics[service_name][direction]['count'] += 1
        metrics[service_name][direction]['durations'].append(span.duration)
        metrics[service_name][direction]['req_sizes'].append(span.req_size)
        metrics[service_name][direction]['resp_sizes'].append(span.resp_size)
        metrics[service_name][direction]['start_time'] = min(metrics[service_name][direction]['start_time'], span.start_time)
        metrics[service_name][direction]['end_time'] = max(metrics[service_name][direction]['end_time'], span.end_time)
        
    stats_metrics = {}
    for component, data in metrics.items():
        stats_metrics[component] = {"Ingress": {}, "Egress": {}, "Tags": data["Tags"]}
        for direction, values in data.items():
            if direction not in ["Ingress", "Egress"]:
                continue
            if values['count'] > 0:
                stats_metrics[component][direction]['avg_duration'] = sum(values['durations']) / values['count'] / 1e6  # 转换为毫秒
                stats_metrics[component][direction]['p99_duration'] = np.percentile(values['durations'], 99) / 1e6  # 转换为毫秒
                stats_metrics[component][direction]['p95_duration'] = np.percentile(values['durations'], 95) / 1e6  # 转换为毫秒
                stats_metrics[component][direction]['avg_req_size'] = sum(values['req_sizes']) / values['count'] if values['count'] > 0 else 0
                stats_metrics[component][direction]['avg_resp_size'] = sum(values['resp_sizes']) / values['count'] if values['count'] > 0 else 0
                stats_metrics[component][direction]['RPS'] = values['count'] / ((values['end_time'] - values['start_time'])/1e9) if (values['end_time'] - values['start_time']) > 0 else 0
            else:
                stats_metrics[component][direction]['avg_duration'] = 0
                stats_metrics[component][direction]['avg_req_size'] = 0
                stats_metrics[component][direction]['avg_resp_size'] = 0
                stats_metrics[component][direction]['p99_duration'] = 0
                stats_metrics[component][direction]['p95_duration'] = 0
                stats_metrics[component][direction]['RPS'] = 0
    # for component, data in stats_metrics.items():
    #     print(f"Component: {component}")
    #     print(f"  Tags: {data['Tags']}")
    #     for direction, values in data.items():
    #         if direction not in ["Ingress", "Egress"]:
    #             continue
    #         print(f"  Direction: {direction}")
    #         print(f"    avg_duration: {values['avg_duration']:.2f} ms")
    #         print(f"    p99_duration: {values['p99_duration']:.2f} ms")
    #         print(f"    p95_duration: {values['p95_duration']:.2f} ms")
    #         print(f"    avg_req_size: {values['avg_req_size']:.2f} bytes")
    #         print(f"    avg_resp_size: {values['avg_resp_size']:.2f} bytes")
    #         print(f"    RPS: {values['RPS']:.2f} requests/sec")
    
    write_service_metrics(stats_metrics)
    

        