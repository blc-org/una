use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct NodeConfig {
    pub url: Option<String>,
    pub macaroon: Option<String>,
    pub tls_certificate: Option<String>,
    pub tls_client_key: Option<String>,
    pub tls_client_certificate: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub enum Backend {
    LndRest,
    LndGrpc,
    ClnGrpc,
    EclairRest,
    InvalidBackend,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum Network {
    Mainnet,
    Testnet,
    Regtest,
    Unknown(String),
}

impl Display for Backend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Backend::LndRest => String::from("LndRest"),
            Backend::LndGrpc => String::from("LndGrpc"),
            Backend::ClnGrpc => String::from("ClnGrpc"),
            Backend::EclairRest => String::from("EclairRest"),
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
            "ClnGrpc" => Backend::ClnGrpc,
            "EclairRest" => Backend::EclairRest,
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
    pub expire_in: Option<u32>,
    pub fallback_address: Option<String>,
    pub payment_preimage: Option<String>,
    pub cltv_expiry: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CreateInvoiceResult {
    pub payment_request: String,
    pub payment_hash: String,
    pub label: Option<String>,
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
    pub settle_date: Option<u64>,
    pub creation_date: u64,
    pub expiry: u64,
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
    pub network: Network,
    pub node_pubkey: String,
    pub channels: ChannelStats,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct PayInvoiceParams {
    pub payment_request: String,
    pub amount: Option<u64>,
    pub amount_msat: Option<u64>,
    pub max_fee_sat: Option<u64>,
    pub max_fee_msat: Option<u64>,
    pub max_fee_percent: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct PayInvoiceResult {
    pub payment_hash: String,
    pub payment_preimage: String,
    pub fees_msat: Option<u64>,
}
