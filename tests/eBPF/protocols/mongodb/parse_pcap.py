import json

def extract_message(packets):
    messages = []
    for packet in packets:
        if "mongo" in packet["_source"]["layers"]:
            mongodb_layer = packet["_source"]["layers"]["mongo"]
            message = {}
            opcode = mongodb_layer['mongo.opcode']
            if opcode in ["2001", "2002", "2004", "2005", "2006", "2007", "2012"]:
                messages.append("Request")
            elif opcode == "1":
                messages.append("Response")
            else:
                if int(mongodb_layer["mongo.response_to"], 16) == 0:
                    messages.append("Request")
                else:
                    messages.append("Response")
    return messages

if __name__ == "__main__":
    packets_file = f"./result/mongodb.json"
    message_file = f"./result/expected.txt"
    with open(packets_file, 'r') as f:
        packets = json.loads(f.read())
        messages = extract_message(packets)
    with open(message_file, 'w') as f:
        for message in messages:
            f.write(f"{message}\n")