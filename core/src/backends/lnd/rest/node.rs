use crate::error::Error;
use crate::node::NodeMethods;
use crate::types::{CreateInvoiceParams, CreateInvoiceResult, NodeInfo};

use super::config::LndRestConfig;
use super::types::{ApiError, CreateInvoiceRequest, CreateInvoiceResponse, GetInfoResponse};

pub struct LndRest {
    config: LndRestConfig,
    client: reqwest::Client,
}

impl LndRest {
    pub fn new(config: LndRestConfig) -> Result<Self, Error> {
        let tls_certificate = reqwest::Certificate::from_pem(&config.tls_certificate)?;

        let mut headers = reqwest::header::HeaderMap::new();
        let mut macaroon_value = reqwest::header::HeaderValue::from_bytes(&config.macaroon)?;
        macaroon_value.set_sensitive(true);
        headers.insert("Grpc-Metadata-macaroon", macaroon_value);

        Ok(LndRest {
            config,
            client: reqwest::Client::builder()
                .add_root_certificate(tls_certificate)
                .default_headers(headers)
                .build()?,
        })
    }

    pub async fn on_response(response: reqwest::Response) -> Result<reqwest::Response, Error> {
        let status = response.status();

        match status {
            reqwest::StatusCode::OK => Ok(response),
            reqwest::StatusCode::INTERNAL_SERVER_ERROR => {
                let error: ApiError = response.json().await?;

                println!("{:?}", error);

                match error.message.as_str() {
                    "permission denied" => Err(Error::UnauthorizedMacaroon),
                    _ => Err(Error::UnknownError(error.message)),
                }
            }
            _ => match response.error_for_status() {
                Ok(_res) => Ok(_res),
                Err(err) => Err(err.into()),
            },
        }
    }
}

#[async_trait::async_trait]
impl NodeMethods for LndRest {
    async fn create_invoice(
        &self,
        invoice: CreateInvoiceParams,
    ) -> Result<CreateInvoiceResult, Error> {
        let url = format!("{}/v1/invoices", self.config.url);

        let request: CreateInvoiceRequest = invoice.into();
        let mut response = self.client.post(&url).json(&request).send().await?;

        response = Self::on_response(response).await?;

        let data: CreateInvoiceResponse = response.json().await?;

        Ok(data.into())
    }

    async fn get_info(&self) -> Result<NodeInfo, Error> {
        let url = format!("{}/v1/getinfo", self.config.url);

        let mut response = self.client.get(&url).send().await?;

        response = Self::on_response(response).await?;

        let data: GetInfoResponse = response.json().await?;

        Ok(data.into())
    }
}
