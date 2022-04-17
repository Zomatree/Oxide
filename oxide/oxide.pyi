from typing import Any

class Response:
    body: str
    status: int

    def __init__(self, body: str, status: int): ...

class Server:
    def __init__(self): ...
    def add_route(self, path: str, route: type, **kwargs: Any): ...
    async def start(self, host: str): ...
