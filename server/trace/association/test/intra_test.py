import copy
import argparse
from database.src.utils import es_read_span_list
from trace.association.src.fifo import fifo
from trace.association.src.vpath import vpath
from trace.association.src.deeptrace import deeptrace
from trace.association.src.wap5 import wap5
from trace.association.src.traceweaver_v1 import traceweaver_v1
from trace.association.src.traceweaver_v2 import traceweaver_v2
from trace.association.src.utils import pair_acc, service_acc, e2e_acc, span_merge
from database.src.utils import es_write_span_list

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--algo", type=str, choices=["traceweaver_v2", "traceweaver_v1", "fifo", "vpath", "deeptrace", "wap5"], default="fifo")

    args = parser.parse_args()
    algo = args.algo

    for rps in [100, 200, 300, 400, 500]:
    # for rps in [100]:
        index_name = f"test-rps-{rps}-spans"
        spans = es_read_span_list(index_name)
        
        print("-" * 50)
        if algo == 'fifo':
            processed_spans = fifo(copy.deepcopy(spans))
        elif algo == 'deeptrace':
            processed_spans = deeptrace(copy.deepcopy(spans))
        elif algo == 'vpath':
            processed_spans = vpath(copy.deepcopy(spans))
        elif algo == 'wap5':
            processed_spans = wap5(copy.deepcopy(spans))
        elif algo == 'traceweaver_v1':
            processed_spans = traceweaver_v1(copy.deepcopy(spans))
        elif algo == 'traceweaver_v2':
            processed_spans = traceweaver_v2(copy.deepcopy(spans))
        
        acc1 = pair_acc(processed_spans)
        acc2 = service_acc(processed_spans)
        acc3 = e2e_acc(processed_spans)
        print(f"RPS: {rps}")
        print("Pair Accuracy:")
        for tgid in acc1:
            for outgoing_ip in acc1[tgid]:
                print(f"    TGID: {tgid:<8} | IP: {outgoing_ip:<10} | Accuracy: {acc1[tgid][outgoing_ip]:.2f}")
        print("service Accuracy:")
        for tgid in acc2:
            print(f"    TGID: {tgid:<8} | Accuracy: {acc2[tgid]:.2f}")
        print(f"End-to-End Accuracy: {acc3:.2f}")

        span_list = span_merge(processed_spans)
        es_write_span_list(f'test-rps-{rps}-mappings', span_list)


        








