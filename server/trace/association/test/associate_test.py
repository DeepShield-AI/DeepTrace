
import sys
import os
# 获取当前脚本所在目录
BASE_DIR = os.path.dirname(os.path.abspath(__file__))
UTILS_DIR = os.path.join(BASE_DIR, "..", "..", "association", "src")
sys.path.append(os.path.normpath(UTILS_DIR))

from utils import Span, es_read_spans, es_write_spans, pair_acc, service_acc, e2e_acc
from fifo import fifo
from vpath import vpath
from deeptrace import deeptrace
from wap5 import wap5
from traceweaver_v1 import traceweaver_v1
from traceweaver_v2 import traceweaver_v2
from cross import inter_association
import copy
# from elasticsearch import Elasticsearch
import json
import argparse

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--algo", type=str, choices=["traceweaver_v2", "traceweaver_v1", "fifo", "vpath", "deeptrace", "wap5"], default="fifo")

    args = parser.parse_args()
    algo = args.algo

    for rps in [100, 200, 300, 400, 500]:
    # for rps in [100]:
        index_name = f"rps-{rps}-spans"
        spans = es_read_spans(index_name)
        # print(spans)

        spans = inter_association(spans)
        
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

        es_write_spans(f'rps-{rps}-mappings', processed_spans)


        








