use clap::{Arg, Command};
use una_core::{
    node::NodeMethods,
    types::{Backend, CreateInvoiceParams, NodeConfig},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("una-cli")
        .version("1.0")
        .author("Bitcoin, Lightning and Camembert")
        .about("Universal Node API, control any node backend from the command-line")
        .arg(
            Arg::new("backend")
                .short('b')
                .long("backend")
                .value_parser(["LndRest", "ClnGrpc"])
                .help("Specifies the node backend")
                .takes_value(true),
        )
        .arg(
            Arg::new("url")
                .long("url")
                .help("[LndRest,ClnGrpc] Sets the node URL")
                .takes_value(true)
                .requires_if("LndRest", "backend")
                .requires_if("ClnGrpc", "backend"),
        )
        .arg(
            Arg::new("macaroon")
                .long("macaroon")
                .help("[LndRest] Sets the node macaroon")
                .takes_value(true)
                .requires_if("LndRest", "backend"),
        )
        .arg(
            Arg::new("tls_certificate")
                .long("tls_certificate")
                .help("[LndRest,ClnGrpc] Sets the node self-signed TLS certificate")
                .takes_value(true)
                .requires_if("LndRest", "backend")
                .requires_if("ClnGrpc", "backend"),
        )
        .arg(
            Arg::new("tls_client_certificate")
                .long("tls_client_certificate")
                .help("[ClnGrpc] Sets the client identity TLS certificate")
                .takes_value(true)
                .requires_if("ClnGrpc", "backend"),
        )
        .arg(
            Arg::new("tls_client_key")
                .long("tls_client_key")
                .help("[ClnGrpc] Sets the client identity TLS key")
                .takes_value(true)
                .requires_if("ClnGrpc", "backend"),
        )
        .subcommand(Command::new("info").about("see information about your node"))
        .subcommand(
            Command::new("createinvoice")
                .about("create new invoice")
                .arg(
                    Arg::new("amount")
                        .required(true)
                        .index(1)
                        .help("amount in sats"),
                )
                .arg(
                    Arg::new("description")
                        .required(false)
                        .index(2)
                        .help("description"),
                ),
        )
        .get_matches();

    let backend: Backend = matches
        .value_of("backend")
        .expect("backend is required")
        .into();

    let config = NodeConfig {
        url: matches.value_of("url").map(|s| s.to_string()),
        macaroon: matches.value_of("macaroon").map(|s| s.to_string()),
        tls_certificate: matches.value_of("tls_certificate").map(|s| s.to_string()),
        tls_client_certificate: matches
            .value_of("tls_client_certificate")
            .map(|s| s.to_string()),
        tls_client_key: matches.value_of("tls_client_key").map(|s| s.to_string()),
    };

    let node = una_core::node::Node::new(backend, config).unwrap();

    let (command, command_args) = matches.subcommand().unwrap();

    match command {
        "info" => {
            let info = node.get_info().await.unwrap();
            println!("{:}", serde_json::to_string_pretty(&info).unwrap());
        }
        "createinvoice" => {
            let args = command_args;
            let amount: u64 = args
                .value_of("amount")
                .expect("amount is a required field")
                .parse()
                .expect("amount must be in satoshis");

            let description = args.value_of("description");

            let invoice = node
                .create_invoice(CreateInvoiceParams {
                    amount: Some(amount),
                    description: Some(description.unwrap_or("").to_string()),
                    amount_msat: None,
                    description_hash: None,
                    label: None,
                    expire_in: None,
                    fallback_address: None,
                    payment_preimage: None,
                    cltv_expiry: None,
                })
                .await
                .unwrap();

            println!("{:}", serde_json::to_string_pretty(&invoice).unwrap());
        }
        _ => {
            println!("invalid command. use una-cli --help to see usage instructions.")
        }
    }

    Ok(())
}
