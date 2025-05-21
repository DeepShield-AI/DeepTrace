
import sys
import os
# 获取当前脚本所在目录
BASE_DIR = os.path.dirname(os.path.abspath(__file__))
UTILS_DIR = os.path.join(BASE_DIR, "..", "..", "association", "src")
sys.path.append(os.path.normpath(UTILS_DIR))

from utils import Span, es_read_spans, es_write_spans, pair_acc, service_acc, e2e_acc
from cross import inter_association
import copy
# from elasticsearch import Elasticsearch
import json
import argparse

if __name__ == "__main__":


    # for rps in [100, 200, 300, 400, 500]:
    for rps in [100]:
        index_name = f"rps-{rps}-spans"
        spans = es_read_spans(index_name)
        spans = inter_association(spans)


        








