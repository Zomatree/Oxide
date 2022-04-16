# Oxide

A Python HTTP server in Rust

## Example

```python
import web
import asyncio

server = web.Server()

@server.route("/echo/:arg", ["GET"])
async def echo(arg: str):
    return web.Response(body=arg, status=200)

async def main():
    await server.start("127.0.0.1:8001")

asyncio.run(main())
```
