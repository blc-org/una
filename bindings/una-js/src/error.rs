use std::result::Result;

use napi::{Error, Status};
use una_core::error::{ConfigError as UnaConfigError, Error as UnaError};

#[derive(Debug)]
pub struct NapiError(pub UnaError);

impl From<NapiError> for Error {
    fn from(err: NapiError) -> Self {
        let una_err = err.0;
        let reason = una_err.to_string();

        let (status, reason) = match una_err {
            UnaError::InvalidBackend => (Status::InvalidArg, reason),
            UnaError::ConfigError(config_err) => {
                let reason = config_err.to_string();

                match config_err {
                    UnaConfigError::InvalidField(_) => (Status::InvalidArg, reason),
                    UnaConfigError::MissingField(_) => (Status::InvalidArg, reason),
                    _ => (Status::GenericFailure, reason),
                }
            }
            _ => (Status::GenericFailure, reason),
        };

        Error::new(status, reason)
    }
}

impl From<UnaError> for NapiError {
    fn from(err: UnaError) -> Self {
        NapiError(err)
    }
}

pub trait OrNapiError<T> {
    fn or_napi_error(self) -> Result<T, NapiError>;
}

impl<T> OrNapiError<T> for Result<T, UnaError> {
    fn or_napi_error(self) -> Result<T, NapiError> {
        match self {
            Ok(t) => Ok(t),
            Err(err) => Err(NapiError(err)),
        }
    }
}
