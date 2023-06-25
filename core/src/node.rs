use crate::backends::cln::grpc::node::ClnGrpc;
use crate::backends::eclair::rest::node::EclairRest;
use crate::backends::lnd::rest::node::LndRest;
use crate::error::Error;
use crate::types::{
    Backend, CreateInvoiceParams, CreateInvoiceResult, DecodeInvoiceResult, NodeConfig, NodeInfo,
    PayInvoiceParams, PayInvoiceResult,
};

#[async_trait::async_trait]
pub trait NodeMethods {
    async fn create_invoice(
        &self,
        invoice: CreateInvoiceParams,
    ) -> Result<CreateInvoiceResult, Error>;
    async fn get_info(&self) -> Result<NodeInfo, Error>;
    async fn pay_invoice(&self, invoice: PayInvoiceParams) -> Result<PayInvoiceResult, Error>;
    async fn decode_invoice(&self, invoice: String) -> Result<DecodeInvoiceResult, Error>;
}

pub struct Node {
    pub backend: Backend,
    pub node: Box<dyn NodeMethods + Send + Sync>,
}

impl Node {
    pub fn new(backend: Backend, config: NodeConfig) -> Result<Self, Error> {
        match backend {
            Backend::LndRest => {
                let node = LndRest::new(config.try_into()?)?;
                Ok(Node {
                    backend,
                    node: Box::new(node),
                })
            }
            Backend::ClnGrpc => {
                let node = ClnGrpc::new(config.try_into()?)?;
                Ok(Node {
                    backend,
                    node: Box::new(node),
                })
            }
            Backend::EclairRest => {
                let node = EclairRest::new(config.try_into()?)?;
                Ok(Node {
                    backend,
                    node: Box::new(node),
                })
            }
            _ => Err(Error::InvalidBackend),
        }
    }
}

#[async_trait::async_trait]
impl NodeMethods for Node {
    async fn create_invoice(
        &self,
        invoice: CreateInvoiceParams,
    ) -> Result<CreateInvoiceResult, Error> {
        self.node.create_invoice(invoice).await
    }

    async fn get_info(&self) -> Result<NodeInfo, Error> {
        self.node.get_info().await
    }

    async fn pay_invoice(&self, invoice: PayInvoiceParams) -> Result<PayInvoiceResult, Error> {
        self.node.pay_invoice(invoice).await
    }

    async fn decode_invoice(&self, invoice: String) -> Result<DecodeInvoiceResult, Error> {
        self.node.decode_invoice(invoice).await
    }
}
