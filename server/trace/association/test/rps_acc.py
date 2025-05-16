
import sys
import os
# 获取当前脚本所在目录
BASE_DIR = os.path.dirname(os.path.abspath(__file__))
UTILS_DIR = os.path.join(BASE_DIR, "..", "..", "association", "src")
sys.path.append(os.path.normpath(UTILS_DIR))

from utils import Span, read_all_spans, service_acc, e2e_acc
from fifo import fifo
import copy
from elasticsearch import Elasticsearch


if __name__ == "__main__":

    for rps in [100, 200, 300, 400, 500]:
    # for rps in [100]:
        index_name = f"rps-{rps}-spans"
        spans = read_all_spans(index_name)
        # print(spans)
        processed_spans = fifo(copy.deepcopy(spans))
        service_acc1 = service_acc(processed_spans)
        e2e_acc1 = e2e_acc(processed_spans)
        print("-" * 50)
        print(f"RPS: {rps}")
        print("Service Accuracy:")
        print("-" * 30)
        for tgid, acc in service_acc1.items():
            print(f" TGID: {tgid:<10} | Accuracy: {acc:.2f}")
        print("-" * 30)
        print("End-to-End Accuracy:")
        print(f" {e2e_acc1:.2f}")





