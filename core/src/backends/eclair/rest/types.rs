use crate::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub code: i32,
    pub message: String,
    pub details: Vec<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInvoiceRequest {
    pub description: Option<String>,
    pub description_hash: Option<String>,
    pub payment_preimage: Option<String>,
    pub amount_msat: u64,
    pub expire_in: u32,
    pub fallback_address: Option<String>,
}

impl From<CreateInvoiceParams> for CreateInvoiceRequest {
    fn from(params: CreateInvoiceParams) -> Self {
        let amount_msat = match (params.amount, params.amount_msat) {
            (Some(amount), _) => amount * 1000,
            (_, Some(amount_msats)) => amount_msats,
            (None, None) => 0,
        };

        CreateInvoiceRequest {
            description: params.description,
            payment_preimage: params.payment_preimage,
            amount_msat,
            description_hash: params.description_hash,
            expire_in: params.expire_in.unwrap_or(3600),
            fallback_address: params.fallback_address,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInvoiceResponse {
    pub serialized: String,
    pub payment_hash: String,
    pub description: Option<String>,
}

impl Into<CreateInvoiceResult> for CreateInvoiceResponse {
    fn into(self) -> CreateInvoiceResult {
        CreateInvoiceResult {
            payment_request: self.serialized,
            payment_hash: self.payment_hash,
            label: self.description,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInfoResponse {
    pub version: String,
    pub node_id: String,
    pub alias: String,
    pub color: String,
    pub network: String,
}

#[derive(Debug, Deserialize)]
pub struct GetChannelsResponse {
    pub state: ChannelState,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ChannelState {
    Normal,
    Offline,
    Closed,
    Pending,
}

#[derive(Debug, Deserialize)]
pub struct Chain {
    pub chain: String,
    pub network: String,
}

#[derive(Debug, Deserialize)]
pub struct Feature {
    pub name: String,
    pub is_required: bool,
    pub is_known: bool,
}

impl Into<NodeInfo> for GetInfoResponse {
    fn into(self) -> NodeInfo {
        let network = match self.network.as_ref() {
            "mainnet" => Network::Mainnet,
            "testnet" => Network::Testnet,
            "regtest" => Network::Regtest,
            _ => Network::Unknown(self.network.clone()),
        };

        NodeInfo {
            backend: Backend::EclairRest,
            version: self.version,
            network,
            node_pubkey: self.node_id,
            channels: ChannelStats {
                active: 0,
                inactive: 0,
                pending: 0,
            },
        }
    }
}
