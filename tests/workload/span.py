from dataclasses import dataclass
from message import Message


@dataclass
class Span:
    tgid: int
    component: str
    protocol: str
    start: int
    end: int
    duration: int
    req_syscall: str
    resp_syscall: str
    req_quintuple: str
    resp_quintuple: str
    req_bytes: int
    resp_bytes: int
    req_data: str
    resp_data: str

    def __init__(self, ingress: Message, egress: Message):
        self.start = ingress.timestamp
        self.end = egress.timestamp
        self.protocol = ingress.protocol
        self.duration = egress.timestamp - ingress.timestamp
        self.component = ingress.component
        self.tgid = ingress.tgid
        self.req_syscall = ingress.syscall
        self.resp_syscall = egress.syscall
        self.req_quintuple = ingress.quintuple
        self.resp_quintuple = egress.quintuple
        self.req_bytes = ingress.length
        self.resp_bytes = egress.length
        self.req_data = ingress.payload
        self.resp_data = egress.payload

    def to_json(self):
        return {
            'tgid': self.tgid,
            'protocol': self.protocol,
            'component': self.component,
            'start': self.start,
            'end': self.end,
            'duration': self.duration,
            # 'req_pid': self.req_pid,
            # 'resp_pid': self.resp_pid,
            'req_quintuple': str(self.req_quintuple),
            'resp_quintuple': str(self.resp_quintuple),
            'req_syscall': self.req_syscall,
            'resp_syscall': self.resp_syscall,
            'req_bytes': self.req_bytes,
            'resp_bytes': self.resp_bytes,
            'req_data': self.req_data,
            'resp_data': self.resp_data
        }
