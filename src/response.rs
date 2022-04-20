use hyper::{Response, Body, http::Error as HyperHttpError};
use pyo3::prelude::*;

#[pyclass(name = "Response")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PyResponse {
    pub body: String,
    pub status: u16,
}

impl PyResponse {
    pub fn into_hyper(self) -> Result<Response<Body>, HyperHttpError> {
        Response::builder()
            .status(self.status)
            .body(Body::from(self.body))
    }
}

#[pymethods]
impl PyResponse {
    #[new]
    fn new(body: String, status: u16) -> Self {
        PyResponse {
            body,
            status,
        }
    }
}
