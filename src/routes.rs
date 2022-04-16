use pyo3::prelude::*;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum PathPart {
    Static(String),
    Variable(String),
}

#[pyclass]
#[derive(Clone, Debug, Default)]
pub struct Route {
    pub path_parts: Vec<PathPart>,

    pub get: Option<Arc<PyObject>>,
    pub post: Option<Arc<PyObject>>,
    pub put: Option<Arc<PyObject>>,
    pub delete: Option<Arc<PyObject>>,
    pub patch: Option<Arc<PyObject>>,
    pub options: Option<Arc<PyObject>>,
    pub head: Option<Arc<PyObject>>,
}

impl Route {
    pub fn new(path: String) -> Self {
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
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Routes {
    pub routes: Vec<Route>,
}

impl Routes {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_route(&mut self, route: Route) {
        self.routes.push(route);
    }

    pub fn get_route<'a>(&'a self, path: &str) -> (Vec<String>, Option<&'a Route>) {
        let parts = path.split('/').collect::<Vec<&str>>();
        let mut var_parts = Vec::new();

        let route = self.routes.iter().find(|&route| {
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
