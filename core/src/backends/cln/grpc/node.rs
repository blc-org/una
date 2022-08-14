use tonic::transport::{Certificate, Channel, ClientTlsConfig, Endpoint, Identity};

use crate::error::Error;
use crate::node::NodeMethods;
use crate::types::{CreateInvoiceParams, CreateInvoiceResult, NodeConfig, NodeInfo};

use super::config::ClnGrpcConfig;
use super::pb::{node_client::NodeClient, GetinfoRequest, InvoiceRequest};

pub struct ClnGrpc {
    endpoint: Endpoint,
}

impl ClnGrpc {
    pub fn new(config: NodeConfig) -> Result<Self, Error> {
        let config: ClnGrpcConfig = config.into();

        let client_identity = Identity::from_pem(
            &hex::decode(config.tls_client_certificate).unwrap(),
            &hex::decode(config.tls_client_key).unwrap(),
        );

        let tls_certificate = Certificate::from_pem(&hex::decode(config.tls_certificate).unwrap());

        let tls = ClientTlsConfig::new()
            .domain_name("cln")
            .ca_certificate(tls_certificate)
            .identity(client_identity);

        let endpoint = Channel::from_shared(config.url)
            .unwrap()
            .tls_config(tls)
            .unwrap();

        Ok(ClnGrpc { endpoint })
    }

    // This is a temporary workaround, as we should spawn a unique channel for all
    // requests, but I'm not yet experienced enough with Rust to find a proper way
    // to do it.
    async fn get_client(&self) -> NodeClient<Channel> {
        let channel = self.endpoint.connect().await.unwrap();
        NodeClient::new(channel)
    }
}

#[async_trait::async_trait]
impl NodeMethods for ClnGrpc {
    async fn get_info(&self) -> Result<NodeInfo, Error> {
        let mut client = self.get_client().await;

        let request = GetinfoRequest {};
        let response = client.getinfo(request).await.unwrap().into_inner();

        Ok(response.into())
    }

    async fn create_invoice(
        &self,
        invoice: CreateInvoiceParams,
    ) -> Result<CreateInvoiceResult, Error> {
        let mut client = self.get_client().await;

        let request: InvoiceRequest = invoice.into();
        let label = request.label.clone();
        let response = client.invoice(request).await.unwrap().into_inner();

        let mut result: CreateInvoiceResult = response.into();
        result.label = Some(label);

        Ok(result)
    }
}
