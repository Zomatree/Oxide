use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Response {
    pub body: String,
    pub status: u16,
}

#[pymethods]
impl Response {
    #[new]
    fn new(body: String, status: u16) -> Self {
        Response {
            body,
            status,
        }
    }
}
