use std::convert::TryFrom;

use crate::error::{ConfigError, Error};
use crate::types::NodeConfig;

#[derive(Clone, Debug)]
pub struct LndRestConfig {
    pub url: String,
    pub macaroon: String,
    pub tls_certificate: Vec<u8>,
}

impl TryFrom<NodeConfig> for LndRestConfig {
    type Error = Error;

    fn try_from(config: NodeConfig) -> Result<Self, Self::Error> {
        let url = config
            .url
            .ok_or_else(|| ConfigError::MissingField("url".to_string()))?;
        let macaroon = config
            .macaroon
            .ok_or_else(|| ConfigError::MissingField("macaroon".to_string()))?;
        let tls_certificate = config
            .tls_certificate
            .ok_or_else(|| ConfigError::MissingField("tls_certificate".to_string()))?;

        let config = LndRestConfig {
            url,
            macaroon,
            tls_certificate: hex::decode(&tls_certificate)
                .map_err(|_| ConfigError::ParsingHexError("tls_certificate".to_string()))?,
        };

        Ok(config)
    }
}
