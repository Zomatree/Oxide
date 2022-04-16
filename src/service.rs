use pyo3::types::PyTuple;
use pyo3_asyncio::{TaskLocals, into_future_with_locals};
use pyo3::prelude::*;
use futures::{Future};
use hyper::{service::Service, Request, Body, Response as HyperResponse, http::Error as HyperError};
use std::task;
use std::sync::Arc;
use std::pin::Pin;

use crate::routes::Routes;
use crate::response::Response;
use crate::exceptions::ResponseError;
use crate::result::FutureResult;

pub struct ServerService {
    pub routes: Routes,
    pub locals: TaskLocals
}

impl ServerService {
    pub fn get_handler(&self, request: Request<Body>) -> (Vec<String>, Option<Arc<Py<PyAny>>>) {
        let uri = request.uri();
        let method = request.method();
        let path = uri.path();

        let (path_parts, handler) = self.routes.get_route(path);

        let func = handler.and_then(|route| {
            match *method {
                hyper::Method::GET => route.get.clone(),
                hyper::Method::POST => route.post.clone(),
                hyper::Method::PUT => route.put.clone(),
                hyper::Method::DELETE => route.delete.clone(),
                hyper::Method::PATCH => route.patch.clone(),
                hyper::Method::OPTIONS => route.options.clone(),
                hyper::Method::HEAD => route.head.clone(),
                _ => None,
            }
        });

        (path_parts, func)
    }
}

impl Service<Request<Body>> for ServerService {
    type Response = HyperResponse<Body>;
    type Error = HyperError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut task::Context<'_>) -> task::Poll<Result<(), Self::Error>> {
        task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let (var_parts, handler) = self.get_handler(req);

        let locals = self.locals.clone();

        Box::pin(async move {
            match handler {
                Some(handler) => {
                    Python::with_gil(|py| {
                        handler
                            .call(py, PyTuple::new(py, var_parts), None)
                            .and_then(|res| into_future_with_locals(&locals, res.as_ref(py)))
                    })
                        .async_and_then(|fut| fut)
                        .await
                        .and_then(|res| Python::with_gil(|py| res.extract::<Response>(py)))
                        .and_then(|res| HyperResponse::builder()
                            .status(res.status)
                            .body(Body::from(res.body))
                            .map_err(|err| ResponseError::new_err(format!("{err:?}"))))
                        .or_else(|err| {
                            Python::with_gil(|py| err.print_and_set_sys_last_vars(py));

                            HyperResponse::builder()
                                .status(500)
                                .body(Body::from(""))
                        })
                },
                None => {
                    HyperResponse::builder()
                        .status(404)
                        .body(Body::from(""))
                }
            }
        })
    }
}
