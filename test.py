from typing import Optional
from oxide import Server, Route, Response, Request
import asyncio

server = Server()

class Echo(Route):
    async def get(self, request: Request, param: str):
        print(request.raw_body)
        return Response(body=param, status=200)

server.add_route('/echo/:param', Echo)

@server.middleware
async def echo_middleware(request: Request) -> Optional[Response]:
    print(request.method, request.path, request.var_parts)
    request.raw_body = b"hello"

    if len(request.var_parts[0]) < 2:
        return Response(body='Too Short :pepepoint:', status=400)

async def main():
    await server.start("127.0.0.1:8001")

asyncio.run(main())
