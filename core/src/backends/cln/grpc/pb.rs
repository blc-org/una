#![allow(clippy::from_over_into)]

use std::{convert::TryInto, time::UNIX_EPOCH};

use crate::{error::Error, types::*, utils};
use cuid;
use lightning_invoice;

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

impl TryInto<DecodeInvoiceResult> for String {
    type Error = Error;

    fn try_into(self) -> Result<DecodeInvoiceResult, Error> {
        // DecodeInvoice gRPC command is not implemented yet on CLN, so we need to use an external parser instead
        let parsed_invoice =
            str::parse::<lightning_invoice::Invoice>(self.as_ref()).map_err(|_| {
                Error::ConversionError(String::from(
                    "provided invoice is not a valid bolt11 invoice",
                ))
            })?;

        let memo = match parsed_invoice.description() {
            lightning_invoice::InvoiceDescription::Direct(direct) => Ok(Some(direct.to_string())),
            lightning_invoice::InvoiceDescription::Hash(_hash) => Err(Error::NotImplemented(
                String::from("Hash transcription is not supported yet"),
            )),
        }?;

        let invoice = DecodeInvoiceResult {
            creation_date: parsed_invoice
                .timestamp()
                .duration_since(UNIX_EPOCH)
                .map_err(|_| {
                    Error::ConversionError(String::from("could not convert creation_date"))
                })?
                .as_millis() as i64,
            amount: utils::get_amount_sat(None, parsed_invoice.amount_milli_satoshis()),
            amount_msat: parsed_invoice.amount_milli_satoshis(),
            destination: None,
            memo,
            payment_hash: parsed_invoice.payment_hash().to_string(),
            expiry: parsed_invoice.expiry_time().as_millis() as i32,
            min_final_cltv_expiry: parsed_invoice.min_final_cltv_expiry() as u32,
            features: None,
            route_hints: parsed_invoice
                .route_hints()
                .into_iter()
                .map(|route_hint| RoutingHint {
                    hop_hints: route_hint
                        .0
                        .into_iter()
                        .map(|hop_hint| HopHint {
                            node_id: hop_hint.src_node_id.to_string(),
                            chan_id: hop_hint.short_channel_id.to_string(),
                            fee_base_msat: hop_hint.fees.base_msat,
                            fee_proportional_millionths: hop_hint.fees.proportional_millionths,
                            cltv_expiry_delta: hop_hint.cltv_expiry_delta as u32,
                        })
                        .collect(),
                })
                .collect(),
        };

        Ok(invoice)
    }
}
