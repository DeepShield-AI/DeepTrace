import socket
import threading

HOST = '127.0.0.1'
PORT = 8080

def handle_client(conn, addr):
    # print(f"connect from {addr}")
    with conn:
        while True:
            try:
                request = b''
                while b'\r\n\r\n' not in request:
                    data = conn.recv(4096)
                    if not data:
                        # print(f"connection {addr} closed.")
                        return
                    request += data
                header_data = request.split(b'\r\n\r\n')[0].decode()
                headers = header_data.split('\r\n')
                request_line = headers[0]
                method, path, version = request_line.split()

                if method == 'GET':
                    body = "Hello, this is a response from the server. " * 500
                    response = (
                        f"HTTP/1.1 200 OK\r\n"
                        f"Content-Length: {len(body.encode())}\r\n"
                        f"Content-Type: text/plain\r\n"
                        f"Connection: keep-alive\r\n"
                        f"\r\n"
                        f"{body}"
                    )
                    conn.sendall(response.encode())
                elif method == 'POST':
                    content_length = 0
                    for header in headers[1:]:
                        if header.lower().startswith('content-length'):
                            content_length = int(header.split(':')[1].strip())
                            break
                    body = request.split(b'\r\n\r\n', 1)[1]
                    while len(body) < content_length:
                        data = conn.recv(4096)
                        if not data:
                            break
                        body += data
                    print(f"recv post request: {len(body)} bytes from {addr}")
                    response_body = "POST request received. " * 500
                    response = (
                        f"HTTP/1.1 200 OK\r\n"
                        f"Content-Length: {len(response_body.encode())}\r\n"
                        f"Content-Type: text/plain\r\n"
                        f"Connection: keep-alive\r\n"
                        f"\r\n"
                        f"{response_body}"
                    )
                    conn.sendall(response.encode())
                else:
                    response = (
                        f"HTTP/1.1 405 Method Not Allowed\r\n"
                        f"Content-Length: 0\r\n"
                        f"Connection: close\r\n"
                        f"\r\n"
                    )
                    conn.sendall(response.encode())
                    return
            except Exception as e:
                print(f"{addr} error: {e}")
                break

def start_server():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind((HOST, PORT))
        s.listen()
        print(f"{HOST}:{PORT}")
        while True:
            conn, addr = s.accept()
            client_thread = threading.Thread(target=handle_client, args=(conn, addr))
            client_thread.daemon = True
            client_thread.start()

if __name__ == "__main__":
    start_server()
