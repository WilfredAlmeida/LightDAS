use solana_rpc_client_api::response::RpcConfirmedTransactionStatusWithSignature;

use crate::{
    config::queue::{push_front, TransactionsQueue},
    rpc::rpc::{get_signatures_for_tree, get_transaction_with_retries},
};

use crate::processor::transaction::process_transaction;

pub async fn backfill_tree(tree_address: String) {
    // TODO: Get this from database
    let mut last_processed_tx: Option<&String> = None;
    let mut signatures: Vec<RpcConfirmedTransactionStatusWithSignature>;
    // let transactions_queue = get_queue();

    loop {
        signatures = get_signatures_for_tree(&tree_address, last_processed_tx).await;

        last_processed_tx = Some(&signatures[0].signature);

        for signature in &signatures {
            
            println!("backfill tx");

            push_front(TransactionsQueue {
                transaction_signature: signature.signature.clone(),
            })
        }

        if signatures.len() < 1000 {
            break;
        }
    }
}
