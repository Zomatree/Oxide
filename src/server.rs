use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3_asyncio::tokio::{get_current_locals, future_into_py};
use hyper::Server as HyperServer;
use std::net::SocketAddr;

use crate::routes::{Routes, Route};
use crate::make_service::MakeService;
use crate::exceptions::BindingError;

#[pyclass(subclass, dict)]
#[derive(Debug, Clone, Default)]
pub struct Server {
    pub routes: Routes,
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

    fn start<'a>(&'a self, py: Python<'a>, host: String) -> PyResult<&'a PyAny> {
        let routes = self.routes.clone();
        let locals = get_current_locals(py)?;

        future_into_py(py, async move {
            let addr: SocketAddr = host.parse().unwrap();
            let server = HyperServer::try_bind(&addr)
                .map_err(|_| BindingError::new_err("could not bind to address"))?
                .serve(MakeService { routes, locals });
            println!("running on {addr}");
            server.await.unwrap();

            Ok(())
        })
    }
}
