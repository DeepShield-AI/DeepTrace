import random
import string
import redis
from concurrent.futures import ThreadPoolExecutor

pool = redis.ConnectionPool(
    host='127.0.0.1',
    port=6379,
    max_connections=10,
    decode_responses=True
)

def execute_redis_command():
    with redis.Redis(connection_pool=pool) as r:
        conn = r.connection_pool.get_connection('_')
        try:
            key = random_string()
            r.set(key, key)
            command_args = ["GETRANGE", key, "0", "24"]
            conn.send_command(*command_args)
            conn.read_response()
            
            log_data = conn.request_log[-2:]
            return {
                'command': ' '.join(command_args),
                'request': log_data[0],
                'response': log_data[1],
            }
        finally:
            pool.release(conn)

def random_string():
    return ''.join(random.choices(string.ascii_letters + string.digits, k=24))

def redis_client():
    with ThreadPoolExecutor(max_workers=4) as executor:
        futures = []
        for _ in range(1000):
            futures.append(executor.submit(execute_redis_command))


if __name__ == '__main__':
    redis_client()
                