use std::pin::Pin;
use std::task;
use futures::Future;
use hyper::{Error as HyperError, service::Service};

use crate::middleware::MiddlewareWrapper;
use crate::routes::Routes;
use crate::service::ServerService;

pub struct MakeService {
    pub routes: Routes,
    pub middleware: Vec<MiddlewareWrapper>,
}

impl<T> Service<T> for MakeService {
    type Response = ServerService;
    type Error = HyperError;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut task::Context) -> task::Poll<Result<(), Self::Error>> {
        task::Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: T) -> Self::Future {
        let routes = self.routes.clone();
        let middleware = self.middleware.clone();

        Box::pin(async move {
            Ok(ServerService { routes, middleware })
        })
    }
}
