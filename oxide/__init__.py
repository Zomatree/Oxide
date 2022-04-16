from .oxide import *
from typing import Callable, Any, Coroutine

class Server(BaseServer):
    def route(self, path: str, methods: list[str]) -> Callable[[Callable[..., Coroutine[Any, Any, Response]]], None]:
        def wrapper(callback: Callable[..., Coroutine[Any, Any, Response]]):
            self._route(path, methods, callback)

        return wrapper
