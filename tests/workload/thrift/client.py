from thrift.protocol import TBinaryProtocol
from thrift.transport import TSocket, TTransport
import sys
import os
sys.path.append(os.path.abspath('gen-py'))
from .echo import EchoService
import string
import random

def generate_random_string(length=10):
    chars = string.ascii_letters + string.digits
    return ''.join(random.choice(chars) for _ in range(length))

def thrift_client():
    transport = TSocket.TSocket('127.0.0.1', 9090)
    transport = TTransport.TBufferedTransport(transport)
    protocol = TBinaryProtocol.TBinaryProtocol(transport)
    client = EchoService.Client(protocol)

    transport.open()
    for _ in range(1000):
        message = generate_random_string(24)
        response = client.echo(message)
        print(f"response: {response}")
    transport.close()

if __name__ == '__main__':
    thrift_client()
