import random
class Event(object):
    def __init__(self, trace_id, span_id, event_time_mus, span_kind, event_kind, sort_key):
        self.trace_id = trace_id
        self.span_id = span_id
        self.event_time_mus = event_time_mus
        self.span_kind = span_kind
        self.event_kind = event_kind
        self.sort_key = sort_key


def random_64bit_id():
    # 生成 16 个十六进制字符（64 位）
    return ''.join(random.choice('0123456789abcdef') for _ in range(16))

def vpath(spans):

    pass