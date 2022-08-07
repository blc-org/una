use crate::node::NodeConfig;

#[derive(Clone, Debug)]
pub struct LndRestConfig {
    pub url: String,
    pub macaroon: String,
    pub certificate: String,
}

impl From<NodeConfig> for LndRestConfig {
    fn from(config: NodeConfig) -> Self {
        LndRestConfig {
            url: config.url.unwrap(),
            macaroon: config.macaroon.unwrap(),
            certificate: config.certificate.unwrap(),
        }
    }
}
