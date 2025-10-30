from thrift.protocol import TBinaryProtocol
from thrift.server import TServer
from thrift.transport import TSocket, TTransport
import sys
import os
sys.path.append(os.path.abspath('gen-py'))
from echo import EchoService

class EchoHandler:
    def echo(self, message):
        print(f"request: {message}")
        return message
    
def thrift_server():
    handler = EchoHandler()
    processor = EchoService.Processor(handler)
    transport = TSocket.TServerSocket('0.0.0.0', 9090)
    tfactory = TTransport.TBufferedTransportFactory()
    pfactory = TBinaryProtocol.TBinaryProtocolFactory()

    server = TServer.TSimpleServer(processor, transport, tfactory, pfactory)

    print("Echo Server started...")
    server.serve()
