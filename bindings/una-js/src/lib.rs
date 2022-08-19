#![allow(dead_code)]

#[macro_use]
extern crate napi_derive;

use std::sync::Arc;
use tokio::sync::Mutex;

use napi::{Env, JsObject, Result};

use una_core::{
    backends::{
        cln::grpc::{config::ClnGrpcConfig, node::ClnGrpc},
        eclair::rest::{config::EclairRestConfig, node::EclairRest},
        lnd::rest::{config::LndRestConfig, node::LndRest},
    },
    error::Error as UnaError,
    node::{Node, NodeMethods},
    types::{Backend, CreateInvoiceParams, NodeConfig, NodeInfo, PayInvoiceParams},
};

pub mod error;

use error::OrNapiError;

#[napi(js_name = "Node")]
struct JsNode(Arc<Mutex<Node>>);

#[napi]
impl JsNode {
    #[napi(constructor, ts_args_type = "backend: Backend, config: NodeConfig")]
    pub fn new(env: Env, backend: String, config: JsObject) -> Result<JsNode> {
        let backend: Backend = backend.as_str().into();
        let config: NodeConfig = env.from_js_value(config)?;

        let node = match backend {
            Backend::LndRest => {
                let config = TryInto::<LndRestConfig>::try_into(config).or_napi_error()?;
                let node = LndRest::new(config).or_napi_error()?;
                Ok(Node {
                    backend,
                    node: Box::new(node),
                })
            }
            Backend::ClnGrpc => {
                let config = TryInto::<ClnGrpcConfig>::try_into(config).or_napi_error()?;
                let node = ClnGrpc::new(config).or_napi_error()?;
                Ok(Node {
                    backend,
                    node: Box::new(node),
                })
            }
            Backend::EclairRest => {
                let config = TryInto::<EclairRestConfig>::try_into(config).or_napi_error()?;
                let node = EclairRest::new(config).or_napi_error()?;
                Ok(Node {
                    backend,
                    node: Box::new(node),
                })
            }
            Backend::LndGrpc => todo!(),
            Backend::InvalidBackend => Err(UnaError::InvalidBackend),
        };

        Ok(Self(Arc::new(Mutex::new(node.or_napi_error()?))))
    }

    #[napi(
        ts_args_type = "invoice: CreateInvoiceParams",
        ts_return_type = "Promise<CreateInvoiceResult>"
    )]
    pub fn create_invoice(&self, env: Env, invoice: JsObject) -> Result<JsObject> {
        let node = self.0.clone();

        let invoice: CreateInvoiceParams = env.from_js_value(invoice)?;

        env.execute_tokio_future(
            async move {
                let payreq = node
                    .lock()
                    .await
                    .create_invoice(invoice)
                    .await
                    .or_napi_error()?;
                Ok(payreq)
            },
            |&mut env, payreq| Ok(env.to_js_value(&payreq)),
        )
    }

    #[napi(ts_return_type = "Promise<NodeInfo>")]
    pub fn get_info(&self, env: Env) -> Result<JsObject> {
        let node = self.0.clone();

        env.execute_tokio_future(
            async move {
                let info: NodeInfo = node.lock().await.get_info().await.or_napi_error()?;
                Ok(info)
            },
            |&mut env, info| Ok(env.to_js_value(&info)),
        )
    }

    #[napi(
        ts_args_type = "paymentHash: string",
        ts_return_type = "Promise<Invoice>"
    )]
    pub fn get_invoice(&self, env: Env, payement_hash: String) -> Result<JsObject> {
        let node = self.0.clone();

        env.execute_tokio_future(
            async move {
                let invoice = node.lock().await.get_invoice(payment_hash).await.or_napi_error()?;
                Ok(invoice)
            },
            |&mut env, invoice| Ok(env.to_js_value(&invoice)),
        )
    }

    #[napi(
        ts_args_type = "invoice: PayInvoiceParams",
        ts_return_type = "Promise<PayInvoiceResult>"
    )]
    pub fn pay_invoice(&self, env: Env, invoice: JsObject) -> Result<JsObject> {
        let node = self.0.clone();

        let invoice: PayInvoiceParams = env.from_js_value(invoice)?;

        env.execute_tokio_future(
            async move {
                let payreq = node
                    .lock()
                    .await
                    .pay_invoice(invoice)
                    .await
                    .or_napi_error()?;
                Ok(payreq)
            },
            |&mut env, payreq| Ok(env.to_js_value(&payreq)),
        )
    }
}
