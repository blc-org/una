use http::uri::Uri;
use std::convert::TryFrom;

use crate::error::{ConfigError, Error};
use crate::types::NodeConfig;

#[derive(Clone, Debug)]
pub struct ClnGrpcConfig {
    pub url: String,
    pub tls_certificate: Vec<u8>,
    pub tls_client_key: Vec<u8>,
    pub tls_client_certificate: Vec<u8>,
}

impl TryFrom<NodeConfig> for ClnGrpcConfig {
    type Error = Error;

    fn try_from(config: NodeConfig) -> Result<Self, Self::Error> {
        let url = config
            .url
            .ok_or_else(|| ConfigError::MissingField("url".to_string()))?;
        let tls_certificate = config
            .tls_certificate
            .ok_or_else(|| ConfigError::MissingField("tls_certificate".to_string()))?;
        let tls_client_key = config
            .tls_client_key
            .ok_or_else(|| ConfigError::MissingField("tls_client_key".to_string()))?;
        let tls_client_certificate = config
            .tls_client_certificate
            .ok_or_else(|| ConfigError::MissingField("tls_client_certificate".to_string()))?;

        // Verify URL
        Uri::from_maybe_shared(url.clone())
            .map_err(|_| ConfigError::InvalidField("url".to_string()))?;

        let config = ClnGrpcConfig {
            url,
            tls_certificate: hex::decode(&tls_certificate)
                .map_err(|_| ConfigError::ParsingHexError("tls_certificate".to_string()))?,
            tls_client_key: hex::decode(&tls_client_key)
                .map_err(|_| ConfigError::ParsingHexError("tls_client_key".to_string()))?,
            tls_client_certificate: hex::decode(&tls_client_certificate)
                .map_err(|_| ConfigError::ParsingHexError("tls_client_certificate".to_string()))?,
        };

        Ok(config)
    }
}
