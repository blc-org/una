#![allow(clippy::from_over_into)]

use crate::types::*;
use cuid;

tonic::include_proto!("cln");

impl From<CreateInvoiceParams> for InvoiceRequest {
    fn from(params: CreateInvoiceParams) -> Self {
        let value = match (params.amount, params.amount_msat) {
            (Some(amount), _) => amount_or_any::Value::Amount(Amount {
                msat: amount * 1000,
            }),
            (_, Some(amount_msat)) => amount_or_any::Value::Amount(Amount { msat: amount_msat }),
            (None, None) => amount_or_any::Value::Any(true),
        };

        InvoiceRequest {
            msatoshi: Some(AmountOrAny { value: Some(value) }),
            description: params.description.unwrap_or_else(|| "".to_string()),
            label: params.label.unwrap_or_else(|| cuid::cuid().unwrap()),
            expiry: params.expire_in.map(|expire_in| expire_in as u64),
            fallbacks: params
                .fallback_address
                .map(|fallback_address| vec![fallback_address])
                .unwrap_or_default(),
            preimage: params
                .payment_preimage
                .map(|preimage| preimage.into_bytes()),
            exposeprivatechannels: None,
            cltv: params.cltv_expiry.map(|cltv_expiry| cltv_expiry as u32),
            deschashonly: Some(false),
        }
    }
}

impl Into<CreateInvoiceResult> for InvoiceResponse {
    fn into(self) -> CreateInvoiceResult {
        CreateInvoiceResult {
            payment_request: self.bolt11,
            payment_hash: hex::encode(self.payment_hash),
            label: None,
        }
    }
}

impl Into<NodeInfo> for GetinfoResponse {
    fn into(self) -> NodeInfo {
        let network = match self.network.as_ref() {
            "bitcoin" => Network::Mainnet,
            "testnet" => Network::Testnet,
            "regtest" => Network::Regtest,
            _ => Network::Mainnet,
        };

        NodeInfo {
            backend: Backend::ClnGrpc,
            version: self.version,
            network,
            node_pubkey: hex::encode(self.id),
            channels: ChannelStats {
                active: self.num_active_channels as i64,
                inactive: self.num_inactive_channels as i64,
                pending: self.num_pending_channels as i64,
            },
        }
    }
}
