from utils import intra_preprocess
class Event(object):
    def __init__(
        self,
        pid,
        trace_id,
        event_time,
        span_kind,
        event_kind,
        sort_key,
        span,
    ):  
        self.pid = pid
        self.trace_id = trace_id # only for debugging
        self.event_time = event_time
        self.span_kind = span_kind
        self.event_kind = event_kind
        self.sort_key = sort_key
        self.span = span
        self.association_tracid = None
        self.parent_id = None

    def __repr__(self):
        return "Event:(%d, %s, %d, %s, %s)" % (
            self.pid,
            self.trace_id,
            self.event_time,
            self.span_kind,
            self.event_kind,
        )

    def __str__(self):
        return self.__repr__()


def vpath(spans):

    span_dict = intra_preprocess(spans)
    for tgid, span_list in span_dict.items():
        if len(span_list['incoming']) == 0 or len(span_list['outgoing']) == 0:
            continue

        output_span_count = 0

        span2event = {}
        # collect events
        all_events = {}
        for in_ip, in_spans in span_list['incoming'].items():
            for span in in_spans:
                if span.pid not in all_events:
                    all_events[span.pid] = []
                start_event = Event(span.pid, span.trace_id, span.start_time, "incoming", "request", 1, span)
                all_events[span.pid].extend([start_event])

                span2event[span] = start_event
        for out_ip, out_spans in span_list['outgoing'].items():
            for span in out_spans:
                if span.pid not in all_events:
                    all_events[span.pid] = []
                start_event = Event(span.pid, span.trace_id, span.start_time, "outgoing", "request", 2, span)
                all_events[span.pid].extend([start_event])


                span2event[span] = start_event
            
        # match events between incoming_spans' (start_event, end_event) 
        for pid, events in all_events.items():
            all_events[pid] = sorted(events, key=lambda x: (float(x.event_time), x.sort_key))

     
        for in_ip, in_spans in span_list['incoming'].items():
            for span in in_spans:
                pid = span.pid
                start_time = span.start_time
                end_time = span.end_time
                trace_id = span.trace_id
                span_id = span.span_id

                for event in all_events[pid]:
                    if event.event_time > end_time:
                        break
                    if event.event_time < start_time:
                        continue
                    
                    event.association_tracid = trace_id
                    # event.parent_id = span_id
                    for span, event2 in span2event.items():
                        if event == event2:
                            event2.parent_id = span.span_id
                            break
                        
            for outgoing_ip, outgoing_spans in span_list['outgoing'].items():
                for span in outgoing_spans:
                    start_event = span2event[span]
                    # span.association_tracid = start_event.association_tracid
                    span.parent_id = start_event.parent_id
                    # print(f"span_id: {span.span_id}, parent_id: {span.parent_id} start_event: {start_event.parent_id}")
    

    return span_dict
