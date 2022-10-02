#![allow(clippy::from_over_into)]

use std::time::UNIX_EPOCH;

use crate::{
    error::Error,
    types::*,
    utils::{self, msat_to_sat},
};
use cuid;

include!(concat!(env!("PROTOBUFS_DIR"), "/cln.rs"));

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

impl TryInto<Invoice> for ListinvoicesResponse {
    type Error = Error;

    fn try_into(self) -> Result<Invoice, Error> {
        let invoice: &ListinvoicesInvoices = self.invoices.get(0).expect("No invoice found");

        let bolt11 = match &invoice.bolt11 {
            Some(bolt11) => bolt11.as_ref(),
            None => return Err(Error::ApiError(String::from("bolt11 missing"))),
        };

        let decoded_invoice = str::parse::<lightning_invoice::Invoice>(bolt11)
            .expect("bolt11 is not a valid invoice");

        let mut settle_date: Option<u64> = None;
        let settled = match invoice.status {
            1 => match invoice.paid_at {
                Some(e) => {
                    settle_date = Some(e);
                    true
                }
                None => true,
            },
            _ => false,
        };

        let status = match invoice.status {
            0 => crate::types::InvoiceStatus::Pending,
            1 => crate::types::InvoiceStatus::Settled,
            2 => crate::types::InvoiceStatus::Cancelled,
            _ => crate::types::InvoiceStatus::Accepted,
        };

        let amount_msat = &decoded_invoice
            .amount_milli_satoshis()
            .ok_or_else(|| Error::ApiError(String::from("amount_milli_satoshis missing")))?;

        Ok(Invoice {
            bolt11: String::from(bolt11),
            memo: match &invoice.description {
                Some(description) => String::from(description),
                None => return Err(Error::ApiError(String::from("description missing"))),
            },
            amount: msat_to_sat(*amount_msat),
            amount_msat: *amount_msat,
            pre_image: invoice.payment_preimage.as_ref().map(hex::encode),
            payment_hash: decoded_invoice.payment_hash().to_string(),
            settled,
            settle_date,
            creation_date: decoded_invoice
                .timestamp()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            expiry: decoded_invoice.expiry_time().as_secs(),
            status,
        })
    }
}

impl From<PayInvoiceParams> for PayRequest {
    fn from(params: PayInvoiceParams) -> Self {
        let amount_msat =
            utils::get_amount_msat(params.amount, params.amount_msat).map(|v| Amount { msat: v });

        PayRequest {
            bolt11: params.payment_request,
            msatoshi: amount_msat,
            label: None,
            riskfactor: None,
            maxfeepercent: params.max_fee_percent,
            retry_for: None,
            maxdelay: None,
            exemptfee: None,
            localofferid: None,
            exclude: vec![],
            maxfee: params.max_fee_msat.map(|v| Amount { msat: v as u64 }),
            description: None,
        }
    }
}

impl Into<PayInvoiceResult> for PayResponse {
    fn into(self) -> PayInvoiceResult {
        let fees_msat = match (self.amount_msat, self.amount_sent_msat) {
            (Some(amount_msat), Some(amount_sent_msat)) => {
                Some(amount_sent_msat.msat - amount_msat.msat)
            }
            _ => None,
        };

        PayInvoiceResult {
            payment_preimage: hex::encode(self.payment_preimage),
            payment_hash: hex::encode(self.payment_hash),
            fees_msat,
        }
    }
}
