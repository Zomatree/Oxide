use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3_asyncio::tokio::{get_current_locals, future_into_py};
use hyper::Server as HyperServer;
use std::net::SocketAddr;

use crate::LOCAL_TASKS;
use crate::middleware::MiddlewareWrapper;
use crate::routes::{Routes, Route};
use crate::make_service::MakeService;
use crate::exceptions::{BindingError, WebError};

#[pyclass(subclass, dict)]
#[derive(Debug, Clone, Default)]
pub struct Server {
    pub routes: Routes,
    pub middleware: Vec<MiddlewareWrapper>
}

#[pymethods]
impl Server {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    #[args(kwargs="**")]
    pub fn add_route(&mut self, path: String, cls: PyObject, kwargs: Option<Py<PyDict>>) {
        let route = Route::new(path, cls, kwargs);
        self.routes.add_route(route);
    }

    pub fn add_middleware(&mut self, middleware: PyObject) {
        self.middleware.push(MiddlewareWrapper::new(middleware));
    }

    pub fn middleware(&mut self, middleware: PyObject) -> PyObject {
        self.add_middleware(middleware);
        self.middleware.last().unwrap().0.clone()
    }

    fn start<'a>(&'a self, py: Python<'a>, host: String) -> PyResult<&'a PyAny> {
        let routes = self.routes.clone();
        let middleware = self.middleware.clone();

        let locals = get_current_locals(py)?;

        LOCAL_TASKS
            .set(locals)
            .unwrap();

        future_into_py(py, async move {
            let addr: SocketAddr = host.parse().unwrap();
            let server = HyperServer::try_bind(&addr)
                .map_err(|_| BindingError::new_err("could not bind to address"))?
                .serve(MakeService { routes, middleware });

            println!("running on {addr}");

            server
                .await
                .map_err(|e| WebError::new_err(format!("{e:?}")))
        })
    }
}
