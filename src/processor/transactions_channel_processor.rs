use program_transformers::ProgramTransformer;
use solana_client::rpc_response::RpcLogsResponse;

use crate::rpc::rpc::get_transaction_with_retries;

use super::transaction::process_transaction;

pub async fn process_transactions_channel(
    mut receiver: tokio::sync::mpsc::UnboundedReceiver<RpcLogsResponse>,
    program_transformer: &ProgramTransformer,
) {
    loop {
        if let Some(logs) = receiver.recv().await {
            let transaction_signature = logs.signature;

            if let Ok(transaction) = get_transaction_with_retries(&transaction_signature).await {
                if let Err(e) = process_transaction(&program_transformer, transaction).await {
                    eprintln!("Transaction processing error: {:?}", e);
                }
            }
        }
    }
}
