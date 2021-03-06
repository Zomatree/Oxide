use pyo3::{prelude::*, types::{PyDict, PyTuple}};
use pyo3_asyncio::into_future_with_locals;
use std::sync::Arc;

use crate::{LOCAL_TASKS, request::PyRequest};

#[derive(Clone, Debug)]
pub enum PathPart {
    Static(String),
    Variable(String),
}

pub struct PyRouteWrapper(PyObject);

impl PyRouteWrapper {
    pub fn new(route: PyObject) -> Self {
        PyRouteWrapper(route)
    }

    pub async fn setup(&self) -> PyResult<PyObject> {
        Python::with_gil::<_, PyResult<_>>(|py| {
            let coro = self.0
                .getattr(py, "setup")?
                .call(py, (), None)?;

                into_future_with_locals(LOCAL_TASKS.get().unwrap(), coro.as_ref(py))
        })?.await
    }

    pub async fn call_method(&self, request: Py<PyRequest>, var_parts: Vec<String>) -> PyResult<PyObject> {
        let (method_name, args) = Python::with_gil(|py| {
            let method_name = request.getattr(py, "method").unwrap().to_string().split('.').last().unwrap().to_lowercase();

            let mut args = Vec::<PyObject>::with_capacity(var_parts.len() + 1);
            args.push(request.into_py(py));
            args.extend(var_parts.into_iter().map(|part| part.into_py(py)));

            (method_name, args)
        });

        Python::with_gil::<_, PyResult<_>>(|py| {
            let coro = self.0
                .getattr(py, &method_name)?
                .call(py, PyTuple::new(py, args), None)?;

                into_future_with_locals(LOCAL_TASKS.get().unwrap(), coro.as_ref(py))
        })?.await
    }
}

#[derive(Clone, Debug)]
pub struct Route {
    pub path_parts: Vec<PathPart>,
    pub python_cls: Py<PyAny>,
    kwargs: Option<Py<PyDict>>,
}

impl Route {
    pub fn new(path: String, cls: PyObject, kwargs: Option<Py<PyDict>>) -> Self {
        let path_parts = path
            .split('/')
            .map(|s| {
                if let Some(variable) = s.strip_prefix(':') {
                    PathPart::Variable(variable.to_string())
                } else {
                    PathPart::Static(s.to_string())
                }
        }).collect();

        Route {
            path_parts,
            python_cls: cls,
            kwargs
        }
    }

    pub fn create_instance(&self) -> PyRouteWrapper {
        let instance = Python::with_gil(|py|
            self.python_cls.call(py, (), self.kwargs.as_ref().map(|d| d.as_ref(py))));

        PyRouteWrapper::new(instance.unwrap())
    }
}

#[derive(Debug, Default, Clone)]
pub struct Routes {
    pub routes: Vec<Arc<Route>>,
}

impl Routes {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_route(&mut self, route: Route) {
        self.routes.push(Arc::new(route));
    }

    pub fn get_route(&self, path: &str) -> (Vec<String>, Option<Arc<Route>>) {
        let parts = path.split('/').collect::<Vec<&str>>();
        let mut var_parts = Vec::new();

        let route = self.routes.iter().cloned().find(|route| {
            var_parts.clear();
            let mut route_parts = route.path_parts.iter();
            let mut parts_iter = parts.iter();

            loop {
                match (route_parts.next(), parts_iter.next()) {
                    (Some(&PathPart::Static(ref route_part)), Some(&part)) => {
                        if route_part != part {
                            return false;
                        }
                    },

                    (Some(&PathPart::Variable(_)), Some(part)) => {
                        var_parts.push(part.to_string());
                        continue;
                    },

                    (None, None) => {
                        return true;
                    },

                    _ => {
                        return false;
                    },
                }
            }
        });

        (var_parts, route)
    }
}
