use crate::types::NodeConfig;

#[derive(Clone, Debug)]
pub struct ClnGrpcConfig {
    pub url: String,
    pub tls_certificate: String,
    pub tls_client_key: String,
    pub tls_client_certificate: String,
}

impl From<NodeConfig> for ClnGrpcConfig {
    fn from(config: NodeConfig) -> Self {
        ClnGrpcConfig {
            url: config.url.unwrap(),
            tls_certificate: config.tls_certificate.unwrap(),
            tls_client_key: config.tls_client_key.unwrap(),
            tls_client_certificate: config.tls_client_certificate.unwrap(),
        }
    }
}
