# 跨组件的span关联
import math
from collections import Counter

def jaccard_similarity(str1, str2):
    str1 = str1.encode('utf-8')
    str2 = str2.encode('utf-8')
    set1 = set(str1)
    set2 = set(str2)
    intersection = len(set1 & set2)
    union = len(set1 | set2)
    return intersection / union if union != 0 else 0

def cosine_similarity(str1, str2):
    str1 = str1.encode('utf-8')
    str2 = str2.encode('utf-8')
    words1 = Counter(str1)
    words2 = Counter(str2)
    all_words = set(words1) | set(words2)
    vec1 = [words1.get(word, 0) for word in all_words]
    vec2 = [words2.get(word, 0) for word in all_words]
    dot_product = sum(a*b for a, b in zip(vec1, vec2))
    norm1 = math.sqrt(sum(a*a for a in vec1))
    norm2 = math.sqrt(sum(b*b for b in vec2))
    return dot_product / (norm1 * norm2) if norm1 and norm2 else 0

# # 在每个连接内，每个ingress span和最近的内容相同的egress span关联
# def inter_association(spans):
#     host_delta = 1e7 # ns = 10ms 不同主机之间的时钟偏差
#     sorted_spans = sorted(spans, key=lambda x: x.start_time)
#     used_set = set()
#     error_count = 0
#     for i, span in enumerate(sorted_spans):
#         if span.direction == 'Ingress':
#             index2score = {}
#             for j in range(0, i)[::-1]:
#                 if span.start_time - sorted_spans[j].start_time > host_delta:
#                     break
#                 # if sorted_spans[j].direction == 'Egress' and sorted_spans[j].src_ip == span.src_ip and \
#                 #         sorted_spans[j].src_port == span.src_port and sorted_spans[j].dst_ip == span.dst_ip and \
#                 #         sorted_spans[j].dst_port == span.dst_port:
#                 if span.endpoint == sorted_spans[j].endpoint:
#                     index2score[j] = jaccard_similarity(span.req_content, sorted_spans[j].req_content)

#             for j in range(i + 1, len(sorted_spans)):
#                 if sorted_spans[j].start_time - span.start_time > host_delta:
#                     break
#                 # if sorted_spans[j].direction == 'Egress' and sorted_spans[j].src_ip == span.src_ip and \
#                 #         sorted_spans[j].src_port == span.src_port and sorted_spans[j].dst_ip == span.dst_ip and \
#                 #         sorted_spans[j].dst_port == span.dst_port:
#                 if span.endpoint == sorted_spans[j].endpoint:
#                     index2score[j] = jaccard_similarity(span.req_content, sorted_spans[j].req_content)
#             sorted_indexs = sorted(index2score.items(), key=lambda x: x[1], reverse=True)
#             for j, score in sorted_indexs:
#                 if j not in used_set:
#                     span.parent_id = sorted_spans[j].span_id
#                     used_set.add(i)
#                     if span.trace_id != sorted_spans[j].trace_id:
#                         error_count += 1
#                         print(f"Warning: trace_id mismatch {span.trace_id} != {sorted_spans[j].trace_id} {span.endpoint} {sorted_spans[j].endpoint}")
#                     break
#     print(f"Inter association error count: {error_count}")
#     return sorted_spans


def inter_association(spans, client_ingress = None, tuple_used=False, tuple_direction=False):
    """
    client_ingress: 如果指定了client_ingress，则跳过该endpoint的Ingress span
    tuple_used: 是否使用四元组进行关联
    tuple_direction: 是否需要调换四元组的方向
    """
    host_delta = 1e7 # ns = 10ms 不同主机之间的时钟偏差
    sorted_spans = sorted(spans, key=lambda x: x.start_time)
    used_set = set()
    error_count = 0
    mapping_score = {}
    cross_count = 0
    for i, span in enumerate(sorted_spans):
        if client_ingress is not None:
            if client_ingress in span.endpoint:
                continue
        if span.direction == 'Ingress':
            cross_count += 1
            index2score = {}
            for j in range(0, i)[::-1]:
                if span.start_time - sorted_spans[j].start_time > host_delta:
                    break
                if tuple_used:
                    if tuple_direction:
                        if (sorted_spans[j].dst_ip, sorted_spans[j].dst_port, sorted_spans[j].src_ip, sorted_spans[j].src_port) != \
                            (span.src_ip, span.src_port, span.dst_ip, span.dst_port):
                            continue
                    else:
                        if (sorted_spans[j].src_ip, sorted_spans[j].src_port, sorted_spans[j].dst_ip, sorted_spans[j].dst_port) != \
                                (span.src_ip, span.src_port, span.dst_ip, span.dst_port):
                            continue
                if span.endpoint == sorted_spans[j].endpoint:
                    mapping_score[(i, j)] = cosine_similarity(span.req_content, sorted_spans[j].req_content)


            for j in range(i + 1, len(sorted_spans)):
                if sorted_spans[j].start_time - span.start_time > host_delta:
                    break
                if tuple_used:
                    if tuple_direction:
                        if (sorted_spans[j].dst_ip, sorted_spans[j].dst_port, sorted_spans[j].src_ip, sorted_spans[j].src_port) != \
                            (span.src_ip, span.src_port, span.dst_ip, span.dst_port):
                            continue
                    else:
                        if (sorted_spans[j].src_ip, sorted_spans[j].src_port, sorted_spans[j].dst_ip, sorted_spans[j].dst_port) != \
                                (span.src_ip, span.src_port, span.dst_ip, span.dst_port):
                            continue
                if span.endpoint == sorted_spans[j].endpoint:
                    mapping_score[(i, j)] = cosine_similarity(span.req_content, sorted_spans[j].req_content)

    mapping_score = sorted(mapping_score.items(), key=lambda x: x[1], reverse=True)
    # fp = open('score.txt', 'w')
    for (i, j), score in mapping_score:
        
        if j not in used_set and i not in used_set:
            # fp.write(f'1 {sorted_spans[i].trace_id} {sorted_spans[j].trace_id} {score} {sorted_spans[i].req_content.encode("utf-8")} {sorted_spans[j].req_content.encode("utf-8")}\n')
            sorted_spans[i].parent_id = sorted_spans[j].span_id
            used_set.add(j)
            used_set.add(i)
            if sorted_spans[i].trace_id != sorted_spans[j].trace_id:
                error_count += 1
                print(f"Warning: trace_id mismatch {sorted_spans[i].trace_id} != {sorted_spans[j].trace_id} {sorted_spans[i].endpoint} {sorted_spans[j].endpoint}")
        else:
            # fp.write(f'0 {sorted_spans[i].trace_id} {sorted_spans[j].trace_id} {score} {sorted_spans[i].req_content.encode("utf-8")} {sorted_spans[j].req_content.encode("utf-8")}\n')
            continue
        
    # print(f"Inter association: error count: {error_count}, sum: {cross_count}, acc: {(cross_count - error_count) / cross_count if cross_count > 0 else 0:.2f}")
    return sorted_spans


