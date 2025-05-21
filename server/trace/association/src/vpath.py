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
        self.association_tracid = 0

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
    processed_spans = {}
    for tgid, span_list in spans.items():
        if len(span_list['incoming']) == 0 or len(span_list['outgoing']) == 0:
            continue

        output_span_count = 0
        processed_spans[tgid] = []

        # collect events
        all_events = {}
        for incoming_ip, incoming_spans in span_list['incoming'].items():
            for span in incoming_spans:
                if span.pid not in all_events:
                    all_events[span.pid] = []
                start_event = Event(span.pid, span.trace_id, span.start_time, "incoming", "request", 1, span)
                all_events[span.pid].extend([start_event])

                span.start_event = start_event
        for outgoing_ip, outgoing_spans in span_list['outgoing'].items():
            for span in outgoing_spans:
                if span.pid not in all_events:
                    all_events[span.pid] = []
                start_event = Event(span.pid, span.trace_id, span.start_time, "outgoing", "request", 2, span)
                all_events[span.pid].extend([start_event])

                span.start_event = start_event
            
        # match events between incoming_spans' (start_event, end_event) 
        for pid, events in all_events.items():
            all_events[pid] = sorted(events, key=lambda x: (float(x.event_time), x.sort_key))

        for incoming_ip, incoming_spans in span_list['incoming'].items():
            for span in incoming_spans:
                pid = span.pid
                start_time = span.start_time
                end_time = span.end_time
                trace_id = span.trace_id

                for event in all_events[pid]:
                    if event.event_time > end_time:
                        break
                    if event.event_time < start_time:
                        continue
                    
                    event.association_tracid = trace_id
                        

        for outgoing_ip, outgoing_spans in span_list['outgoing'].items():
            for span in outgoing_spans:
                start_event = span.start_event

                span.association_tracid = start_event.association_tracid
                processed_spans[tgid].append(span)

    return processed_spans