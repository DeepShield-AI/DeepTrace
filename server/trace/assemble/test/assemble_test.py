
import sys
import os
# 获取当前脚本所在目录
BASE_DIR = os.path.dirname(os.path.abspath(__file__))
UTILS_DIR = os.path.join(BASE_DIR, "..", "..", "assemble", "src")
sys.path.append(os.path.normpath(UTILS_DIR))

from utils import Span, es_read_spans, es_write_traces
from assemble import assemble_trace
import copy
# from elasticsearch import Elasticsearch
import json

if __name__ == "__main__":


    for rps in [100, 200, 300, 400, 500]:
    # for rps in [100]:
        index_name = f"rps-{rps}-mappings"
        spans = es_read_spans(index_name)
        traces = assemble_trace(spans)
        es_write_traces(f'rps-{rps}-traces', traces)



        








