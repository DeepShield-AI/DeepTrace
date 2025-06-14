

import re
import random
import string

class Span:
    def __init__(self, span_json):
        self.start_time = span_json.get("start_time")
        self.end_time = span_json.get("end_time")
        self.tgid = span_json.get("tgid")
        self.pid = span_json.get("pid")
        self.protocol = span_json.get("protocol")
        self.component_name = span_json.get("component_name")
        self.req_content = span_json.get("req_content")
        self.resp_content = span_json.get("resp_content")
        self.trace_id = self.get_traceid(span_json)
        self.direction = span_json.get("direction")
        self.src_ip = span_json.get("src_ip")
        self.dst_ip = span_json.get("dst_ip")
        self.src_port = span_json.get("src_port")
        self.dst_port = span_json.get("dst_port")
        self.direction = span_json.get("direction")
        self.duration = float(self.end_time - self.start_time) if self.start_time and self.end_time else None
        self.req_size = len(self.req_content) if self.req_content else 0
        self.resp_size = len(self.resp_content) if self.resp_content else 0
        self.span_id = self.get_spanid(span_json) 
        self.endpoint = self.get_endpoint(span_json)
        self.parent_id = span_json.get('parent_id', None)


    def get_traceid(self, span_json):
        if span_json.get('trace_id'):
            return span_json['trace_id']
        content = span_json.get('req_content', '')
        match = re.search(r'uber-trace-id\x00{3}([0-9a-f]+:[0-9a-f]+:[0-9a-f]+:[0-9a-f]+)\x00', content)
        if match:
            trace_id = match.group(1).split(":")[0][1:]
            return trace_id 
        return 'UnknownTraceID'

    def get_spanid(self, span_json):
        if span_json.get('span_id'):
            return span_json['span_id']
        return ''.join(random.choices(string.ascii_letters + string.digits, k=64))

    def get_endpoint(self, span_json):
        if span_json.get('endpoint'):
            return span_json['endpoint']
        content = span_json.get('req_content', '')
        if self.protocol == 'Thrift':
            match = re.search(r'([A-Z][A-Za-z0-9_]+)[\x00\u0000]', content)
            if match:
                method_name = match.group(1)
                return method_name
        return 'UnknownEndpoint'

    def __str__(self):
        """
        返回 Span 的字符串表示
        """
        return (f"Span(start_time={self.start_time}, end_time={self.end_time}, "
                f"tgid={self.tgid}, pid={self.pid}, protocol={self.protocol}, component_name={self.component_name}), "
                f"trace_id={self.trace_id}, duration={self.duration}, direction={self.direction}) ")
    def tojson(self):
        """
        将 Span 的所有成员变量动态转换为 JSON 格式
        """
        return {attr: getattr(self, attr) for attr in self.__dict__}
