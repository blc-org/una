#![allow(dead_code)]

#[macro_use]
extern crate napi_derive;

use napi::{bindgen_prelude::*, Env, Error, JsObject, Result};

use std::sync::Arc;
use tokio::sync::Mutex;
use una_core::{
    backends::lnd::rest::node::LndRest,
    node::{Node, NodeMethods},
    types::{Backend, CreateInvoiceParams, NodeConfig, NodeInfo},
};

#[napi(js_name = "Node")]
struct JsNode(Arc<Mutex<Node>>);

#[napi]
impl JsNode {
    #[napi(constructor, ts_args_type = "backend: Backend, config: NodeConfig")]
    pub fn new(env: Env, backend: String, config: JsObject) -> Result<JsNode> {
        let backend: Backend = backend.as_str().into();
        let config: NodeConfig = env.from_js_value(config)?;

        match backend {
            Backend::LndRest => {
                let node = LndRest::new(config).unwrap();
                Ok(Self(Arc::new(Mutex::new(Node {
                    backend,
                    node: Box::new(node),
                }))))
            }
            Backend::LndGrpc | Backend::ClnRest => todo!(),
            Backend::InvalidBackend => Err(Error::new(
                Status::InvalidArg,
                "Invalid backend".to_string(),
            )),
        }
    }

    #[napi(
        ts_args_type = "invoice: CreateInvoiceParams",
        ts_return_type = "CreateInvoiceResult"
    )]
    pub fn create_invoice(&self, env: Env, invoice: JsObject) -> Result<JsObject> {
        let node = self.0.clone();

        let invoice: CreateInvoiceParams = env.from_js_value(invoice)?;

        env.execute_tokio_future(
            async move {
                let payreq = node.lock().await.create_invoice(invoice).await.unwrap();
                Ok(payreq)
            },
            |&mut env, payreq| Ok(env.to_js_value(&payreq)),
        )
    }

    #[napi(ts_return_type = "NodeInfo")]
    pub fn get_info(&self, env: Env) -> Result<JsObject> {
        let node = self.0.clone();

        env.execute_tokio_future(
            async move {
                let info: NodeInfo = node.lock().await.get_info().await.unwrap();
                Ok(info)
            },
            |&mut env, info| Ok(env.to_js_value(&info)),
        )
    }
}
