use crate::types::NodeConfig;

#[derive(Clone, Debug)]
pub struct LndRestConfig {
    pub url: String,
    pub macaroon: String,
    pub tls_certificate: String,
}

impl From<NodeConfig> for LndRestConfig {
    fn from(config: NodeConfig) -> Self {
        LndRestConfig {
            url: config.url.unwrap(),
            macaroon: config.macaroon.unwrap(),
            tls_certificate: config.tls_certificate.unwrap(),
        }
    }
}
