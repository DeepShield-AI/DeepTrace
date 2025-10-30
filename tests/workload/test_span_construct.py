import json

def prepare():
    raw_file = open("../output/spans.json")
    spans_file = open("./spans.json", "w+")
    lines = raw_file.readlines()

    if len(lines) >= 3:
        target_line = lines[-3]
    
        target_line = target_line.replace(',', '')
        lines[-3] = '}\n'
        spans_file.writelines(lines)

    spans_file.close()
    spans_file = open("./spans.json", "r")
    spans = json.loads(spans_file.read())["spans"]
    spans_file.close()
    return spans

def check_redis(spans):
    if len(spans) == 0:
        print("No spans found for Redis protocol.")
        return
    sum = 0
    all = 0
    for span in spans:
        req_payload = span['req']['payload']
        resp_payload = span['resp']['payload']
        if "GETRANGE" not in req_payload:
            continue
        all += 1
        req_payload = req_payload[23:47]
        resp_payload = resp_payload[5:29]
        if req_payload == resp_payload:
            sum += 1
        else:
            print("req: ", span['req']['payload'])
    print("Protocol: Redis")
    print("Total: ", all)
    print("Correct: ", sum)
    print("Accuracy: ", sum / all)

def check_http1(spans):
    if len(spans) == 0:
        print("No spans found for HTTP1 protocol.")
        return
    sum = 0
    for span in spans:
        req_payload = span['req']['payload'][-24:]
        resp_payload = span['resp']['payload'][-28:-4]
        if req_payload == resp_payload:
            sum += 1
        else:
            print("Span: ", span)

    print("Protocol: HTTP1")
    print("Total: ", len(spans))
    print("Correct: ", sum)
    print("Accuracy: ", sum / len(spans))

def check_thrift(spans):
    if len(spans) == 0:
        print("No spans found for Thrift protocol.")
        return
    sum = 0
    for span in spans:
        req_payload = span['req']['payload'][-25:-1]
        resp_payload = span['resp']['payload'][-25:-1]
        if req_payload == resp_payload:
            sum += 1
        else:
            print("Span: ", span)

    print("Protocol: Thrift")
    print("Total: ", len(spans))
    print("Correct: ", sum)
    print("Accuracy: ", sum / len(spans))

def check_memcached(spans):
    if len(spans) == 0:
        print("No spans found for Memcached protocol.")
        return
    sum = 0
    all = 0
    for span in spans:
        if ord(span['req']['payload'][1]) == 1:
            continue
        all += 1
        req_payload = span['req']['payload'][-24:]
        resp_payload = span['resp']['payload'][-24:]
        if req_payload == resp_payload:
            sum += 1
        else:
            print("Span: ", span)

    print("Protocol: Memcached")
    print("Total: ", all)
    print("Correct: ", sum)
    print("Accuracy: ", sum / all)

spans = prepare()

protocols = {"Redis": check_redis, "HTTP1": check_http1, "Memcached": check_memcached}

if __name__ == '__main__':
    all_spans = prepare()
    for protocol, checker in protocols.items():
        spans = [span for span in all_spans if span['req']['protocol'] == protocol and span['resp']['protocol'] == protocol]
        checker(spans)