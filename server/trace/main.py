
from database.src.utils import poll_agents_new_spans, check_db
from trace.association.src.cross import inter_association
from config.parse_config import load_agents, get_server_mode
from trace.association.src import deeptrace
from trace.association.src.utils import span_merge
from trace.assemble.src.utils import assemble_trace_from_spans
import threading
import queue
import time
import os
from log.utils import log




def span2trace(spans):
    spans = inter_association(spans, client_ingress='ComposePost', tuple_used=False)
    span_dict = deeptrace.deeptrace(spans)
    span_list = span_merge(span_dict)
    trace_num = assemble_trace_from_spans(span_list, 'traces')
    log(f"Assemble: {trace_num} traces")


def consumer(queue):
    count = 0
    spans = []
    while True:
        span = queue.get()
        if span is None:
            break
        spans.append(span)
        if len(spans) >= 5000:
            span2trace(spans)
            spans = []
        count += 1
        queue.task_done()

if __name__ == "__main__":
    # server_mode = get_server_mode()

    # if server_mode != 'automatic':
    #     log(f"Server mode is set to {server_mode}, ...")
    #     os.system('tail -f /dev/null')

    # log("Starting DeepTrace Analysis server...")
    # check_db()  # 检查数据库连接
    agents = load_agents()
    q = queue.Queue()

    # 启动生产者线程
    producer_thread = threading.Thread(target=poll_agents_new_spans, args=(agents, q, 2), daemon=True)
    producer_thread.start()

    # 启动消费者线程
    consumer_thread = threading.Thread(target=consumer, args=(q,), daemon=True)
    consumer_thread.start()

    # 主线程等待
    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        log("Exiting...")
        q.put(None)  # 通知消费者线程退出
        consumer_thread.join()