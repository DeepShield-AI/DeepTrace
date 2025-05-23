from utils import Span, intra_preprocess
import copy
import statistics
import scipy

mean_delay = {}
def EstimateDelay(in_ep, out_ep, in_spans, out_spans):
    global mean_delay
    spans = in_spans + out_spans
    spans.sort(key=lambda x: x.start_time)

    samples = []
    for i, span in enumerate(spans):
        if span.span_kind == 'incoming':
            continue
        
        for prev_span in reversed(spans[:i]):
            if prev_span.span_kind == 'incoming':
                samples.append(span.start_time - prev_span.start_time)
                break

    mean_delay[in_ep, out_ep] = statistics.mean(samples)

def ScoreParents(in_ep, out_ep, in_spans, out_spans, largest_delay):
    spans = in_spans + out_spans
    spans.sort(key=lambda x: x.start_time)

    mean = mean_delay[(in_ep, out_ep)]
    for i, span in enumerate(spans):
        if span.span_kind == 'incoming':
            continue
        
        candidates = []
        for prev_span in reversed(spans[:i]):
            if span.start_time - prev_span.start_time > 4 * largest_delay:
                break
            if prev_span.span_kind == 'outgoing':
                continue
            
            span.association_tracid  = prev_span.trace_id
            p = scipy.stats.expon.logpdf(span.start_time - prev_span.start_time, scale=mean)
            candidates.append((p, prev_span))
        
        candidates.sort(key=lambda x: x[0], reverse=True)
        if len(candidates) != 0:
            score, parent = candidates[0]
            span.association_tracid  = parent.trace_id
            span.parent_id = parent.span_id


# 
def wap5(spans):
    global mean_delay
    mean_delay = {}
    span_dict = intra_preprocess(spans)
    for tgid, tgid_spans in span_dict.items():
        incoming_spans = tgid_spans['incoming']
        outgoing_spans = tgid_spans['outgoing']

        for in_ep, in_spans in incoming_spans.items():
            largest_delay = max([span.end_time - span.start_time for span in in_spans])
            for in_span in in_spans:
                in_span.span_kind = 'incoming'
                
            for out_ep, out_spans in outgoing_spans.items():
                for out_span in out_spans:
                    out_span.span_kind = 'outgoing'

                EstimateDelay(in_ep, out_ep, in_spans, out_spans)
                ScoreParents(in_ep, out_ep, in_spans, out_spans, largest_delay)
    
    return span_dict

