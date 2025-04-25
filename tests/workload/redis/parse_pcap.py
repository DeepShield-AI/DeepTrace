import json

def extract_message(packets):
    messages = []
    for packet in packets:
        if "resp" in packet["_source"]["layers"]:
            resp_layer = packet["_source"]["layers"]["resp"]
            if "resp.array" not in resp_layer:
                messages.append("Response")
            else:
                array = resp_layer['resp.array']
                string = array["resp.bulk_string"]
                first = string["resp.bulk_string.value"]
                parts = first.split(":")[:2]
                result = [int(x, 16) for x in parts]
                if 65 <= result[0] <= 90 and 65 <= result[1] <= 90:
                    messages.append("Request")
                else:
                    messages.append("Response")
    return messages

def json_parse_hook(lst):
    result = {}
    count = {}
    for key, val in lst:
        if key in count: count[key] += 1
        else: count[key] = 1
        if key == 'resp.bulk_string' and key in result:
            pass
        else:
            result[key] = val
    
    return result

if __name__ == "__main__":
    packets_file = f"./result/redis.json"
    message_file = f"./result/expected.txt"
    with open(packets_file, 'r') as f:
        packets = json.loads(f.read(), object_pairs_hook=json_parse_hook)
        messages = extract_message(packets)
    with open(message_file, 'w') as f:
        for message in messages:
            f.write(f"{message}\n")