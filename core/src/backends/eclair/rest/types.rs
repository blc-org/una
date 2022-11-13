#![allow(clippy::from_over_into)]

use std::convert::TryInto;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::Error;
use crate::{types::*, utils};

#[derive(Debug, Deserialize)]
pub struct ApiError {
    #[serde(rename = "error")]
    pub message: String,
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayInvoiceRequest {
    pub invoice: String,
    pub amount_msat: Option<u64>,
    pub max_attempts: Option<u64>,
    pub max_fee_flat_sat: Option<u64>,
    pub max_fee_pct: Option<u64>,
    pub external_id: Option<String>,
    pub path_finding_experiment_name: Option<String>,
    pub blocking: bool,
}

impl From<PayInvoiceParams> for PayInvoiceRequest {
    fn from(params: PayInvoiceParams) -> Self {
        let amount_msat = utils::get_amount_msat(params.amount, params.amount_msat);
        let max_fee_sat = utils::get_amount_msat(params.max_fee_sat, params.max_fee_msat);

        PayInvoiceRequest {
            invoice: params.payment_request,
            amount_msat,
            max_attempts: None,
            max_fee_flat_sat: max_fee_sat,
            max_fee_pct: params.max_fee_percent.map(|v| v as u64),
            external_id: None,
            path_finding_experiment_name: None,
            blocking: true,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayInvoiceResponse {
    #[serde(rename = "type")]
    pub type_field: String,
    pub id: String,
    pub payment_hash: String,
    pub payment_preimage: Option<String>,
    pub recipient_amount: Option<u64>,
    pub recipient_node_id: Option<String>,
    pub parts: Option<Vec<PaymentPart>>,
    pub failures: Option<Vec<PaymentFailure>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentPart {
    pub id: String,
    pub amount: i64,
    pub fees_paid: i64,
    pub to_channel_id: String,
    pub route: Vec<Route>,
    pub timestamp: Timestamp,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentUpdate {
    pub signature: String,
    pub chain_hash: String,
    pub short_channel_id: String,
    pub timestamp: Timestamp,
    pub channel_flags: ChannelFlags,
    pub cltv_expiry_delta: i64,
    pub htlc_minimum_msat: i64,
    pub fee_base_msat: i64,
    pub fee_proportional_millionths: i64,
    pub htlc_maximum_msat: i64,
    pub tlv_stream: TlvStream,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    pub node_id: String,
    pub next_node_id: String,
    pub last_update: PaymentUpdate,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TlvStream {
    pub records: Vec<Value>,
    pub unknown: Vec<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct ChannelFlags {
    pub is_enabled: bool,
    pub is_node1: bool,
}

#[derive(Debug, Deserialize)]

pub struct Timestamp {
    pub iso: String,
    pub unix: u64,
}

#[derive(Debug, Deserialize)]
pub struct PaymentFailure {
    pub amount: i64,
    pub route: Vec<Route>,
    #[serde(rename = "e")]
    pub route_error: Option<RouteError>,
    #[serde(rename = "t")]
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteError {
    pub origin_node: String,
    pub failure_message: String,
}

impl TryInto<PayInvoiceResult> for PayInvoiceResponse {
    type Error = Error;

    fn try_into(self) -> Result<PayInvoiceResult, Self::Error> {
        if self.type_field.as_str() == "payment-failed" {
            let failures = self.failures.ok_or_else(|| {
                Error::ApiError(String::from(
                    "error paying invoice, couldn't extract error message",
                ))
            })?;
            let failure = failures.first().ok_or_else(|| {
                Error::ApiError(String::from(
                    "error paying invoice, couldn't extract error message",
                ))
            })?;

            return match (&failure.route_error, &failure.error) {
                (Some(err), _) => Err(Error::ApiError(err.failure_message.to_string())),
                (_, Some(err)) => Err(Error::ApiError(err.to_string())),
                _ => Err(Error::ApiError(String::from(
                    "error paying invoice, couldn't extract error message",
                ))),
            };
        }

        let fees_msat = self.parts.map(|parts| {
            parts
                .iter()
                .fold(0, |acc, part| acc + part.fees_paid as u64)
        });

        let result = PayInvoiceResult {
            payment_hash: self.payment_hash,
            payment_preimage: self.payment_preimage.ok_or_else(|| {
                Error::ApiError(String::from("invoice paid but missing preimage"))
            })?,
            fees_msat,
        };

        Ok(result)
    }
}

#[derive(Debug, Serialize)]
pub struct DecodeInvoiceRequest {
    invoice: String,
}

impl From<String> for DecodeInvoiceRequest {
    fn from(invoice: String) -> DecodeInvoiceRequest {
        return DecodeInvoiceRequest { invoice };
    }
}

#[derive(Debug, Deserialize)]
pub struct Activated {
    pub var_onion_optin: Option<String>,
    pub payment_secret: Option<String>,
    pub basic_mpp: Option<String>,
    pub option_payment_metadata: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct Features {
    pub activated: Activated,
    pub unknown: Vec<Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecodeInvoiceRoutingInfo {
    pub node_id: String,
    pub short_channel_id: String,
    pub fee_base: u32,
    pub fee_proportional_millionths: u32,
    pub cltv_expiry_delta: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecodeInvoiceResponse {
    pub prefix: String,
    pub timestamp: i64,
    pub node_id: String,
    pub serialized: String,
    pub description: Option<String>,
    pub payment_hash: String,
    pub payment_metadata: Option<String>,
    pub expiry: i32,
    pub min_final_cltv_expiry: Option<u32>,
    pub amount: u64,
    pub features: Features,
    pub routing_info: Vec<Vec<DecodeInvoiceRoutingInfo>>,
}

fn extract_feature_status(feature_status_str: Option<String>) -> FeatureActivationStatus {
    match feature_status_str {
        None => FeatureActivationStatus::Unknown,
        Some(f) => match f.as_str() {
            "optional" => FeatureActivationStatus::Optional,
            "mandatory" => FeatureActivationStatus::Mandatory,
            _ => FeatureActivationStatus::Unknown,
        },
    }
}

impl TryInto<DecodeInvoiceResult> for DecodeInvoiceResponse {
    type Error = Error;

    fn try_into(self) -> Result<DecodeInvoiceResult, Error> {
        let invoices_feature = InvoiceFeatures {
            payment_secret: extract_feature_status(self.features.activated.payment_secret),
            basic_mpp: extract_feature_status(self.features.activated.basic_mpp),
            option_payment_metadata: extract_feature_status(
                self.features.activated.option_payment_metadata,
            ),
            var_onion_optin: extract_feature_status(self.features.activated.var_onion_optin),
        };

        let route_hints: Vec<RoutingHint> = self
            .routing_info
            .into_iter()
            .map(|route_info| {
                let info = RoutingHint {
                    hop_hints: route_info
                        .into_iter()
                        .map(|hop_hint| {
                            let hop = HopHint {
                                node_id: hop_hint.node_id.to_string(),
                                chan_id: hop_hint.short_channel_id,
                                fee_base_msat: hop_hint.fee_base,
                                fee_proportional_millionths: hop_hint.fee_proportional_millionths,
                                cltv_expiry_delta: hop_hint.cltv_expiry_delta as u32,
                            };

                            Ok::<HopHint, Error>(hop)
                        })
                        .try_collect()?,
                };

                Ok::<RoutingHint, Error>(info)
            })
            .try_collect()?;

        let invoice = DecodeInvoiceResult {
            creation_date: self.timestamp,
            amount: Some(utils::msat_to_sat(self.amount)),
            amount_msat: Some(self.amount),
            destination: Some(self.node_id),
            memo: self.description,
            payment_hash: self.payment_hash,
            expiry: self.expiry,
            min_final_cltv_expiry: 18,
            features: Some(invoices_feature),
            route_hints,
        };

        Ok(invoice)
    }
}
