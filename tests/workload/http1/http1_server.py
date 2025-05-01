from aiohttp import web

async def handle_post(request):
    payload = request.headers.get('Payload')
    
    headers = {
        "Content-Type": "text/plain",
        "Connection": "close",
        "Payload": payload
    }
    
    return web.Response(
        text="OK",
        headers=headers,
        status=200
    )

def http1_server(port=8000):
    app = web.Application()
    app.router.add_post('/echo', handle_post)
    
    print(f"Starting aiohttp server on port {port}")
    
    web.run_app(app, port=port)

if __name__ == '__main__':
    http1_server()