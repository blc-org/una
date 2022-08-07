use crate::backends::lnd::rest::node::LndRest;
use crate::error::Error;
use crate::types::{Backend, CreateInvoiceParams, NodeInfo};

pub struct NodeConfig {
    pub url: Option<String>,
    pub macaroon: Option<String>,
    pub certificate: Option<String>,
}

#[async_trait::async_trait]
pub trait Node {
    async fn create_invoice(&self, invoice: CreateInvoiceParams) -> Result<String, Error>;
    async fn get_info(&self) -> Result<NodeInfo, Error>;
}

pub fn new(backend: Backend, config: NodeConfig) -> Result<Box<dyn Node>, Error> {
    match backend {
        Backend::LndRest => {
            let node = LndRest::new(config).unwrap();
            Ok(Box::new(node))
        }
        _ => Err(Error::InvalidBackend),
    }
}
