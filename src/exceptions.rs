use pyo3::{create_exception, exceptions::PyException};

create_exception!(oxide, WebError, PyException, "base error for the web module");
create_exception!(oxide, WebServerError, WebError, "error while starting the server");
create_exception!(oxide, BindingError, WebServerError, "error while binding to the address");
create_exception!(oxide, ResponseError, WebError, "error while creating the response");
