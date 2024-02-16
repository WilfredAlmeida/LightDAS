use anyhow::Result;
use config::rpc_config::{get_pubsub_client, initialize_clients};
use dotenv::dotenv;
use futures::prelude::*;
use futures::{future::join_all, stream::select_all};
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

    let pubsub_client = get_pubsub_client();

    let tree_addresses: Vec<String> = vec![
        "GXTXbFwcbNdWbiCWzZc3J2XGofopnhN9T98jnG29D2Yw".to_string(),
        // "Aju7YfPdhjaqJbRdow48PqxcWutDDHWww6eoDC9PVY7m".to_string(),
    ];

    let mut stream = select_all(
        join_all(tree_addresses.iter().map(|address| {
            pubsub_client.logs_subscribe(
                RpcTransactionLogsFilter::Mentions(vec![address.to_string()]),
                RpcTransactionLogsConfig {
                    commitment: Some(CommitmentConfig::processed()),
                },
            )
        }))
        .await
        .into_iter()
        .flat_map(|result| match result {
            Ok(subscription) => Some(subscription.0),
            Err(e) => {
                eprintln!("error creating subscription: {e}");
                None
            }
        }),
    );

    loop {
        let logs = stream.next().await.unwrap();
        process_logs(logs.value).await;
    }
}
