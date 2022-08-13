use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct NodeConfig {
    pub url: Option<String>,
    pub macaroon: Option<String>,
    pub certificate: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub enum Backend {
    LndRest,
    LndGrpc,
    ClnRest,
    InvalidBackend,
}

impl Display for Backend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Backend::LndRest => String::from("LndRest"),
            Backend::LndGrpc => String::from("LndGrpc"),
            Backend::ClnRest => String::from("ClnRest"),
            Backend::InvalidBackend => String::from("InvalidBackend"),
        };

        write!(f, "{}", str)
    }
}

impl From<&str> for Backend {
    fn from(s: &str) -> Self {
        match s {
            "LndRest" => Backend::LndRest,
            "LndGrpc" => Backend::LndGrpc,
            "ClnRest" => Backend::ClnRest,
            // etc.
            _ => Backend::InvalidBackend,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CreateInvoiceParams {
    pub amount: Option<u64>,
    pub amount_msat: Option<u64>,
    pub description: Option<String>,
    pub description_hash: Option<String>,
    pub label: Option<String>,
    pub expire_in: Option<i32>,
    pub fallback_address: Option<String>,
    pub payment_preimage: Option<String>,
    pub cltv_expiry: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Invoice {
    pub bolt11: String,
    pub memo: String,
    pub amount: u64,
    pub amount_msat: u64,
    pub pre_image: Option<String>,
    pub payment_hash: String,
    pub settled: bool,
    pub settle_date: Option<i64>,
    pub creation_date: i64,
    pub expiry: i32,
    pub status: InvoiceStatus,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub enum InvoiceStatus {
    Pending,
    Settled,
    Cancelled,
    Accepted,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ChannelStats {
    pub active: i64,
    pub inactive: i64,
    pub pending: i64,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct NodeInfo {
    pub backend: Backend,
    pub version: String,
    pub testnet: bool,
    pub node_pubkey: String,
    pub channels: ChannelStats,
}
