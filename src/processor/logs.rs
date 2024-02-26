use solana_client::rpc_response::RpcLogsResponse;

use crate::{
    config::queue::{push_back, TransactionsQueue},
    rpc::rpc::get_transaction_with_retries,
};

pub async fn process_logs(logs_response: RpcLogsResponse) {
    let transaction_signature = logs_response.signature;

    let transaction = get_transaction_with_retries(&transaction_signature)
        .await
        .expect("Failed to get transaction");

    // push_back(TransactionsQueue{})

    // process_transaction(transaction)
    println!("websocket tx");
}
