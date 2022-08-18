use pyo3::{exceptions::PyValueError, prelude::*};
use pythonize::{depythonize, pythonize};
use std::sync::Arc;
use tokio::sync::Mutex;

use una_core::{
    backends::{
        cln::grpc::{config::ClnGrpcConfig, node::ClnGrpc},
        eclair::rest::{config::EclairRestConfig, node::EclairRest},
        lnd::rest::{config::LndRestConfig, node::LndRest},
    },
    node::{Node, NodeMethods},
    types::{
        Backend, CreateInvoiceParams, CreateInvoiceResult, NodeConfig, NodeInfo, PayInvoiceParams,
        PayInvoiceResult,
    },
};

pub mod error;

use error::{OrPyError, PyApiError, PyConfigError};

#[pyclass(name = "Node")]
struct PyNode(Arc<Mutex<Node>>);

#[pymethods]
impl PyNode {
    #[new]
    fn new(backend: String, config: PyObject) -> PyResult<Self> {
        let backend: Backend = backend.as_str().into();
        let config =
            Python::with_gil(|py| depythonize::<NodeConfig>(config.as_ref(py)).or_py_error())?;

        match backend {
            Backend::LndRest => {
                let config = TryInto::<LndRestConfig>::try_into(config).or_py_error()?;
                let node = LndRest::new(config).or_py_error()?;
                Ok(Self(Arc::new(Mutex::new(Node {
                    backend: Backend::LndRest,
                    node: Box::new(node),
                }))))
            }
            Backend::ClnGrpc => {
                let config = TryInto::<ClnGrpcConfig>::try_into(config).or_py_error()?;
                let node = ClnGrpc::new(config).or_py_error()?;
                Ok(Self(Arc::new(Mutex::new(Node {
                    backend: Backend::ClnGrpc,
                    node: Box::new(node),
                }))))
            }
            Backend::EclairRest => {
                let config = TryInto::<EclairRestConfig>::try_into(config).or_py_error()?;
                let node = EclairRest::new(config).or_py_error()?;
                Ok(Self(Arc::new(Mutex::new(Node {
                    backend: Backend::EclairRest,
                    node: Box::new(node),
                }))))
            }
            Backend::LndGrpc => todo!(),
            Backend::InvalidBackend => Err(PyValueError::new_err("Invalid backend")),
        }
    }

    pub fn create_invoice<'p>(&self, py: Python<'p>, invoice: PyObject) -> PyResult<&'p PyAny> {
        let node = self.0.clone();

        let invoice = Python::with_gil(|py| {
            depythonize::<CreateInvoiceParams>(invoice.as_ref(py)).or_py_error()
        })?;

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let result = node
                .lock()
                .await
                .create_invoice(invoice)
                .await
                .or_py_error()?;
            let result =
                Python::with_gil(|py| pythonize::<CreateInvoiceResult>(py, &result).or_py_error())?;
            Ok(result)
        })
    }

    pub fn get_info<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let node = self.0.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let result = node.lock().await.get_info().await.or_py_error()?;
            let result = Python::with_gil(|py| pythonize::<NodeInfo>(py, &result).or_py_error())?;
            Ok(result)
        })
    }

    pub fn pay_invoice<'p>(&self, py: Python<'p>, invoice: PyObject) -> PyResult<&'p PyAny> {
        let node = self.0.clone();

        let invoice = Python::with_gil(|py| {
            depythonize::<PayInvoiceParams>(invoice.as_ref(py)).or_py_error()
        })?;

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let result = node.lock().await.pay_invoice(invoice).await.or_py_error()?;
            let result =
                Python::with_gil(|py| pythonize::<PayInvoiceResult>(py, &result).or_py_error())?;
            Ok(result)
        })
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn una(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyNode>()?;
    m.add("ApiError", py.get_type::<PyApiError>())?;
    m.add("ConfigError", py.get_type::<PyConfigError>())?;
    Ok(())
}
