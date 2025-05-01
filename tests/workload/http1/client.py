import aiohttp
import asyncio
import random
import string

def generate_random_string(length=10):
    chars = string.ascii_letters + string.digits
    return ''.join(random.choice(chars) for _ in range(length))

async def send_async_request(session, data):
    url = 'http://localhost:8000/echo'
    headers = {'Payload': data}
    async with session.post(url, data=data, headers=headers) as response:
        pass

async def request():
    data_list = [generate_random_string(24) for _ in range(1000)]
    async with aiohttp.ClientSession() as session:
        tasks = [send_async_request(session, data) for data in data_list]
        await asyncio.gather(*tasks)

def http1_client():
    asyncio.run(request())

if __name__ == '__main__':
    http1_client()