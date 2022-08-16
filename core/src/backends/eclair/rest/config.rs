use crate::types::NodeConfig;

#[derive(Clone, Debug)]
pub struct EclairRestConfig {
    pub url: String,
    pub username: String,
    pub password: String,
}

impl From<NodeConfig> for EclairRestConfig {
    fn from(config: NodeConfig) -> Self {
        EclairRestConfig {
            url: config.url.unwrap(),
            username: config.username.unwrap(),
            password: config.password.unwrap(),
        }
    }
}
