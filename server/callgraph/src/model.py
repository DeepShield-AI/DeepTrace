class Node:
    def __init__(self, name, tags=None):
        self.service_name = name
        self.tags = tags

class Edge:
    def __init__(self, src, dst, metrics=None):
        self.src = src
        self.dst = dst
        self.metrics = metrics if metrics is not None else {}

class Graph:
    def __init__(self):
        self.nodes = {}
        self.edges = {}

    def add_node(self, name, tags=None):
        if name not in self.nodes:
            self.nodes[name] = Node(name, tags)
    
    def add_edge(self, src, dst, metrics=None):
        edge_key = (src, dst)
        if edge_key not in self.edges:
            self.edges[edge_key] = Edge(src, dst, metrics)


    def to_dict(self):
        return {
            "nodes": [
                {"name": node.service_name, "tags": node.tags}
                for node in self.nodes.values()
            ],
            "edges": [
                {"src": edge.src, "dst": edge.dst, "metrics": edge.metrics}
                for edge in self.edges.values()
            ]
        }
        
    def print_graph(self):
        print("Nodes:")
        for node in self.nodes.values():
            print(f"  {node.service_name} - Tags: {node.tags}")
        print("Edges:")
        for edge in self.edges.values():
            print(f"  {edge.src} -> {edge.dst} ")

