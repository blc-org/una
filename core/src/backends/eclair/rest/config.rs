use crate::error::{ConfigError, Error};
use crate::types::NodeConfig;

#[derive(Clone, Debug)]
pub struct EclairRestConfig {
    pub url: String,
    pub username: String,
    pub password: String,
}

impl TryFrom<NodeConfig> for EclairRestConfig {
    type Error = Error;

    fn try_from(config: NodeConfig) -> Result<Self, Self::Error> {
        let url = config
            .url
            .ok_or_else(|| ConfigError::MissingField("url".to_string()))?;
        let username = config
            .username
            .ok_or_else(|| ConfigError::MissingField("username".to_string()))?;
        let password = config
            .password
            .ok_or_else(|| ConfigError::MissingField("password".to_string()))?;

        let config = EclairRestConfig {
            url,
            username,
            password,
        };

        Ok(config)
    }
}
