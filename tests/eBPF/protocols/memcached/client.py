from concurrent.futures import ThreadPoolExecutor
import random
import string
import bmemcached

mc = bmemcached.Client(['localhost:11211'])

def random_string():
    return ''.join(random.choices(string.ascii_letters + string.digits, k=24))

def request():
    key = random_string()
    mc.set(key, key)
    mc.get(key)

def memcached_client():
    with ThreadPoolExecutor(max_workers=4) as executor:
        futures = []
        for _ in range(1000):
            futures.append(executor.submit(request)) 

if __name__ == '__main__':
    memcached_client()