


# Egress的时候, dst_ip为node的ip, Ingress的时候, src_ip为node的ip


def construct_edges(spans, ip2nodeid, aggretags):
    simplifiededges = []
    fulledges = []
    for span in spans:
        if span['tag']['ebpf_tag']['direction'] == "Egress":
            src_id = span['tag']['docker_tag']['tgid']
            dst_ip = span['tag']['ebpf_tag']['src_ip']
            if dst_ip in ip2nodeid:
                dst_id = ip2nodeid[dst_ip]
                simplifiededges.append({'src_nodeid': src_id, 'dst_nodeid': dst_id, 'metric': span['metric'], 'status_code': "200"})
                fulledges.append({'src_nodeid': src_id, 'dst_nodeid': dst_id, 'metric': span['metric'], 'tag': span['tag'], 'trace_tags': aggretags, 'context': span['context'], 'status_code': "200"})
            # else:
            #     print(f"Warning: dst_ip {dst_ip} not in ip2nodeid")
                # print(span['tag']['ebpf_tag'])
    # print(ip2nodeid)
    return simplifiededges, fulledges
    