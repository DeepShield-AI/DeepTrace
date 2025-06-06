
from trace.assemble.src.assemble import assemble_trace
from database.src.utils import es_read_span_list, es_write_traces

def assemble_trace_from_db(src_index, dest_index):
    spans = es_read_span_list(src_index)
    traces = assemble_trace(spans)
    es_write_traces(dest_index, traces)
