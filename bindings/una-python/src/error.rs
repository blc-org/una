use std::result::Result;

use pyo3::exceptions::{
    PyBaseException, PyConnectionError, PyException, PyKeyError, PyNotImplementedError,
    PyPermissionError, PyTimeoutError, PyValueError,
};
use pyo3::{create_exception, PyErr};
use pythonize::PythonizeError;
use una_core::error::{ConfigError as UnaConfigError, Error as UnaError};

create_exception!(una, PyApiError, PyException);
create_exception!(una, PyConfigError, PyException);

#[derive(Debug)]
pub struct PyError(pub UnaError);

impl From<PyError> for PyErr {
    fn from(err: PyError) -> Self {
        let una_err = err.0;
        let message = una_err.to_string();

        match una_err {
            UnaError::InvalidBackend => PyValueError::new_err(message),
            UnaError::ConfigError(config_err) => {
                let message = config_err.to_string();

                match config_err {
                    UnaConfigError::InvalidField(field) => PyValueError::new_err(field),
                    UnaConfigError::MissingField(field) => PyKeyError::new_err(field),
                    _ => PyConfigError::new_err(message),
                }
            }
            UnaError::Unauthorized => PyPermissionError::new_err(message),
            UnaError::NotImplemented(message) => PyNotImplementedError::new_err(message),
            UnaError::ConnectionError(message) => {
                if message.contains("timeout") {
                    PyTimeoutError::new_err(message)
                } else {
                    PyConnectionError::new_err(message)
                }
            }
            UnaError::ApiError(message) => PyApiError::new_err(message),
            _ => PyBaseException::new_err(message),
        }
    }
}

impl From<UnaError> for PyError {
    fn from(err: UnaError) -> Self {
        PyError(err)
    }
}

impl From<PythonizeError> for PyError {
    fn from(err: PythonizeError) -> Self {
        PyError(UnaError::ConversionError(err.to_string()))
    }
}

pub trait OrPyError<T> {
    fn or_py_error(self) -> Result<T, PyError>;
}

impl<T> OrPyError<T> for Result<T, UnaError> {
    fn or_py_error(self) -> Result<T, PyError> {
        match self {
            Ok(t) => Ok(t),
            Err(err) => Err(PyError(err)),
        }
    }
}

impl<T> OrPyError<T> for Result<T, PythonizeError> {
    fn or_py_error(self) -> Result<T, PyError> {
        match self {
            Ok(t) => Ok(t),
            Err(err) => Err(err.into()),
        }
    }
}
