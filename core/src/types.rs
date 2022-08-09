use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Deserialize, Serialize)]
pub enum Backend {
    LndRest,
    LndGrpc,
    ClnRest,
    // etc.
}

impl Display for Backend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Backend::LndRest => String::from("lnd-rest"),
            Backend::LndGrpc => String::from("lnd-grpc"),
            Backend::ClnRest => String::from("cln-rest"),
            // etc.
        };

        write!(f, "{}", str)
    }
}

impl From<&str> for Backend {
    fn from(s: &str) -> Self {
        match s {
            "lnd-rest" => Backend::LndRest,
            "lnd-grpc" => Backend::LndGrpc,
            "cln-rest" => Backend::ClnRest,
            // etc.
            _ => panic!("Invalid backend"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateInvoiceParams {
    pub amount: Option<u64>,
    pub amount_msats: Option<u64>,
    pub description: Option<String>,
    pub description_hash: Option<String>,
    pub label: Option<String>,
    pub expire_in: Option<i32>,
    pub fallback_address: Option<String>,
    pub payment_preimage: Option<String>,
    pub cltv_expiry: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub enum InvoiceStatus {
    Pending,
    Settled,
    Cancelled,
    Accepted,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChannelStats {
    pub active: i64,
    pub inactive: i64,
    pub pending: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeInfo {
    pub backend: Backend,
    pub version: String,
    pub testnet: bool,
    pub node_pubkey: String,
    pub channels: ChannelStats,
}
