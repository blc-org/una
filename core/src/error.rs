use std::{
    fmt::{self, Display},
    io::ErrorKind,
};

#[derive(Debug)]
pub enum ConfigError {
    MissingField(String),
    InvalidField(String),
    ParsingHexError(String),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::MissingField(field) => write!(f, "Missing field: {}", field),
            ConfigError::InvalidField(field) => write!(f, "Invalid field: {}", field),
            ConfigError::ParsingHexError(field) => {
                write!(f, "Error parsing field {}: expected hex string", field)
            }
        }
    }
}

#[derive(Debug)]
pub enum Error {
    MissingBackend,
    InvalidBackend,
    Unauthorized,
    NotImplemented,
    ConfigError(ConfigError),
    ConnectionError(String),
    ApiError(String),
    UnknownError(String),
    ConversionError(String),
    ParseNumberError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Error::InvalidBackend => String::from("invalid backend"),
            Error::MissingBackend => String::from("missing backend"),
            Error::Unauthorized => String::from("unauthorized credentials"),
            Error::NotImplemented => String::from("not implemented"),
            Error::ConfigError(err) => err.to_string(),
            Error::ConnectionError(err) => err.to_string(),
            Error::ApiError(err) => err.clone(),
            Error::ConversionError(err) => err.clone(),
            Error::UnknownError(err) => err.clone(),
            Error::ParseNumberError(err) => err.clone(),
        };

        write!(f, "{}", str)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            let message = match err.url() {
                Some(url) => format!("timeout: {}", url),
                None => String::from("timeout"),
            };
            Error::ConnectionError(message)
        } else if err.is_connect() {
            let message = match err.url() {
                Some(url) => format!("couldn't connect to {}", url),
                None => err.to_string(),
            };
            Error::ConnectionError(message)
        } else if err.is_builder() {
            Error::ConnectionError(err.to_string().replace("builder error: ", ""))
        } else {
            Error::ApiError(err.to_string())
        }
    }
}

impl From<tonic::transport::Error> for Error {
    fn from(err: tonic::transport::Error) -> Self {
        Error::ConnectionError(err.to_string())
    }
}

impl From<reqwest::header::InvalidHeaderValue> for Error {
    fn from(err: reqwest::header::InvalidHeaderValue) -> Self {
        Error::ApiError(err.to_string())
    }
}

impl From<tonic::Status> for Error {
    fn from(status: tonic::Status) -> Self {
        let status_message = status.message().to_string();

        let rpc_error_regex =
            regex::Regex::new(r"RpcError").expect("Hardcoded regex should be valid.");
        if rpc_error_regex.is_match(&status_message) {
            let rpc_error_message_regex = regex::Regex::new(r#"message: "(?P<msg>.*)""#)
                .expect("Hardcoded regex should be valid.");
            let rpc_error_message = rpc_error_message_regex
                .captures(&status_message)
                .and_then(|cap| cap.name("msg").map(|msg| msg.as_str()));

            if let Some(message) = rpc_error_message {
                return Error::ApiError(message.to_string());
            }
        }

        Error::ApiError(status_message)
    }
}

impl From<ConfigError> for Error {
    fn from(err: ConfigError) -> Self {
        Error::ConfigError(err)
    }
}

impl From<base64::DecodeError> for Error {
    fn from(_: base64::DecodeError) -> Self {
        Error::ConversionError(String::from("couldn't convert base64 to hex"))
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(_: std::num::ParseIntError) -> Self {
        Error::ConversionError(String::from("couldn't convert string to integer"))
    }
}

impl From<Error> for std::io::Error {
    fn from(e: Error) -> std::io::Error {
        std::io::Error::new(ErrorKind::Other, e.to_string())
    }
}
