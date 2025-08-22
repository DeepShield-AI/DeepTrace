

import re
import random
import string

class Span:
    def __init__(self, span_json):
        self.docker_tag = span_json.get("tag", "UnknownTag").get("docker_tag", "UnknownDockerTag")
        self.ebpf_tag = span_json.get("tag", "UnknownTag").get("ebpf_tag", "UnknownEBPFTag")
        self.metric = span_json.get("metric", "UnknownMetric")
        self.content = span_json.get("content", "UnknownContent")
        self.context = {}

        self.start_time = self.metric.get("start_time")
        self.end_time = self.metric.get("end_time")
        self.duration = self.metric.get("duration")
        self.req_size = self.metric.get("req_size", 0)
        self.resp_size = self.metric.get("resp_size", 0)

        self.tgid = self.ebpf_tag.get("tgid")
        self.pid = self.ebpf_tag.get("pid")
        self.protocol = self.ebpf_tag.get("protocol")
        self.component_name = self.ebpf_tag.get("component")
        self.direction = self.ebpf_tag.get("direction")
        self.src_ip = self.ebpf_tag.get("src_ip")
        self.dst_ip = self.ebpf_tag.get("dst_ip")
        self.src_port = self.ebpf_tag.get("src_port")
        self.dst_port = self.ebpf_tag.get("dst_port")

        self.req_content = self.content.get("req_content")
        self.resp_content = self.content.get("resp_content")

        self.trace_id = self.get_traceid(span_json)
        self.span_id = self.get_spanid(span_json) 
        self.endpoint = self.get_endpoint(span_json)
        self.parent_id = self.get_parentid(span_json)
        self.parent_traceid = self.get_parent_traceid(span_json)
    
    def get_parent_traceid(self, span_json):
        if 'context' in span_json:
            if 'parent_trace_id' in span_json['context']:
                return span_json['context']['parent_trace_id']
        return None
        

    def get_parentid(self, span_json):
        if 'context' in span_json:
            if 'parent_id' in span_json['context']:
                return span_json['context']['parent_id']
        return None

    def get_traceid(self, span_json):
        if 'context' in span_json:
            if 'trace_id' in span_json['context']:
                return span_json['context']['trace_id']
        content = self.req_content
        if self.protocol == 'Thrift':
            match = re.search(r'uber-trace-id\x00{3}([0-9a-f]+:[0-9a-f]+:[0-9a-f]+:[0-9a-f]+)\x00', content)
            if match:
                trace_id = match.group(1).split(":")[0][1:]
                return trace_id 
        if self.protocol == "HTTP1":
            match = re.search(r'Request-ID:\s*(\d+)', content)
            if match:
                return match.group(1)
            match = re.search(r'Request-Id:\s*(\d+)', content)
            if match:
                return match.group(1)
            match = re.search(r'Requestid:\s*(\d+)', content)
            if match:
                return match.group(1)
        if 'x-b3-traceid' in content:
            match = re.search(r"x-b3-traceid:\s*([0-9a-fA-F]+)", content)
            if match:
                return match.group(1)
        if "X-B3-TraceId" in content:
            match = re.search(r"X-B3-TraceId:\s*([0-9a-fA-F]+)", content)
            if match:
                return match.group(1)
        return 'UnknownTraceID'

    def get_spanid(self, span_json):
        if 'context' in span_json:
            if 'span_id' in span_json['context']:
                return span_json['context']['span_id']
        return ''.join(random.choices(string.ascii_letters + string.digits, k=64))

    def get_endpoint(self, span_json):
        if span_json.get('tag').get('ebpf_tag').get('endpoint'):
            return span_json['tag']['ebpf_tag']['endpoint']
        content = self.req_content
        if self.protocol == 'Thrift':
            match = re.search(r'([A-Z][A-Za-z0-9_]+)[\x00\u0000]', content)
            if match:
                method_name = match.group(1)
                return method_name
            # else:
            #     print(f"Thrift protocol but no method name found in content: {content}")
        elif self.protocol and 'mongo' in self.protocol.lower():
            # 匹配常见MongoDB操作
            match = re.search(r'(findAndModify|find|insert|update|delete)', content, re.IGNORECASE)
            if match:
                return match.group(1)
            # else:
            #     print(f"MongoDB protocol but no endpoint found in content: {content}")
        elif self.protocol and 'redis' in self.protocol.lower():
            # 匹配 Redis 命令（如 ZADD、GET、SET 等）
            match = re.search(r'\$\d+\s+([A-Z]+)', content)
            if match:
                return match.group(1)
            # else:
            #     print(f"Redis protocol but no command found in content: {content}")
        elif self.protocol and 'dns' in self.protocol.lower():
            # 匹配 DNS 域名
            match = re.search(r'([a-zA-Z0-9\-\.]+)\)', content)
            if match:
                # print(f"DNS protocol detected: {match.group(1)}")
                return match.group(1)
            # else:
            #     print(f"DNS protocol but no domain found in content: {content}")
        # print(f"{self.protocol}: {content}")
        elif self.protocol == "HTTP1":
            match = re.search(r'GET\s+([^\s]+)', content)
            if match:
                return match.group(1)
            match = re.search(r'ET\s+([^\s]+)', content)
            if match:
                return match.group(1)
            match = re.search(r'POST\s+([^\s]+)', content)
            if match:
                return match.group(1)
            match = re.search(r'PUT\s+([^\s]+)', content)
            if match:
                return match.group(1)
            match = re.search(r'DELETE\s+([^\s]+)', content)
            if match:
                return match.group(1)

        return 'UnknownEndpoint'

    def __str__(self):
        """
        返回 Span 的字符串表示
        """
        return (f"Span(start_time={self.start_time}, end_time={self.end_time}, "
                f"tgid={self.tgid}, pid={self.pid}, protocol={self.protocol}, component_name={self.component_name}), "
                f"trace_id={self.trace_id}, duration={self.duration}, direction={self.direction}) ")
    def get_context(self):
        return {'trace_id': self.trace_id, 'span_id': self.span_id, 'parent_id': self.parent_id, 'parent_trace_id': self.parent_traceid,}
    def tojson(self):
        """
        将 Span 的所有成员变量动态转换为 JSON 格式
        """
        self.ebpf_tag['endpoint'] = self.endpoint
        return {'context': self.get_context(),
                'tag': {'docker_tag': self.docker_tag, 'ebpf_tag': self.ebpf_tag},
                'metric': self.metric, 'content': self.content}
