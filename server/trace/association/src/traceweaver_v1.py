# traceweaver_v1

from utils import Span, intra_preprocess
import copy
import math
import scipy.stats
from collections import defaultdict

delay_dists = {}
span2used = defaultdict(lambda: False)

def getOutEpsInOrder(spans):
    out_eps = []
    for tgid, tgid_spans in spans.items():
       for dst_ip, outgoing_spans in tgid_spans['outgoing'].items():
            tgid_spans['outgoing'][dst_ip] = sorted(outgoing_spans, key=lambda x: (x.start_time, x.end_time))
            out_eps.append((dst_ip, tgid_spans['outgoing'][dst_ip][0].start_time))
    out_eps.sort(key=lambda x: x[1])
    return [x[0] for x in out_eps]

def calDelayDist(ep1, ep2, t1, t2):
    mean = (sum(t2) - sum(t1)) / len(t1)

    bucket_means = []
    bucket_num = 10
    bucket_size = math.ceil(float(len(t1)) / bucket_num)
    for i in range(bucket_num):
        start = i * bucket_size
        end = (i + 1) * bucket_size
        if end > len(t1):
            end = len(t1)
        bucket_means.append((sum(t2[start:end]) - sum(t1[start:end])) / (end - start))
    std = math.sqrt(bucket_size) * scipy.stats.tstd(bucket_means)

    delay_dists[(ep1, ep2)] = mean, std

def calcDelayDists(in_ep, out_eps, incoming_spans, outgoing_spans, start, end, is_parallel):
    if is_parallel:
        pass
    else:
        # between incoming -- first outgoing
        ep1 = in_ep
        ep2 = out_eps[0]
        t1 = sorted([s.start_time for s in incoming_spans])[start:end]
        t2 = sorted([s.start_time for s in outgoing_spans[ep2]])[start:end]
        calDelayDist(ep1, ep2, t1, t2)

        # between outgoing -- outgoing
        for i in range(len(out_eps) - 1):
            ep1 = out_eps[i]
            ep2 = out_eps[i + 1]
            t1 = sorted([s.end_time for s in outgoing_spans[ep1]])[start:end]
            t2 = sorted([s.start_time for s in outgoing_spans[ep2]])[start:end]
            calDelayDist(ep1, ep2, t1, t2)

        # between last outgoing -- incoming
        ep1 = out_eps[-1]
        ep2 = in_ep
        t1 = sorted([s.end_time for s in outgoing_spans[ep1]])[start:end]
        t2 = sorted([s.end_time for s in incoming_spans])[start:end]
        calDelayDist(ep1, ep2, t1, t2)

def calcScore(path, in_ep, out_eps, is_parallel):
    cost = 0
    for i in range(len(path)):
        if i == 0:
            ep1 = in_ep
            t1 = path[0].start_time
            ep2 = out_eps[0]
            t2 = path[1].start_time

        elif i == len(path) - 1:
            ep1 = out_eps[-1]
            t1 = path[-1].end_time
            ep2 = in_ep
            t2 =  path[0].end_time

        else:
            ep1 = out_eps[i - 1]
            t1 = path[i - 1].end_time
            ep2 = out_eps[i]
            t2 = path[i].start_time


        mean, std = delay_dists[(ep1, ep2)]
        p = scipy.stats.norm.logpdf(t2 - t1, loc=mean, scale=std)
        cost += p
   
    return cost

def joint_optimization_by_dfs(in_ep, out_eps, in_span, outgoing_spans, is_parallel):
    path = [in_span]
    index_path = [0]
    global best_score, best_mapping
    best_score = -100000.0
    best_mapping = None

    def dfs(path):
        global best_score, best_mapping

        if is_parallel:
            pass
        
        else:
            if len(path) == 1:
                last_span = path[-1]
                out_ep = out_eps[len(path) - 1]
                for i in range(len(outgoing_spans[out_ep])):
                    out_span = outgoing_spans[out_ep][i]
                    if span2used[out_span] == True:
                    # if out_span.used == True:
                        continue
                    if out_span.start_time < last_span.start_time or out_span.end_time > last_span.end_time:
                        continue
                    path.append(out_span)
                    index_path.append(0)
                    dfs(path)
                    break
            elif len(path) < len(out_eps) + 1:
                last_span = path[-1]
                out_ep = out_eps[len(path) - 1]
                for i in range(len(outgoing_spans[out_ep])):
                    out_span = outgoing_spans[out_ep][i]
                    if span2used[out_span] == True:
                    # if out_span.used == True:
                        continue
                    if out_span.start_time < last_span.end_time or out_span.end_time > in_span.end_time:
                        continue
                    path.append(out_span)
                    index_path.append(i)
                    dfs(path)
                    break
            else:
                score = calcScore(path, in_ep, out_eps, is_parallel)
                if score > best_score:
                    best_score = score
                    best_mapping = path
    
    dfs(path)

    if best_mapping is None:
        return None

    for span in best_mapping[1:]:
        span2used[span] = True
        # span.used = True

    return best_mapping[1:]


# spans[tgid][direction][dst_ip] = span_list
# TODO: parallel
def traceweaver_v1(spans, is_parallel=False):
    global delay_dists
    delay_dists = {}
    span_dict = intra_preprocess(spans)

    bacth_size = 100
    spanid2parentid = {}
    out_eps = getOutEpsInOrder(span_dict)
    # TODO: find candidates by using call graph, dependency order and time constraints
    # batching
    processed_cnt = 0
    for tgid, tgid_spans in span_dict.items():
        if len(tgid_spans['incoming']) == 0 or len(tgid_spans['outgoing']) == 0:
            continue


        outgoing_spans = copy.deepcopy(tgid_spans['outgoing'])

        for in_ep, incoming_spans in tgid_spans['incoming'].items():
            span_list = sorted(incoming_spans, key=lambda x: (x.start_time, x.end_time))

            cnt = 0
            for span in span_list:
                # TODO: batching condition when has candidates
                if cnt % bacth_size == 0:
                    # construct delay distributions
                    # TODO: update delay distributions using GMM   
                    calcDelayDists(in_ep, out_eps, incoming_spans, outgoing_spans, 
                                cnt, min(cnt + bacth_size, len(span_list)), is_parallel) 

                best_mapping = joint_optimization_by_dfs(in_ep, out_eps, span, outgoing_spans, is_parallel)
                if best_mapping is None:
                    continue
                for outgoing_span in best_mapping:
                    spanid2parentid[outgoing_span.span_id] = span.span_id
                    outgoing_span.parent_id = span.span_id
                    processed_cnt += 1
                
                cnt += 1
        
    # ranking candidate mappings

    # joint optimization
    for tgid, tgid_spans in span_dict.items():
        if len(tgid_spans['incoming']) == 0 or len(tgid_spans['outgoing']) == 0:
            continue
        for out_ep, outgoing_spans in tgid_spans['outgoing'].items():
            for span in outgoing_spans:
                span.parent_id = spanid2parentid.get(span.span_id, None)

    return span_dict

