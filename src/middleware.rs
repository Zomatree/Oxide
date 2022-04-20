use pyo3::prelude::*;
use pyo3_asyncio::into_future_with_locals;

use crate::request::PyRequest;
use crate::response::PyResponse;
use crate::LOCAL_TASKS;

#[derive(Debug, Clone)]
pub struct MiddlewareWrapper(pub PyObject);

impl MiddlewareWrapper {
    pub fn new(obj: PyObject) -> Self {
        MiddlewareWrapper(obj)
    }

    pub async fn call(&self, request: Py<PyRequest>) -> PyResult<Option<PyResponse>> {
        Python::with_gil::<_, PyResult<_>>(|py| {
            let coro = self.0
                .call1(py, (request,))?;

            into_future_with_locals(LOCAL_TASKS.get().unwrap(), coro.as_ref(py))
        })?
        .await
        .and_then(|any| Python::with_gil(|py| any.extract::<Option<PyResponse>>(py)))
    }
}
