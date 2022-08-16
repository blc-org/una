use crate::error::Error;
use crate::node::NodeMethods;
use crate::types::{CreateInvoiceParams, CreateInvoiceResult, NodeInfo};

use super::config::EclairRestConfig;
use super::types::{
    ApiError, ChannelState, CreateInvoiceRequest, CreateInvoiceResponse, GetChannelsResponse,
    GetInfoResponse,
};

pub struct EclairRest {
    config: EclairRestConfig,
    client: reqwest::Client,
}

impl EclairRest {
    pub fn new(config: EclairRestConfig) -> Result<Self, Error> {
        let auth = format!("{}:{}", &config.username, &config.password);
        let authorization = format!("Basic {}", base64::encode(auth));

        let mut headers = reqwest::header::HeaderMap::new();
        let authorization_value = reqwest::header::HeaderValue::from_str(&authorization)?;
        headers.insert("Authorization", authorization_value);

        Ok(EclairRest {
            config,
            client: reqwest::Client::builder()
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

fn is_channel_in_state(n: &GetChannelsResponse, state: ChannelState) -> bool {
    match state {
        ChannelState::Normal => matches!((*n).state, ChannelState::Normal),
        ChannelState::Pending => matches!((*n).state, ChannelState::Pending),
        ChannelState::Offline => matches!((*n).state, ChannelState::Offline),
        _ => false,
    }
}

#[async_trait::async_trait]
impl NodeMethods for EclairRest {
    async fn create_invoice(
        &self,
        invoice: CreateInvoiceParams,
    ) -> Result<CreateInvoiceResult, Error> {
        let url = format!("{}/createinvoice", self.config.url);

        let request: CreateInvoiceRequest = invoice.into();
        let mut response = self.client.post(&url).form(&request).send().await?;

        response = Self::on_response(response).await?;

        let data: CreateInvoiceResponse = response.json().await?;

        Ok(data.into())
    }

    async fn get_info(&self) -> Result<NodeInfo, Error> {
        let url = format!("{}/getinfo", self.config.url);
        let mut response = self.client.post(&url).send().await?;
        response = Self::on_response(response).await?;
        let data: GetInfoResponse = response.json().await?;

        let url_channels = format!("{}/channels", self.config.url);
        let mut response_channels = self.client.post(&url_channels).send().await?;
        response_channels = Self::on_response(response_channels).await?;

        let data_channels: Vec<GetChannelsResponse> = response_channels.json().await?;

        let mut node_info: NodeInfo = data.into();

        node_info.channels.active = data_channels
            .iter()
            .filter(|n| is_channel_in_state(n, ChannelState::Normal))
            .count() as i64;
        node_info.channels.inactive = data_channels
            .iter()
            .filter(|n| is_channel_in_state(n, ChannelState::Offline))
            .count() as i64;
        node_info.channels.pending = data_channels
            .iter()
            .filter(|n| is_channel_in_state(n, ChannelState::Pending))
            .count() as i64;

        Ok(node_info)
    }
}
