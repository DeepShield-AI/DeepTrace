import json

def extract_message(packets):
    messages = []
    for packet in packets:
        if "memcache" in packet["_source"]["layers"]:
            memcached_layer = packet["_source"]["layers"]["memcache"]
            magic = memcached_layer['memcache.magic']
            if int(magic) == 128:
                messages.append("Request")
            else:
                messages.append("Response")
    return messages

if __name__ == "__main__":
    packets_file = f"./result/memcached.json"
    message_file = f"./result/expected.txt"
    with open(packets_file, 'r') as f:
        packets = json.loads(f.read())
        messages = extract_message(packets)
    with open(message_file, 'w') as f:
        for message in messages:
            f.write(f"{message}\n")