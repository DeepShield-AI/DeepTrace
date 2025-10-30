import socket
import time

HOST = '127.0.0.1'
PORT = 8080

def send_request(sock, method, path, body=None):
    if body:
        request = (
            f"{method} {path} HTTP/1.1\r\n"
            f"Host: {HOST}:{PORT}\r\n"
            f"Content-Length: {len(body.encode())}\r\n"
            f"Content-Type: text/plain\r\n"
            f"Connection: keep-alive\r\n"
            f"\r\n"
            f"{body}"
        )
    else:
        request = (
            f"{method} {path} HTTP/1.1\r\n"
            f"Host: {HOST}:{PORT}\r\n"
            f"Connection: keep-alive\r\n"
            f"\r\n"
        )
    sock.sendall(request.encode())

def receive_response(sock):
    response = b''
    while True:
        data = sock.recv(4096)
        if not data:
            break
        response += data
        if b'\r\n\r\n' in response:
            header, remaining = response.split(b'\r\n\r\n', 1)
            headers = header.decode().split('\r\n')
            content_length = 0
            for h in headers:
                if h.lower().startswith('content-length'):
                    content_length = int(h.split(':')[1].strip())
                    break
            body = remaining
            while len(body) < content_length:
                data = sock.recv(4096)
                if not data:
                    break
                body += data
            return header.decode(), body.decode()

def client():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
        sock.connect((HOST, PORT))
        for i in range(50):
            if i % 2 == 0:
                send_request(sock, 'GET', '/')
            else:
                large_body = "This is a large POST request body. " * 500  # 大约 14KB
                print(f"\nPOST request #{i+1}, length: {len(large_body.encode())} bytes")
                send_request(sock, 'POST', '/submit', body=large_body)
            
            header, body = receive_response(sock)
            print(f"header:\n{header}")
            print(f"reponse length: {len(body.encode())} bytes")
            time.sleep(1)

        sock.close()

if __name__ == "__main__":
    client()
