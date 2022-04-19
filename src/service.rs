use pyo3::prelude::*;
use futures::{Future};
use hyper::{service::Service, Request, Body, Response as HyperResponse, http::Error as HyperError};
use std::task;
use std::pin::Pin;

use crate::routes::Routes;
use crate::response::Response;
use crate::exceptions::ResponseError;

pub struct ServerService {
    pub routes: Routes,
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
        let method = req.method().clone();

        Box::pin(async move {
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
                        .call_method(method, var_parts)
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
