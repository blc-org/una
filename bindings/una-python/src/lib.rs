use pyo3::{exceptions::PyValueError, prelude::*};
use pythonize::{depythonize, pythonize};
use std::sync::Arc;
use tokio::sync::Mutex;

use una_core::{
    backends::cln::grpc::node::ClnGrpc,
    backends::eclair::rest::node::EclairRest,
    backends::lnd::rest::node::LndRest,
    node::{Node, NodeMethods},
    types::{Backend, CreateInvoiceParams, CreateInvoiceResult, NodeConfig, NodeInfo},
};

#[pyclass(name = "Node")]
struct PyNode(Arc<Mutex<Node>>);

#[pymethods]
impl PyNode {
    #[new]
    fn new(backend: String, config: PyObject) -> PyResult<Self> {
        let backend: Backend = backend.as_str().into();
        let config = Python::with_gil(|py| depythonize::<NodeConfig>(config.as_ref(py)).unwrap());

        match backend {
            Backend::LndRest => {
                let node = LndRest::new(config).unwrap();
                Ok(Self(Arc::new(Mutex::new(Node {
                    backend: Backend::LndRest,
                    node: Box::new(node),
                }))))
            }
            Backend::ClnGrpc => {
                let node = ClnGrpc::new(config).unwrap();
                Ok(Self(Arc::new(Mutex::new(Node {
                    backend: Backend::ClnGrpc,
                    node: Box::new(node),
                }))))
            }
            Backend::EclairRest => {
                let node = EclairRest::new(config.try_into().unwrap()).unwrap();
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

        let invoice =
            Python::with_gil(|py| depythonize::<CreateInvoiceParams>(invoice.as_ref(py)).unwrap());

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let result = node.lock().await.create_invoice(invoice).await.unwrap();
            let result =
                Python::with_gil(|py| pythonize::<CreateInvoiceResult>(py, &result).unwrap());
            Ok(result)
        })
    }

    pub fn get_info<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let node = self.0.clone();

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let result = node.lock().await.get_info().await.unwrap();
            let result = Python::with_gil(|py| pythonize::<NodeInfo>(py, &result).unwrap());
            Ok(result)
        })
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn una(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyNode>()?;
    Ok(())
}
