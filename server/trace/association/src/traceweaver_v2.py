           
# traceweaver v2
from utils import Span, intra_preprocess
import copy
import math
import scipy.stats
import heapq
import networkx as nx
from tqdm import tqdm
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
        t1 = [s.start_time for s in incoming_spans][start:end]
        t2 = [s.start_time for s in outgoing_spans[ep2]][start:end]
        calDelayDist(ep1, ep2, t1, t2)

        # between outgoing -- outgoing
        for i in range(len(out_eps) - 1):
            ep1 = out_eps[i]
            ep2 = out_eps[i + 1]
            t1 = [s.end_time for s in outgoing_spans[ep1]][start:end]
            t2 = [s.start_time for s in outgoing_spans[ep2]][start:end]
            calDelayDist(ep1, ep2, t1, t2)

        # between last outgoing -- incoming
        ep1 = out_eps[-1]
        ep2 = in_ep
        t1 = [s.end_time for s in outgoing_spans[ep1]][start:end]
        t2 = [s.end_time for s in incoming_spans][start:end]
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

def findCandidateMappings(in_ep, out_eps, in_span, is_parallel):
    path = [in_span]
    candidates = in_span.candidates
    K = 8
    top_k_mappings = []

    global total_path_cnt
    total_path_cnt = 0

    def dfs(path):
        global total_path_cnt
        if len(path) == 1:
            last_span = path[-1]
            out_ep = out_eps[len(path) - 1]
            for i in range(len(candidates[out_ep])):
                out_span = candidates[out_ep][i]
                if span2used[out_span] == True:
                    continue
                if out_span.start_time < last_span.start_time or out_span.end_time > last_span.end_time:
                    continue
                dfs(path + [out_span])
        elif len(path) < len(out_eps) + 1:
            last_span = path[-1]
            out_ep = out_eps[len(path) - 1]
            for i in range(len(candidates[out_ep])):
                out_span = candidates[out_ep][i]
                if span2used[out_span] == True:
                    continue
                if out_span.start_time < last_span.end_time or out_span.end_time > in_span.end_time:
                    continue
                dfs(path + [out_span])
        else:
            score = calcScore(path, in_ep, out_eps, is_parallel)
            total_path_cnt += 1
            heapq.heappush(top_k_mappings, (score, path.copy()))
            if len(top_k_mappings) > K:
                heapq.heappop(top_k_mappings)
    dfs(path)

    top_k_mappings.sort(reverse=True)
    return top_k_mappings, total_path_cnt

def calcMIS(all_candidate_mappings):
    # create a graph
    graph = nx.Graph()
    # all_candidate_mappings[nod_index][mapping_id] = (score, path)
    for i in range(len(all_candidate_mappings)):
        top_k_mappings = all_candidate_mappings[i]
        for j in range(len(top_k_mappings)):
            # add nodes
            weight, mapping = top_k_mappings[j]
            index = (i, j)
            graph.add_node(index, weight=10000.0 + weight)
            # print(f"{index}: {weight}")
            # mappings for the same in_span
            for k in range(j):
                prev_index = (i, k)
                graph.add_edge(prev_index, index)
            # mappings which have the same out_span
            for m in range(i):
                for n in range(len(all_candidate_mappings[m])):
                    prev_mapping = all_candidate_mappings[m][n][1]
                    for k in range(1, len(prev_mapping)):
                        if prev_mapping[k] == mapping[k]:
                            prev_index = (m, n)
                            graph.add_edge(prev_index, index)
                            break

    if len(graph.nodes) == 0:
        return {}
    # solve MIS
    best_mis = None
    best_score = -math.inf
    for i in range(20000):
        mis = nx.maximal_independent_set(graph)
        score = sum([graph.nodes[n]['weight'] for n in mis])
        if score > best_score or best_mis is None:
            best_mis = mis
            best_score = score

    # get the best mapping
    mappings = {}
    for in_span_id, mapping_id in best_mis:
        score, mapping = all_candidate_mappings[in_span_id][mapping_id]
        mappings[in_span_id] = mapping
    return mappings

def addCandidateSpans(in_span, out_eps, out_spans):
    in_span.candidates = {}
    for out_ep in out_eps:
        in_span.candidates[out_ep] = []
        for out_span in out_spans[out_ep]:
            if out_span.start_time > in_span.end_time:
                break
            if out_span.start_time < in_span.start_time or out_span.end_time > in_span.end_time:
                continue
            if span2used[out_span] == True:
                continue
          
            in_span.candidates[out_ep].append(out_span)

def isBatchEnd(in_span1, incoming_spans, ind, out_eps):
    prev_spans = incoming_spans[:ind]
    if not prev_spans:
        return False 

    prev_span = max(prev_spans, key=lambda x: x.end_time)
    for out_ep in out_eps:
        for c1 in in_span1.candidates[out_ep]:
            for c2 in prev_span.candidates[out_ep]:
                if c1 == c2:
                    return False

    if prev_span.end_time < in_span1.end_time:
        return True
    return False


# spans[tgid][direction][dst_ip] = span_list
# TODO: parallel
def traceweaver_v2(spans, is_parallel=False):
    processed_spans = {}
    global delay_dists
    delay_dists = {}
    span_dict = intra_preprocess(spans)
    spanid2parentid = {}

    bacth_size = 100
    bacth_size_mis = 30

    out_eps = getOutEpsInOrder(span_dict)
    # batching
    for tgid, tgid_spans in span_dict.items():
        if len(tgid_spans['incoming']) == 0 or len(tgid_spans['outgoing']) == 0:
            continue

        processed_spans[tgid] = []
        total_cnt = 0

        outgoing_spans = tgid_spans['outgoing']
        for out_ep in outgoing_spans.keys():
            outgoing_spans[out_ep].sort(key=lambda x: (x.start_time, x.end_time))

        for in_ep, incoming_spans in tgid_spans['incoming'].items():
            cnt = 0
            all_candidate_mappings = []

            incoming_spans.sort(key=lambda x: (x.start_time, x.end_time))
            # find cadidate spans
            for span in incoming_spans:
                addCandidateSpans(span, out_eps, outgoing_spans)
               
            
            for i in range(len(incoming_spans)):
                span = incoming_spans[i]
                # construct delay distributions
                if i % bacth_size == 0:
                    # TODO: update delay distributions using GMM   
                    calcDelayDists(in_ep, out_eps, incoming_spans, outgoing_spans, 
                                i, min(i + bacth_size, len(incoming_spans)), is_parallel) 
                # ranking candidate mappings using delay distributions
                top_k_mappings, total_path_cnt = findCandidateMappings(in_ep, out_eps, span, is_parallel)
                all_candidate_mappings.append(top_k_mappings)

                # joint optimization
                if (i + 1) % bacth_size_mis == 0 or i == len(incoming_spans) - 1 or \
                isBatchEnd(incoming_spans[i+1], incoming_spans, i+1, out_eps):
                    mappings = calcMIS(all_candidate_mappings)
                    # print(f"MIS: {len(mappings)} / {len(all_candidate_mappings)} ")
                    for in_span_id, mapping in mappings.items():
                        total_cnt += len(mapping) - 1
                        # mapping[0].used = True
                        span2used[mapping[0]] = True
                        for span in mapping[1:]:
                            # span.association_tracid = mapping[0].trace_id
                            spanid2parentid[span.span_id] = mapping[0].span_id
                            # span.used = True
                            span2used[span] = True

                    all_candidate_mappings = []
                    



        # for out_ep, out_spans in outgoing_spans.items():
        #     for span in out_spans:
        #         # print(f"{span.association_tracid} -> {span.trace_id} {span.association_tracid == span.trace_id}")
        #         processed_spans[tgid].append(span)
        
        print(f"{tgid}: {total_cnt}/{2*287}")
    for tgid, tgid_spans in span_dict.items():
        if len(tgid_spans['incoming']) == 0 or len(tgid_spans['outgoing']) == 0:
            continue
        for out_ep, out_spans in tgid_spans['outgoing'].items():
            for span in out_spans:
                span.parent_id = spanid2parentid.get(span.span_id, None)
    return span_dict