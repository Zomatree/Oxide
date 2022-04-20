from typing import Any, Callable, TypeAlias, Coroutine

Middleware: TypeAlias = Callable[[Request], Coroutine[Any, Any, Response | None]]

class Method:
    GET = "GET"
    POST = "POST"
    PUT = "PUT"
    DELETE = "DELETE"
    PATCH = "PATCH"
    HEAD = "HEAD"
    OPTIONS = "OPTIONS"
    CONNECT = "CONNECT"
    TRACE = "TRACE"
    OTHER = "OTHER"

class Request:
    raw_body: bytes
    path: str
    method: Method
    headers: dict[str, str]
    params: dict[str, str]
    var_parts: list[str]

class Response:
    body: str
    status: int

    def __init__(self, body: str, status: int): ...

class Server:
    def __init__(self): ...
    def add_route(self, path: str, route: type, **kwargs: Any): ...
    def add_middleware(self, middleware: Middleware): ...
    def middleware(self, middleware: Middleware) -> Middleware: ...
    async def start(self, host: str): ...

class Route:
    async def setup(self): ...
    async def get(self, request: Request, *args: str): ...
    async def post(self, request: Request, *args: str): ...
    async def put(self, request: Request, *args: str): ...
    async def delete(self, request: Request, *args: str): ...
    async def patch(self, request: Request, *args: str): ...
    async def options(self, request: Request, *args: str): ...
    async def head(self, request: Request, *args: str): ...
