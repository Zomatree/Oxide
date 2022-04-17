import oxide
import asyncio

server = oxide.Server()

class Echo(oxide.Route):
    async def setup(self):
        print("setup")

    async def get(self, param: str):
        return oxide.Response(body=param, status=200)

server.add_route('/echo/:param', Echo)

async def main():
    await server.start("127.0.0.1:8001")

asyncio.run(main())
