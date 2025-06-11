

from database.src.utils import es_read_span_list, es_write_traces
from trace.assemble.src.assemble import assemble_trace

def perform_assemble():
    for rps in [100, 200, 300, 400, 500]:
        index_name = f"test-rps-{rps}-mappings"
        spans = es_read_span_list(index_name)
        traces = assemble_trace(spans)
        es_write_traces(f"test-rps-{rps}-traces", traces)
        print(f"rps {rps} assemble done, traces num: {len(traces)}")

if __name__ == "__main__":
    for  rps in [100, 200, 300, 400, 500]:
        index_name = f"test-rps-{rps}-mappings"
        spans = es_read_span_list(index_name)
        traces = assemble_trace(spans)
        es_write_traces(f"test-rps-{rps}-traces", traces)
        print(f"rps {rps} assemble done, traces num: {len(traces)}")




        








