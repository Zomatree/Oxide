use pyo3::prelude::*;
use futures::{Future};
use hyper::{service::Service, Request, Body, Response as HyperResponse, http::Error as HyperError};
use std::task;
use std::pin::Pin;

use crate::middleware::MiddlewareWrapper;
use crate::request::PyRequest;
use crate::routes::Routes;
use crate::response::PyResponse;
use crate::exceptions::ResponseError;

pub struct ServerService {
    pub routes: Routes,
    pub middleware: Vec<MiddlewareWrapper>,
}

impl Service<Request<Body>> for ServerService {
    type Response = HyperResponse<Body>;
    type Error = HyperError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut task::Context<'_>) -> task::Poll<Result<(), Self::Error>> {
        task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let (var_parts, route) = self.routes.get_route(req.uri().path());
        let middleware = self.middleware.clone();

        Box::pin(async move {
            let request = PyRequest::new(req, &var_parts).await;
            let py_request = Python::with_gil(|py| Py::new(py, request)).unwrap();

            for middleware in middleware {
                let response = middleware
                    .call(Python::with_gil(|py| py_request.clone_ref(py)))
                    .await;

                match response {
                    Ok(Some(response)) => return response.into_hyper(),
                    Ok(None) => continue,
                    Err(err) => {
                        Python::with_gil(|py| err.print_and_set_sys_last_vars(py));

                        return HyperResponse::builder()
                            .status(500)
                            .body(Body::from("500: Internal Server Error"))
                    },
                }
            };

            match route {
                Some(route) => {
                    let instance = route.create_instance();

                    if let Err(err) = instance.setup().await {
                        Python::with_gil(|py| err.print_and_set_sys_last_vars(py));

                        return HyperResponse::builder()
                            .status(500)
                            .body(Body::from("500: Internal Server Error"))
                    };

                    instance
                        .call_method(py_request, var_parts)
                        .await
                        .and_then(|res| Python::with_gil(|py| res.extract::<PyResponse>(py)))
                        .and_then(|resp| resp
                            .into_hyper()
                            .map_err(|err| ResponseError::new_err(format!("{err:?}"))))
                        .or_else(|err| {
                            Python::with_gil(|py| err.print_and_set_sys_last_vars(py));

                            HyperResponse::builder()
                                .status(500)
                                .body(Body::from("500: Internal Server Error"))
                        })
                },
                None => {
                    HyperResponse::builder()
                        .status(404)
                        .body(Body::from("404: Not Found"))
                }
            }
        })
    }
}
