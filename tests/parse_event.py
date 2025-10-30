from dataclasses import dataclass, field
import json
import struct
from typing import Tuple

type_map = {
    1: 'socket begin',
    2: 'socket end and connect begin',
    3: 'connect end and filter test begin',
    4: 'filter test end and filter begin',
    5: 'filter end and copy begin',
    6: 'copy end',
}

@dataclass
class Event:
    tgid: int
    type: str
    timestamp_ns: int

    def __init__(self, tgid: int, type: int, timestamp_ns: int):
        self.tgid = tgid
        self.type = type_map.get(type, 'unknown')
        self.timestamp_ns = timestamp_ns

    def __str__(self):
        return f'{self.tgid} {self.type} {self.timestamp_ns}'

event_file_path = 'event.txt'
event_file = open('event.txt', 'r')

events = json.loads(event_file.read())
event_file.close()

event_list = []
for event in events:
    key = bytes([int(b, 16) for b in event['key']])
    value = event['value']

    event_key = struct.unpack('<Q', key)[0]
    tgid = event_key >> 32
    
    event_type = struct.unpack('<Q', bytes([int(b, 16) for b in value[:8]]))[0] & 0xffffffff
    event_timestamp = struct.unpack('<Q', bytes([int(b, 16) for b in value[8:]]))[0]
    # event_type = int(event_type, 16)
    
    if tgid == 3568627:
        event_list.append(Event(tgid, event_type, event_timestamp))

event_list.sort(key=lambda x: x.timestamp_ns)
grouped = list(zip(*[iter(event_list)] * 6))

socket_time = 0
conn_time = 0
filter_test_time = 0
filter_time = 0
copy_time = 0
total_time = 0
# len = 0
for (socket_begin, socket_end_and_conn_begin, conn_end_and_filter_test_begin, filter_test_end_and_filter_begin, filter_end_and_copy_begin, copy_end) in grouped:
    print((copy_end.timestamp_ns - socket_begin.timestamp_ns - (filter_test_end_and_filter_begin.timestamp_ns - conn_end_and_filter_test_begin.timestamp_ns)) / 1000)
#     len += 1
# print(len)
# print("socket: ", socket_time / len(grouped))
# print("connect: ", conn_time / len(grouped))
# print("filter test: ", filter_test_time / len(grouped))
# print("filter: ", filter_time / len(grouped))
# print("copy: ", copy_time / len(grouped))
# print("total: ", total_time / len(grouped))

json.dump([event.__dict__ for event in event_list], open('parsed_event.json', 'w'), indent=4)