use pyo3::prelude::*;
use pyo3_asyncio::tokio::{get_current_locals, future_into_py};
use hyper::Server as HyperServer;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::routes::{Routes, Route};
use crate::make_service::MakeService;
use crate::exceptions::BindingError;

#[pyclass(subclass, dict)]
#[derive(Debug, Clone, Default)]
pub struct BaseServer {
    pub routes: Routes,
}

#[pymethods]
impl BaseServer {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    fn _route(&mut self, path: String, methods: Vec<&str>, func: PyObject) {
        let handler = Arc::new(func);

        let mut route = Route::new(path);

        if methods.contains(&"GET") {
            route.get = Some(handler.clone());
        };

        if methods.contains(&"POST") {
            route.post = Some(handler.clone());
        };

        if methods.contains(&"PUT") {
            route.put = Some(handler.clone());
        };

        if methods.contains(&"DELETE") {
            route.delete = Some(handler.clone());
        };

        if methods.contains(&"PATCH") {
            route.patch = Some(handler.clone());
        };

        if methods.contains(&"OPTIONS") {
            route.options = Some(handler.clone());
        };

        if methods.contains(&"HEAD") {
            route.head = Some(handler);
        };

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
