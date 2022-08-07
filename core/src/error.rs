use std::{
    fmt::{self, Display},
    io::ErrorKind,
};

#[derive(Debug)]
pub enum Error {
    MissingBackend,
    MissingConfig,
    InvalidBackend,
    UnauthorizedMacaroon,
    ReqwestError(reqwest::Error),
    UnknownError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Error::InvalidBackend => String::from("invalid backend"),
            Error::MissingBackend => String::from("missing backend"),
            Error::MissingConfig => String::from("missing config"),
            Error::UnauthorizedMacaroon => String::from("unauthorized macaroon"),
            Error::ReqwestError(err) => err.to_string(),
            Error::UnknownError(err) => err.clone(),
        };

        write!(f, "{}", str)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::ReqwestError(err)
    }
}

impl From<Error> for std::io::Error {
    fn from(e: Error) -> std::io::Error {
        std::io::Error::new(ErrorKind::Other, e.to_string())
    }
}
