from re import T
from typing import Callable, Coroutine, Any

class Response:
    body: str
    status: int

    def __init__(self, body: str, status: int): ...

class BaseServer:
    def __init__(self): ...
    def _route(self, path: str, methods: list[str], callback: Callable[..., Coroutine[Any, Any, Response]]): ...
    async def start(self, host: str): ...