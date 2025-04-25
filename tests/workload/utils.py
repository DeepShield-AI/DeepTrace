import json
import re
from typing import List
from message import Message
from span import Span

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

def write_spans_to_json(spans: List[Span], output_file: str):
    with open(output_file, "w", encoding="utf-8") as f:
        json.dump(spans, f, indent=4, ensure_ascii=False)
    print(f"Successfully wrote {len(spans)} spans to {output_file}")
