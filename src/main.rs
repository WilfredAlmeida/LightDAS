use anyhow::Result;
use futures::prelude::*;
use processor::logs::process_logs;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter},
};
use solana_sdk::commitment_config::CommitmentConfig;

const WS_URL: &str = "";

mod processor;
mod rpc;

#[tokio::main]
async fn main() -> Result<()> {

    let tree_addresses: Vec<String> = vec!["GXTXbFwcbNdWbiCWzZc3J2XGofopnhN9T98jnG29D2Yw".to_string()];

    let client = PubsubClient::new(WS_URL).await?;
    let (mut stream, _) = client
        .logs_subscribe(
            RpcTransactionLogsFilter::Mentions(tree_addresses),
            RpcTransactionLogsConfig {
                commitment: Some(CommitmentConfig::processed()),
            },
        )
        .await?;

    loop {
        
        let logs = stream.next().await.unwrap();
        // println!("logs: {:?}", logs);
        process_logs(logs.value).await;
    }
}