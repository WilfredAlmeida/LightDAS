use solana_rpc_client_api::response::RpcConfirmedTransactionStatusWithSignature;

use crate::rpc::rpc::{get_signatures_for_tree, get_transaction_with_retries};

pub async fn backfill_tree(tree_address: String) {
    // TODO: Get this from database
    let mut last_processed_tx: Option<&String> = None;
    let mut signatures: Vec<RpcConfirmedTransactionStatusWithSignature>;

    loop {
        signatures = get_signatures_for_tree(&tree_address, last_processed_tx).await;

        last_processed_tx = Some(&signatures[0].signature);

        for signature in &signatures {
            let transaction = get_transaction_with_retries(&signature.signature)
                .await
                .unwrap();

            // process_transaction(transaction);
            println!("backfill tx");
        }

        if signatures.len() < 1000 {
            break;
        }
    }
}
