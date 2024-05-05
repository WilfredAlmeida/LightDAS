use solana_client::rpc_response::RpcLogsResponse;

use crate::{
    config::transaction_queue::{push_back, TransactionsQueue},
    rpc::rpc::get_transaction_with_retries,
};

pub async fn process_logs(logs_response: RpcLogsResponse) {
    let transaction_signature = logs_response.signature;

    println!("websocket tx");
    push_back(TransactionsQueue {
        transaction_signature: transaction_signature.clone(),
        tree_address: None,
    });
}
