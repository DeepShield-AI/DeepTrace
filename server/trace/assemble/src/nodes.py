

def construct_nodes(spans, aggre_tags):
    """Construct nodes from spans and node IDs.

    Args:
        spans (list): A list of span objects.
    """
    
    simplifiednodes = []
    fullnodes = []
    
    for span in spans:
        simplifiednodes.append({'nodeId': span['tag']['docker_tag']['tgid'], 'metric': span['metric'], 'status_code': "200"})
        fullnodes.append({'nodeId': span['tag']['docker_tag']['tgid'], 'metric': span['metric'], 'tag': span['tag'], 'trace_tags': aggre_tags, 'context': span['context'], 'status_code': "200", "component": span['tag']['docker_tag']['container_name']})
    return simplifiednodes, fullnodes