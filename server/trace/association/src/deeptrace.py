from utils import Span, pair_acc, service_acc, e2e_acc, intra_preprocess
import copy
from collections import Counter, defaultdict
import math
import time
from scipy.stats import pearsonr, gaussian_kde
from numpy import dot
from numpy.linalg import norm
import numpy as np




span_fields = {}
endpoint_ships = {}
multi_metrics = {} # tgid -> (endpoint1, endpoint2) -> metric -> pdf
maping_scores = {} # tgid -> (span1, span2) -> type -> score
tgid_weights = {} # tgid -> weights


def cosine_similarity(x, y):
    return dot(x, y) / (norm(x) * norm(y))

def transaction_field(spans, window_size=4):
    """
    使用 n-gram 分割内容为字段，并基于 TF-IDF 更新字段的权重。

    :param spans: 包含请求和响应内容的 span 列表
    :param window_size: n-gram 的大小，默认为 4
    :return: 每个 span 的字段权重字典
    """
    t1 = time.time()
    tgid_outgoing = set() # 记录tgid是否有出span
    for span in spans:
        if span.direction == 'Egress':
            tgid_outgoing.add(span.tgid)
    global span_fields
    span_fields = {}
    field_to_spans = defaultdict(set)  # 记录每个字段在哪些 spans 中出现
    all_fields = Counter()

    # 计算每个 span 的 n-gram 计数，并记录字段出现的 spans
    for span in spans:
        if span.tgid not in tgid_outgoing:
            continue
        content = span.req_content + span.resp_content
        ngrams = [content[i:i + window_size] for i in range(len(content) - window_size + 1)]
        span_fields[span] = Counter(ngrams)
        all_fields.update(span_fields[span])
        for ngram in span_fields[span]:
            field_to_spans[ngram].add(span)

    # 计算 IDF（逆文档频率）
    num_spans = len(spans)
    idf = {field: math.log(num_spans / (1 + len(spans_set)))  # 平滑处理，避免除以 0
           for field, spans_set in field_to_spans.items()}

    # 更新每个 span 的字段计数为 TF-IDF 权重
    for span, fields in span_fields.items():
        total_count = sum(fields.values())  # 计算总词频
        for field in fields:
            tf = fields[field] / total_count  # 计算 TF（词频）
            fields[field] = tf * idf[field]  # 计算 TF-IDF 权重

    t2 = time.time()
    print(f"Transaction field time: {t2 - t1:.4f} seconds")
    return span_fields

 
def field_similarity(span1, span2):
    """
    计算两个 span 的字段集合的余弦相似度。

    :param spanid1: 第一个 span 的 ID
    :param spanid2: 第二个 span 的 ID
    :param span_fields: 包含所有 span 字段权重的字典
    :return: 两个 span 的余弦相似度
    """
    global span_fields
    span1_fields = span_fields[span1]
    span2_fields = span_fields[span2]

    # 计算分子：字段权重的点积
    dot_product = sum(span1_fields[field] * span2_fields.get(field, 0) for field in span1_fields)

    # 计算分母：两个字段权重向量的模长
    magnitude1 = math.sqrt(sum(weight ** 2 for weight in span1_fields.values()))
    magnitude2 = math.sqrt(sum(weight ** 2 for weight in span2_fields.values()))

    # 避免除以 0
    if magnitude1 == 0 or magnitude2 == 0:
        return 0.0

    # 计算余弦相似度
    similarity = dot_product / (magnitude1 * magnitude2)
    return similarity

def get_endpoint_ship(spans, interval = 100):
    t1 = time.time()
    global endpoint_ships
    endpoint_ships = {}
    traffic = {} # tgid -> endpoint -> traffic
    index = 0
    for i in range(0, len(spans), interval):
        for span in spans[i:i+interval]:
            if span.tgid not in traffic:
                traffic[span.tgid] = {}
            if span.endpoint not in traffic[span.tgid]:
                traffic[span.tgid][span.endpoint] = [0 for _ in range(1 + len(spans) // interval)]
            traffic[span.tgid][span.endpoint][index] += 1
        index += 1
    for tgid in traffic.keys():
        endpoints = traffic[tgid].keys()
        for endpoint1 in endpoints:
            for endpoint2 in endpoints:
                if endpoint1 == endpoint2:
                    continue
                if tgid not in endpoint_ships:
                    endpoint_ships[tgid] = {}
                if (endpoint1, endpoint2) not in endpoint_ships[tgid]:
                    endpoint_ships[tgid][(endpoint1, endpoint2)] = 0
                
                x = traffic[tgid][endpoint1]
                y = traffic[tgid][endpoint2]
                # 使用 scipy.stats.pearsonr 计算皮尔森相关性
                # correlation, _ = pearsonr(x, y)
                correlation = cosine_similarity(x, y)
                endpoint_ships[tgid][(endpoint1, endpoint2)] = correlation
    t2 = time.time()
    print(f"Endpoint ship time: {t2 - t1:.4f} seconds")
    # for tgid in endpoint_ships.keys():
    #     for endpoint_pair in endpoint_ships[tgid].keys():
    #         print(f"traffic: {traffic[tgid][endpoint_pair[0]]} | {traffic[tgid][endpoint_pair[1]]}")
    #         print(f"TGID: {tgid} | Endpoint Pair: {endpoint_pair} | Correlation: {endpoint_ships[tgid][endpoint_pair]:.4f}")


def build_pdf(samples, weights, sample_type="1d"):
    """
    构建概率密度函数 (PDF)。

    :param samples: 样本点，1D 或 2D 数组。
                    - 如果是 1D：形状为 (n_samples,)
                    - 如果是 2D：形状为 (n_samples, 2)
    :param weights: 样本点的权重，形状为 (n_samples,)。
    :param sample_type: 样本类型，"1d" 表示一元，"2d" 表示二元。
    :return: PDF 函数，可用于计算任意点的密度。
    """
    if sample_type == "1d":
        if len(set(samples)) == 1:
            # 如果所有样本点相同，返回一个常数函数
            return lambda x: np.ones_like(x) * weights[0], 'error'
    if sample_type == "2d":
        if len(set(map(tuple, samples))) == 1:
            # 如果所有样本点相同，返回一个常数函数
            return lambda x: np.ones_like(x) * weights[0], 'error'
    # 确保权重归一化
    weights = np.array(weights)
    weights /= np.sum(weights)

    # 样本点转换为 NumPy 数组
    samples = np.array(samples)

    # 构建核密度估计 (KDE)
    if sample_type == "1d":
        # 一元样本
        kde = gaussian_kde(samples, weights=weights)
    elif sample_type == "2d":
        # 二元样本
        kde = gaussian_kde(samples.T, weights=weights)
    else:
        raise ValueError("Invalid sample_type. Must be '1d' or '2d'.")

    # 返回 PDF 函数
    return kde, 'success'

def get_multi_metrics(candidate_mappings):
    t1 = time.time()
    global multi_metrics
    multi_metrics = {}
    for tgid, mappings in candidate_mappings.items():
        parent_nums = {}
        for span1, span2 in mappings:
            if span2 not in parent_nums:
                parent_nums[span2] = 0
            parent_nums[span2] += 1
        for span1, span2 in mappings:
            if span1.tgid not in multi_metrics:
                multi_metrics[span1.tgid] = {}
            if (span1.endpoint, span2.endpoint) not in multi_metrics[span1.tgid]:
                multi_metrics[span1.tgid][(span1.endpoint, span2.endpoint)] = {}
                for metric in ['t1', 't2', 'dura', 'size']:
                    multi_metrics[span1.tgid][(span1.endpoint, span2.endpoint)][metric] = [[], []] # sample, weight
            for metric in ['t1', 't2', 'dura', 'size']:
                # print(multi_metrics[span1.tgid][(span1.endpoint, span2.endpoint)][metric][1])
                multi_metrics[span1.tgid][(span1.endpoint, span2.endpoint)][metric][1].append(1 / parent_nums[span2])
                if metric == 't1':
                    multi_metrics[span1.tgid][(span1.endpoint, span2.endpoint)][metric][0].append(span2.start_time - span1.start_time)
                elif metric == 't2':
                    multi_metrics[span1.tgid][(span1.endpoint, span2.endpoint)][metric][0].append(span1.end_time - span2.end_time)
                elif metric == 'dura':
                    multi_metrics[span1.tgid][(span1.endpoint, span2.endpoint)][metric][0].append([span1.duration, span2.duration])
                elif metric == 'size':
                    multi_metrics[span1.tgid][(span1.endpoint, span2.endpoint)][metric][0].append([span1.req_size, span2.resp_size])
    for tgid in multi_metrics.keys():
        for endpoint_pair in multi_metrics[tgid].keys():
            for metric in multi_metrics[tgid][endpoint_pair].keys():
                # print(f"TGID: {tgid} | Endpoint Pair: {endpoint_pair} | Metric: {metric} | Samples: {multi_metrics[tgid][endpoint_pair][metric][0]} | Weights: {multi_metrics[tgid][endpoint_pair][metric][1]}")
                if metric == 't1' or metric == 't2':
                    pdf, status = build_pdf(multi_metrics[tgid][endpoint_pair][metric][0], multi_metrics[tgid][endpoint_pair][metric][1], sample_type="1d")
                elif metric == 'dura' or metric == 'size':
                    pdf, status = build_pdf(multi_metrics[tgid][endpoint_pair][metric][0], multi_metrics[tgid][endpoint_pair][metric][1], sample_type="2d")
                # if status == 'error':
                #     print(f"Error in building PDF for TGID: {tgid} | Endpoint Pair: {endpoint_pair} | Metric: {metric}")
                multi_metrics[tgid][endpoint_pair][metric] = [pdf, status]
    t2 = time.time()
    print(f"Multi metrics time: {t2 - t1:.4f} seconds")
        

def adjust_weights(span_mappings):
    global tgid_weights
    global maping_scores
    tgid_weights = {}
    maping_scores = {} # tgid -> (span1, span2) -> type -> score
    for tgid, mappings in span_mappings.items():

        if tgid not in maping_scores:
            maping_scores[tgid] = {}
        for span1, span2 in mappings:
            if span1.tgid in endpoint_ships:
                endpoint_pair = (span1.endpoint, span2.endpoint)
            else:
                continue
            
            metrics = multi_metrics[span1.tgid].get(endpoint_pair)
            if (span1, span2) not in maping_scores[tgid]:
                maping_scores[tgid][(span1, span2)] = [0, 0, 0, 0, 0, 0] # endpoint, field, t1, t2, dura, size
            if endpoint_pair in endpoint_ships[span1.tgid]:
                maping_scores[tgid][(span1, span2)][0] = float(endpoint_ships[span1.tgid][endpoint_pair])
            maping_scores[tgid][(span1, span2)][1] = float(field_similarity(span1, span2))

            if metrics:
                # 遍历所有 metric
                for i, metric in enumerate(['t1', 't2', 'dura', 'size'], start=2):
                    pdf, status = metrics[metric]
                    if status == 'error':
                        continue  # 跳过错误的 PDF

                    # 根据 metric 类型计算分数
                    if metric == 't1':
                        value = span2.start_time - span1.start_time
                    elif metric == 't2':
                        value = span1.end_time - span2.end_time
                    elif metric == 'dura':
                        value = [span1.duration, span2.duration]
                    elif metric == 'size':
                        value = [span1.req_size, span2.req_size]
                    else:
                        continue
                    # 计算 PDF 分数
                    maping_scores[tgid][(span1, span2)][i] = float(pdf(value))
    
        
    for tgid in maping_scores.keys():
        if tgid not in tgid_weights:
            tgid_weights[tgid] = [0, 0, 0, 0, 0, 0]
         
        scores = [list(score_list) for score_list in maping_scores[tgid].values()]
        scores = np.array(scores)
        min_scores = np.min(scores, axis=0)  # 计算每一列的最小值
        max_scores = np.max(scores, axis=0)
        for mapping, m_scores in maping_scores[tgid].items():
            maping_scores[tgid][mapping] = (np.array(m_scores) - min_scores) / (max_scores - min_scores + 1e-10)
        scores = [list(score_list) for score_list in maping_scores[tgid].values()]
        # 计算每一列的方差
        variances = np.var(scores, axis=0)  # 计算每列的方差

        # 对方差进行归一化
        if np.sum(variances) > 0:
            normalized_variances = variances / np.sum(variances)  # 归一化
        else:
            normalized_variances = np.zeros_like(variances)  # 如果方差全为 0，则权重全为 0

        # 将归一化后的方差作为权重
        tgid_weights[tgid] = normalized_variances.tolist()
        # print(f"TGID: {tgid} | Weights: {tgid_weights[tgid]} | variances: {variances}")



def get_candidate_mappings(spans):
    tgid_spans = {}
    for span in spans:
        if span.tgid not in tgid_spans:
            tgid_spans[span.tgid] = []
        tgid_spans[span.tgid].append(span)
    candidates = {}
    for tgid, span_list in tgid_spans.items():
        span_list = sorted(span_list, key=lambda x: x.start_time)
        for i, span in enumerate(span_list):
            if span.direction == 'Egress':
                for pre_span in span_list[:i][::-1]:
                    if pre_span.direction == 'Ingress':
                        if tgid not in candidates:
                            candidates[tgid] = []
                        if pre_span.start_time < span.start_time and pre_span.end_time > span.start_time:
                            candidates[tgid].append((pre_span, span))
        if tgid in candidates:
            candidates[tgid] = sorted(candidates[tgid], key=lambda x: x[1].start_time)
    return candidates




def scoring(span1, span2):

    global tgid_weights
    global maping_scores
    if span1.tgid not in tgid_weights or span1.tgid not in maping_scores:
        return 0
    weights = tgid_weights[span1.tgid]
    scores = maping_scores[span1.tgid][(span1, span2)]
    return sum(w * s for w, s in zip(weights, scores))
    

def iterative(span_mappings):
    t1 = time.time()
    global span_fields
    child2parent = {}
    for tgid, mappings in span_mappings.items():
        
        span_parent = {} # 存储每个 出span 的备选父 span集合
        for span1, span2 in mappings:
            if span2 not in span_parent:
                span_parent[span2] = []
            span_parent[span2].append(span1)
        for outgoing_span, parent_spans in span_parent.items():
            if len(parent_spans) == 1:
                child2parent[outgoing_span.span_id] = list(parent_spans)[0].span_id
            else:
                socres = [scoring(parent_span, outgoing_span) for parent_span in parent_spans]
                max_index = socres.index(max(socres))
                # selected_mappings[tgid].append((list(parent_spans)[max_index], outgoing_span))
                child2parent[outgoing_span.span_id] = list(parent_spans)[max_index].span_id
    t2 = time.time()
    print(f"Iterative time: {t2 - t1:.4f} seconds")
    return child2parent

def deeptrace(spans):
    t1 = time.time()
    transaction_field(spans)
    get_endpoint_ship(spans)
    span_mappings = get_candidate_mappings(spans)
    get_multi_metrics(span_mappings)
    adjust_weights(span_mappings)
    # print(f"Candidate Mappings: {span_mappings}")
    child2parent = iterative(span_mappings)
    spans_dict = intra_preprocess(spans)
    for tgid, tgid_spans in spans_dict.items():
        for direction, span_list in tgid_spans.items():
            for ip in span_list:
                for span in span_list[ip]:
                    if span.span_id in child2parent:
                        span.parent_id = child2parent[span.span_id]
    t2 = time.time()
    print(f"DeepTrace time: {t2 - t1:.4f} seconds")
    return spans_dict

