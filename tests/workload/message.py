from dataclasses import dataclass, field
from typing import Tuple

@dataclass
class Message:
    tgid: int
    syscall: str
    component: str
    quintuple: Tuple[str, str, str, int, int]
    timestamp: int
    enter_seq: int
    exit_seq: int
    length: int
    protocol: str
    message_type: str
    sequence_id: int = field(default=0)
    reassembled: bool = field(default=False)
    payload: bytes = field(default=b"")

    def __init__(self, tgid: int, syscall: str, component: str, quintuple: Tuple[str, str, str, int, int], timestamp: int, enter_seq: int, exit_seq: int, length: int, protocol: str, message_type: str, sequence_id: int, payload: str):
        self.tgid = tgid
        self.syscall = syscall
        self.component = component
        self.quintuple = quintuple
        self.timestamp = timestamp
        self.enter_seq = enter_seq
        self.exit_seq = exit_seq
        self.length = length
        self.protocol = protocol
        self.sequence_id = sequence_id
        self.message_type = message_type
        self.reassembled = False
        self.payload = payload

    def __str__(self):
        return f"{self.tgid}, {self.syscall}, {self.component}, {self.quintuple} {self.length}, {self.protocol}, {self.timestamp}, {self.message_type}, {self.enter_seq}, {self.exit_seq}, {self.payload}"

    def buf_to_hex(self):
        return self.payload.hex()