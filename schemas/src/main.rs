use schemars::{schema::RootSchema, schema_for};
use std::env;

use una_core::types::{
    Backend, ChannelStats, CreateInvoiceParams, CreateInvoiceResult, Network, NodeConfig, NodeInfo,
};

fn write_schema(dir: &std::path::Path, name: &str, schema: &RootSchema) -> std::io::Result<()> {
    let output = serde_json::to_string_pretty(schema).unwrap();
    let output_path = dir.join(format!("{}.json", name));
    std::fs::write(output_path, output)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = match args.get(1) {
        Some(path) => std::path::PathBuf::from(path),
        None => std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("schemas"),
    };

    std::fs::create_dir_all(&dir).unwrap();

    let schema = schema_for!(Backend);
    write_schema(&dir, "backend", &schema).unwrap();

    let schema = schema_for!(Network);
    write_schema(&dir, "network", &schema).unwrap();

    let schema = schema_for!(NodeConfig);
    write_schema(&dir, "node_config", &schema).unwrap();

    let schema = schema_for!(NodeInfo);
    write_schema(&dir, "node_info", &schema).unwrap();

    let schema = schema_for!(ChannelStats);
    write_schema(&dir, "channel_stats", &schema).unwrap();

    let schema = schema_for!(CreateInvoiceParams);
    write_schema(&dir, "create_invoice_params", &schema).unwrap();

    let schema = schema_for!(CreateInvoiceResult);
    write_schema(&dir, "create_invoice_result", &schema).unwrap();

    println!("Wrote schemas to {}", dir.to_string_lossy());
}
