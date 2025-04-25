from dataclasses import dataclass, field
import re
from typing import List, Tuple
import argparse

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
        # return f"{self.tgid}, {self.syscall}, {self.component}, {self.quintuple} {self.length}, {self.protocol}, {self.timestamp}, {self.message_type}, {self.enter_seq}, {self.exit_seq}, {self.payload}"
        return f"{self.message_type}"
    def buf_to_hex(self):
        return self.payload.hex()
    
def parse_raw_data(raw_data: str) -> List[Message]:
    entries = raw_data.split('\n')
    result = []
    pattern = r'(\d+), \s*([A-Za-z]+), \s*([^,]+), \s*l4_protocol:\s*([^,]+), \s*saddr:\s*([^,]+), \s*daddr:\s*([^,]+), \s*sport:\s*(\d+), \s*dport:\s*(\d+), \s*(\d+), \s*(\d+), \s*(\d+), \s*length:\s*(\d+), \s*([A-Za-z]+), \s*([A-Za-z]+), \s*(.*)'
    for entry in entries:
        match = re.match(pattern, entry)
        if match:
            tgid = int(match.group(1))
            syscall = str(match.group(2))
            component = str(match.group(3))
            quintuple = (str(match.group(4)), str(match.group(5)), str(match.group(6)), int(match.group(7)), int(match.group(8)))
            timestamp_ns = int(match.group(9))
            enter_seq = int(match.group(10))
            exit_seq = int(match.group(11))
            length = int(match.group(12))
            protocol = str(match.group(13))
            message_type = str(match.group(14))
            buf = match.group(15)
            message = Message(tgid, syscall, component, quintuple, timestamp_ns, enter_seq, exit_seq, length, protocol, message_type, 0, buf)
            result.append(message)
    return result

def parse(input: str = './result/ebpf.txt', output: str = './result/actual.txt'):
    file = open(input, 'r')
    messages = parse_raw_data(file.read())
    file.close()
    # messages = reassemble_messages(messages)
    with open(output, 'w', encoding="utf-8") as f:
        for message in messages:
            f.write(str(message) + '\n')

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description='Parse eBPF data.')
    parser.add_argument('--protocol', type=str, help='protocol name')
    # parser.add_argument('--input', type=str, default='./result/ebpf.txt', help='Input file path')
    
    args = parser.parse_args()
    input = f"{args.protocol}/result/ebpf.txt"
    output = f"{args.protocol}/result/actual.txt"
    parse(input, output)