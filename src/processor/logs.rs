use solana_client::rpc_response::RpcLogsResponse;

use crate::rpc::rpc::get_transaction_with_retries;

use super::transaction::process_transaction;


pub async fn process_logs(logs_response: RpcLogsResponse) {
    let transaction_signature = logs_response.signature;

    let transaction = get_transaction_with_retries(&transaction_signature).await.expect("Failed to get transaction");

    process_transaction(transaction)
}
