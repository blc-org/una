use std::collections::HashMap;

use crate::types::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub code: i32,
    pub message: String,
    pub details: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateInvoiceRequest {
    pub memo: Option<String>,
    pub r_preimage: Option<String>,
    pub value_msat: u64,
    pub description_hash: Option<String>,
    pub expiry: i32,
    pub fallback_addr: Option<String>,
    pub cltv_expiry: Option<i32>,
}

impl From<CreateInvoiceParams> for CreateInvoiceRequest {
    fn from(params: CreateInvoiceParams) -> Self {
        let value_msat = match (params.amount, params.amount_msats) {
            (Some(amount), _) => amount * 1000,
            (_, Some(amount_msats)) => amount_msats,
            (None, None) => 0,
        };

        CreateInvoiceRequest {
            memo: params.description,
            r_preimage: params.payment_preimage,
            value_msat,
            description_hash: params.description_hash,
            expiry: params.expire_in.unwrap_or(3600),
            fallback_addr: params.fallback_address,
            cltv_expiry: params.cltv_expiry,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateInvoiceResponse {
    pub r_hash: String,
    pub payment_request: String,
    pub add_index: String,
    pub payment_addr: String,
}

#[derive(Debug, Deserialize)]
pub struct GetInfoResponse {
    pub version: String,
    pub commit_hash: String,
    pub identity_pubkey: String,
    pub alias: String,
    pub color: String,
    pub num_pending_channels: i64,
    pub num_active_channels: i64,
    pub num_inactive_channels: i64,
    pub num_peers: i64,
    pub block_height: i64,
    pub block_hash: String,
    pub best_header_timestamp: String,
    pub synced_to_chain: bool,
    pub synced_to_graph: bool,
    pub testnet: bool,
    pub chains: Vec<Chain>,
    pub uris: Vec<String>,
    pub features: HashMap<String, Feature>,
    pub require_htlc_interceptor: Option<bool>,
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
        NodeInfo {
            backend: Backend::LndRest,
            version: self.version,
            testnet: self.testnet,
            node_pubkey: self.identity_pubkey,
            channels: ChannelStats {
                active: self.num_active_channels,
                inactive: self.num_inactive_channels,
                pending: self.num_pending_channels,
            },
        }
    }
}