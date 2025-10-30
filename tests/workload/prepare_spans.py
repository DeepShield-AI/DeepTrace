from .redis.client import redis_client
# from .http1.client import http1_client
# from .thrift.client import thrift_client
from .memcached.client import memcached_client

workloads = { 
    "redis": redis_client, 
    # "http": http1_client,
#   "thrift": thrift_client 
    "memcached": memcached_client
    }

def load_workloads():
    for name, client in workloads.items():
        try:
            client()
            print(f"{name} workload completed successfully.")
        except Exception as e:
            print(f"Error during {name} workload: {e}")


if __name__ == "__main__":
    load_workloads()