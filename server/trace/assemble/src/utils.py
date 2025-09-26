
from trace.assemble.src.assemble import assemble_trace
from database.src.utils import es_read_span_list, es_write_traces, es_write_nodes_edges

def assemble_trace_from_db(src_index, dest_index):
    spans = es_read_span_list(src_index)
    traces, all_nodes, all_edges = assemble_trace(spans)
    es_write_traces(dest_index, traces)
    es_write_nodes_edges(all_nodes, all_edges)

def assemble_trace_from_spans(spans, dest_index):
    traces, all_nodes, all_edges = assemble_trace(spans)
    es_write_traces(dest_index, traces)
    return len(traces)
