from trace.association.src.cross import inter_association
from database.src.utils import es_read_span_list

if __name__ == "__main__":

    for rps in [100, 200, 300, 400, 500]:
        index_name = f"test-rps-{rps}-spans"
        spans = es_read_span_list(index_name)
        spans = inter_association(spans, client_ingress = None, tuple_used=True, tuple_direction=True)


        








