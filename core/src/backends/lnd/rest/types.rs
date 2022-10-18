#![allow(clippy::from_over_into)]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::convert::TryInto;

use crate::backends::eclair::rest::types;
use crate::error::Error;
use crate::{types::*, utils};

pub type Base64String = String;

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
        let value_msat = match (params.amount, params.amount_msat) {
            (Some(amount), _) => amount * 1000,
            (_, Some(amount_msat)) => amount_msat,
            (None, None) => 0,
        };

        CreateInvoiceRequest {
            memo: params.description,
            r_preimage: params.payment_preimage,
            value_msat,
            description_hash: params.description_hash,
            expiry: params.expire_in.unwrap_or(3600).try_into().unwrap(),
            fallback_addr: params.fallback_address,
            cltv_expiry: params.cltv_expiry.map(|cltv_expiry| cltv_expiry as i32),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateInvoiceResponse {
    pub r_hash: Base64String,
    pub payment_request: String,
    pub add_index: String,
    pub payment_addr: Base64String,
}

impl TryInto<CreateInvoiceResult> for CreateInvoiceResponse {
    type Error = Error;

    fn try_into(self) -> Result<CreateInvoiceResult, Self::Error> {
        let payment_hash = utils::b64_to_hex(&self.r_hash)?;

        let result = CreateInvoiceResult {
            payment_request: self.payment_request,
            payment_hash,
            label: None,
        };

        Ok(result)
    }
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
        let network = match self.chains.get(0) {
            Some(chain) => match chain.network.as_ref() {
                "mainnet" => Network::Mainnet,
                "testnet" => Network::Testnet,
                "regtest" => Network::Regtest,
                _ => Network::Unknown(chain.network.clone()),
            },
            None => Network::Unknown("Unknown".to_string()),
        };

        NodeInfo {
            backend: Backend::LndRest,
            version: self.version,
            network,
            node_pubkey: self.identity_pubkey,
            channels: ChannelStats {
                active: self.num_active_channels,
                inactive: self.num_inactive_channels,
                pending: self.num_pending_channels,
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FeeLimit {
    pub fixed: Option<String>,
    pub fixed_msat: Option<String>,
    pub percent: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SendPaymentSyncRequest {
    pub dest: Option<Base64String>,
    pub amt: Option<String>,
    pub amt_msat: Option<String>,
    pub payment_hash: Option<Base64String>,
    pub payment_request: String,
    pub final_cltv_delta: Option<i32>,
    pub fee_limit: Option<FeeLimit>,
    pub outgoing_chan_id: Option<String>,
    pub last_hop_pubkey: Option<Base64String>,
    pub cltv_limit: Option<i64>,
    pub allow_self_payment: Option<bool>,
    pub dest_features: Option<Vec<u8>>,
    pub payment_addr: Option<Base64String>,
}

impl From<PayInvoiceParams> for SendPaymentSyncRequest {
    fn from(params: PayInvoiceParams) -> Self {
        let amount_msat = utils::get_amount_msat(params.amount, params.amount_msat);
        let max_fee_msat = utils::get_amount_msat(params.max_fee_sat, params.max_fee_msat);

        SendPaymentSyncRequest {
            dest: None,
            amt: None,
            amt_msat: amount_msat.map(|v| v.to_string()),
            payment_hash: None,
            payment_request: params.payment_request,
            final_cltv_delta: None,
            fee_limit: Some(FeeLimit {
                fixed: None,
                fixed_msat: max_fee_msat.map(|v| v.to_string()),
                percent: params.max_fee_percent.map(|v| v.to_string()),
            }),
            outgoing_chan_id: None,
            last_hop_pubkey: None,
            cltv_limit: None,
            allow_self_payment: Some(false),
            dest_features: None,
            payment_addr: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Hop {
    pub chan_id: String,
    pub chan_capacity: String,
    pub amt_to_forward: String,
    pub fee: String,
    pub expiry: i64,
    pub amt_to_forward_msat: String,
    pub fee_msat: String,
    pub pub_key: Option<String>,
    pub tlv_payload: bool,
    pub mpp_record: Option<MppRecord>,
    pub amp_record: Option<AmpRecord>,
    pub custom_records: HashMap<String, String>,
    pub metadata: Base64String,
}

#[derive(Debug, Deserialize)]
pub struct MppRecord {
    pub payment_addr: Base64String,
    pub total_amt_msat: String,
}

#[derive(Debug, Deserialize)]
pub struct AmpRecord {
    pub root_share: Base64String,
    pub set_id: Base64String,
    pub child_index: i64,
}

#[derive(Debug, Deserialize)]
pub struct Route {
    pub total_time_lock: i64,
    pub total_amt: String,
    pub total_amt_msat: String,
    pub total_fees: String,
    pub total_fees_msat: String,
    pub hops: Vec<Hop>,
}

#[derive(Debug, Deserialize)]
pub struct SendPaymentSyncResponse {
    pub payment_error: String,
    pub payment_preimage: Base64String,
    pub payment_route: Option<Route>,
    pub payment_hash: Base64String,
}

impl TryInto<PayInvoiceResult> for SendPaymentSyncResponse {
    type Error = Error;

    fn try_into(self) -> Result<PayInvoiceResult, Self::Error> {
        if !self.payment_error.is_empty() {
            return Err(Error::ApiError(self.payment_error));
        }

        let result = PayInvoiceResult {
            payment_hash: utils::b64_to_hex(&self.payment_hash)?,
            payment_preimage: utils::b64_to_hex(&self.payment_preimage)?,
            fees_msat: Some(
                self.payment_route
                    .ok_or_else(|| Error::ApiError(String::from("invoice paid but missing route")))?
                    .total_fees_msat
                    .parse()?,
            ),
        };

        Ok(result)
    }
}

#[derive(Debug, Deserialize)]
pub struct DecodeInvoiceResponse {
    pub destination: String,
    pub payment_hash: String,
    pub num_satoshis: String,
    pub timestamp: String,
    pub expiry: String,
    pub description: String,
    pub description_hash: String,
    pub fallback_addr: String,
    pub cltv_expiry: String,
    pub payment_addr: String,
    pub num_msat: String,
    pub features: HashMap<u16, DecodeInvoiceFeature>,
    pub route_hints: Vec<DecodeInvoiceRoutingHint>,
}

#[derive(Debug, Deserialize)]
pub struct DecodeInvoiceFeature {
    pub name: String,
    pub is_required: bool,
    pub is_known: bool,
}
#[derive(Debug, Deserialize)]
pub struct DecodeInvoiceRoutingHint {
    pub hop_hints: Vec<HopHint>,
}

#[derive(Debug, Deserialize)]
pub struct DecodeInvoiceHopHint {
    pub node_id: String,
    pub chan_id: u64,
    pub fee_base_msat: u32,
    pub fee_proportional_millionths: u32,
    pub cltv_expiry_delta: u32,
}

fn extract_feature_status(feature: Option<&DecodeInvoiceFeature>) -> FeatureActivationStatus {
    match feature {
        None => FeatureActivationStatus::Unknown,
        Some(f) => match (f.is_known, f.is_required) {
            (true, true) => FeatureActivationStatus::Mandatory,
            (true, false) => FeatureActivationStatus::Optional,
            _ => FeatureActivationStatus::Unknown,
        },
    }
}

impl TryInto<DecodeInvoiceResult> for DecodeInvoiceResponse {
    type Error = Error;

    fn try_into(self) -> Result<DecodeInvoiceResult, Self::Error> {
        let invoice_features = InvoiceFeatures {
            payment_secret: extract_feature_status(self.features.get(&14)),
            basic_mpp: extract_feature_status(self.features.get(&17)),
            option_payment_metadata: FeatureActivationStatus::Unknown,
            var_onion_optin: extract_feature_status(self.features.get(&9)),
        };

        let result = DecodeInvoiceResult {
            creation_date: self.timestamp.parse()?,
            amount: Some(self.num_satoshis.parse()?),
            amount_msat: Some(self.num_msat.parse()?),
            destination: Some(self.destination),
            memo: self.description,
            payment_hash: self.payment_hash,
            expiry: self.expiry.parse()?,
            min_final_cltv_expiry: self.cltv_expiry.parse()?,
            features: Some(invoice_features),
            route_hints: self
                .route_hints
                .into_iter()
                .map(|route_hint| RoutingHint {
                    hop_hints: route_hint
                        .hop_hints
                        .into_iter()
                        .map(|hop_hint| HopHint {
                            node_id: hop_hint.node_id,
                            chan_id: hop_hint.chan_id,
                            fee_base_msat: hop_hint.fee_base_msat,
                            fee_proportional_millionths: hop_hint.fee_proportional_millionths,
                            cltv_expiry_delta: hop_hint.cltv_expiry_delta,
                        })
                        .collect(),
                })
                .collect(),
        };

        Ok(result)
    }
}
