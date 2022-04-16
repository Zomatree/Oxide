use pyo3::prelude::*;

mod exceptions;
mod make_service;
mod response;
mod result;
mod routes;
mod server;
mod service;

#[pymodule]
fn oxide(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<server::BaseServer>()?;
    m.add_class::<response::Response>()?;
    Ok(())
}
