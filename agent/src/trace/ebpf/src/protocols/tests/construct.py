from collections import defaultdict
from typing import List
from message import Message
from span import Span
from utils import write_spans_to_json, parse_raw_data

def reassemble_messages(messages: List[Message]) -> List[Message]:
    reassembled_messages = []
    groups = defaultdict(list)
    for message in messages:
        groups[message.quintuple].append(message)
    for group in groups.values():
        group.sort(key=lambda x: x.timestamp)
        i = 0
        while i < len(group) - 1:
            curr_msg = group[i]
            next_msg = group[i + 1]

            if curr_msg.exit_seq == next_msg.enter_seq and curr_msg.timestamp < next_msg.timestamp:
                curr_msg.payload = curr_msg.payload[:-1] + ', ' + next_msg.payload[1:]

                curr_msg.exit_seq = next_msg.exit_seq
                curr_msg.length += next_msg.length
                curr_msg.reassembled = True
                del group[i + 1]
            else:
                i += 1
        reassembled_messages.extend(group)
        reassembled_messages.sort(key=lambda x: x.timestamp)
    return reassembled_messages

protolcols = ['Thrift', 'HTTP', 'MongoDB', 'Redis', 'Memcached']

PARSED = "../output/parsed.txt"
UNMATCHED = "../output/unmatched.txt"

def parse(log_file_path: str = '../output/output.txt'):
    file = open(log_file_path, 'r')
    messages = parse_raw_data(file.read())
    file.close()
    # messages = reassemble_messages(messages)
    with open(PARSED, 'w', encoding="utf-8") as f:
        for message in messages:
            f.write(str(message) + '\n')
    for protocol in protolcols:
        message = [entry for entry in messages if entry.protocol == protocol]
        message.sort(key=lambda x: x.timestamp)
        with open(f'./output/{protocol}_messages.txt', 'w', encoding='utf-8') as f:
            for entry in message:
                f.write(str(entry) + '\n')
                
    grouped_data = defaultdict(lambda: defaultdict(list))
    
    for entry in messages:
        key = entry.quintuple
        grouped_data[entry.protocol][key].append(entry)

    result = defaultdict(list)
    un_matched_calls = defaultdict(lambda: defaultdict(list))
    
    for protocol, protocol_messages in grouped_data.items():
        if protocol in ['Thrift', 'HTTP', 'MongoDB', 'Redis', 'Memcached']:
            for ip_pair, entries in protocol_messages.items():
                egress = [entry for entry in entries if entry.message_type == 'Response']
                ingress = [entry for entry in entries if entry.message_type == 'Request']
                print(f"Protocol: {protocol}, ip_pairs: {ip_pair}, ingress: {len(ingress)}, egress: {len(egress)}")
                if not ingress or not egress:
                    if ingress:
                        un_matched_calls[protocol][ip_pair].extend(ingress)
                    if egress:
                        un_matched_calls[protocol][ip_pair].extend(egress)
                    continue
                
                ingress.sort(key=lambda x: x.timestamp)
                egress.sort(key=lambda x: x.timestamp)
                
                i, j = 0, 0
                matched_pairs = []
                while i < len(ingress) and j < len(egress):
                    req, res = ingress[i], egress[j]
                    
                    if req.timestamp < res.timestamp:
                        matched_pairs.append((req, res))
                        i += 1
                        j += 1
                    else:
                        j += 1
                
                result[protocol].extend([Span(req, res) for req, res in matched_pairs])

                un_matched_ingress = ingress[i:]
                un_matched_egress = egress[j:]
                
                if un_matched_ingress:
                    un_matched_calls[protocol][ip_pair].extend(un_matched_ingress)
                if un_matched_egress:
                    un_matched_calls[protocol][ip_pair].extend(un_matched_egress)
        else:
            for ip_pairs, entries in protocol_messages.items():
                egress = [entry for entry in entries if entry.message_type in ['Response']]
                ingress = [entry for entry in entries if entry.message_type in ['Request']]
                if len(ingress) == 0 or len(egress) == 0:
                    continue
                print(f"Protocol: {protocol}, ip_pairs: {ip_pairs}, ingress: {len(ingress)}, egress: {len(egress)}")
                egress.sort(key=lambda x: x.timestamp)
                ingress.sort(key=lambda x: x.timestamp)
                for e in reversed(egress):
                    for i in reversed(ingress):
                        if i.timestamp < e.timestamp and i.sequence_id == e.sequence_id:
                            result[protocol].append(Span(i, e))
                            ingress.remove(i)
                            egress.remove(e)
                            break
                            
                if ingress:
                    if ip_pairs not in un_matched_calls[protocol]:
                        un_matched_calls[protocol][ip_pairs] = []
                    un_matched_calls[protocol][ip_pairs].extend(ingress)
                if egress:
                    if ip_pairs not in un_matched_calls[protocol]:
                        un_matched_calls[protocol][ip_pairs] = []
                    un_matched_calls[protocol][ip_pairs].extend(egress)

    un_matched = open(UNMATCHED, 'w')
    for protocol, protocol_messages in un_matched_calls.items():
        print(f"Unmatched {protocol} calls: {len(protocol_messages)}")
        for ip_pairs, entries in protocol_messages.items():
            un_matched.write(f"Protocol: {protocol}, ip_pairs: {ip_pairs}\n")
            for entry in entries:
                un_matched.write(str(entry) + '\n')
    un_matched.close()

    for protocol, spans in result.items():
        write_spans_to_json([span.to_json() for span in spans], f"../output/{protocol}_spans.json")

if __name__ == '__main__':
    parse()