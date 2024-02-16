use anyhow::Result;
use config::rpc_config::{get_pubsub_client, initialize_clients};
use dotenv::dotenv;
use futures::prelude::*;
use processor::logs::process_logs;
use solana_client::rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter};
use solana_sdk::commitment_config::CommitmentConfig;
mod config;
mod processor;
mod rpc;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    initialize_clients().await;

    let tree_addresses: Vec<String> = vec![
        "GXTXbFwcbNdWbiCWzZc3J2XGofopnhN9T98jnG29D2Yw".to_string(),
        // "Aju7YfPdhjaqJbRdow48PqxcWutDDHWww6eoDC9PVY7m".to_string(),
    ];

    let pubsub_client = get_pubsub_client();

    let (mut stream, _) = pubsub_client
        .logs_subscribe(
            RpcTransactionLogsFilter::Mentions(tree_addresses),
            RpcTransactionLogsConfig {
                commitment: Some(CommitmentConfig::processed()),
            },
        )
        .await?;

    loop {
        let logs = stream.next().await.unwrap();
        process_logs(logs.value).await;
    }
}
