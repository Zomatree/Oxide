use hyper::{Request, Body, Method, body::to_bytes};
use pyo3::{prelude::*, types::{PyBytes, PyDict, PyList}};

#[pyclass]
pub enum PyMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
    Connect,
    Trace,
    Other
}

impl From<&Method> for PyMethod {
    fn from(method: &Method) -> Self {
        match *method {
            Method::GET => PyMethod::Get,
            Method::POST => PyMethod::Post,
            Method::PUT => PyMethod::Put,
            Method::DELETE => PyMethod::Delete,
            Method::PATCH => PyMethod::Patch,
            Method::HEAD => PyMethod::Head,
            Method::OPTIONS => PyMethod::Options,
            Method::CONNECT => PyMethod::Connect,
            Method::TRACE => PyMethod::Trace,
            _ => PyMethod::Other,
        }
    }
}

#[pyclass(name = "Request")]
#[derive(Clone)]
pub struct PyRequest {
    #[pyo3(get, set)]
    pub raw_body: Py<PyBytes>,
    #[pyo3(get, set)]
    pub path: String,
    #[pyo3(get, set)]
    pub method: Py<PyAny>,
    #[pyo3(get, set)]
    pub headers: Py<PyDict>,
    #[pyo3(get, set)]
    pub params: Py<PyDict>,
    #[pyo3(get, set)]
    pub var_parts: Py<PyList>
}

impl PyRequest {
    pub async fn new(request: Request<Body>, var_parts: &[String]) -> Self {
        let uri = request.uri();
        let path = uri.path().to_string();
        let method = Python::with_gil(|py| PyMethod::from(request.method()).into_py(py));
        let headers = Python::with_gil(|py| request
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap().to_string()))
            .fold(PyDict::new(py), |dict, (k, v)| {
                dict.set_item(k, v).unwrap();
                dict
            })
            .into_py(py)
        );

        let params = Python::with_gil(|py| uri
            .query()
            .unwrap_or("").split('&').fold(PyDict::new(py), |params, param| {
                let mut parts = param.split('=');
                let key = parts.next().unwrap();
                let value = parts.next().unwrap_or("");

                params.set_item(key, value).unwrap();
                params
            })
            .into_py(py)
        );

        let bytes = to_bytes(request.into_body()).await;
        let raw_body = Python::with_gil(|py| PyBytes::new(py, bytes.as_ref().unwrap()).into_py(py));
        let var_parts = Python::with_gil(|py| PyList::new(py, var_parts).into_py(py));

        PyRequest {
            raw_body,
            path,
            method,
            headers,
            params,
            var_parts
        }
    }
}
