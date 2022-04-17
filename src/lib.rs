use pyo3::prelude::*;
use pyo3_asyncio::tokio::future_into_py;

mod exceptions;
mod make_service;
mod response;
mod result;
mod routes;
mod server;
mod service;

#[pyclass(subclass, dict, name="Route")]
struct PyRoute;

#[pymethods]
impl PyRoute {
    #[new]
    fn new() -> Self {
        Self
    }

    fn setup<'a>(&self, py: Python<'a>) -> PyResult<&'a PyAny> {
        future_into_py(py, async {
            Ok(())
        })
    }
}

#[pymodule]
fn oxide(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<server::Server>()?;
    m.add_class::<response::Response>()?;
    m.add_class::<PyRoute>()?;
    Ok(())
}
