# Oxide

A Python HTTP server in Rust

## Example

```python
import oxide
import asyncio

server = oxide.Server()

@server.route("/echo/:arg", ["GET"])
async def echo(arg: str):
    return oxide.Response(body=arg, status=200)

async def main():
    await server.start("127.0.0.1:8001")

asyncio.run(main())
```
