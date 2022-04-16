import oxide
import asyncio

server = oxide.Server()

@server.route("/echo/:arg", ["GET"])
async def callback(arg: str):
    return oxide.Response(body=arg, status=200)

async def main():
    await server.start("127.0.0.1:8001")

asyncio.run(main())
