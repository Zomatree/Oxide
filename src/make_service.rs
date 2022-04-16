use std::pin::Pin;
use std::task;
use futures::Future;
use hyper::{Error as HyperError, service::Service};
use pyo3_asyncio::{TaskLocals};

use crate::routes::Routes;
use crate::service::ServerService;

pub struct MakeService {
    pub routes: Routes,
    pub locals: TaskLocals
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
        let locals = self.locals.clone();
        Box::pin(async move {
            Ok(ServerService { routes, locals })
        })
    }
}
