use sea_orm::{ConnectionTrait, DatabaseConnection, Statement};
use solana_rpc_client_api::response::RpcConfirmedTransactionStatusWithSignature;

use crate::{
    config::transaction_queue::{push_front, TransactionsQueue},
    rpc::rpc::get_signatures_for_tree,
};

pub async fn backfill_tree(tree_address: String, db_connection: DatabaseConnection) {
    let mut last_processed_tx: Option<String> = None;
    let mut until_signature: Option<String> = None;
    let mut genesis_backfill_completed: bool = false;

    let query = "SELECT last_processed_signature,genesis_backfill_completed FROM ld_merkle_trees WHERE address=$1;";

    let query_res = db_connection
        .query_one(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            query,
            vec![sea_orm::Value::from(tree_address.as_str())],
        ))
        .await
        .unwrap();

    match query_res {
        Some(row) => {
            let sign: Option<String> = row.try_get("", "last_processed_signature").unwrap_or(None);
            println!("last_processed_signature: {:?}", sign);
            last_processed_tx = sign;

            let gbc: bool = row
                .try_get("", "genesis_backfill_completed")
                .unwrap_or(false);

            genesis_backfill_completed = gbc;
        }
        None => {
            println!(
                "No last_processed_signature found for tree: {}",
                tree_address
            );
        }
    }

    if genesis_backfill_completed {
        until_signature = last_processed_tx.clone();
        last_processed_tx.take();
    }

    println!(
        "genesis_backfill_completed: {:?}",
        genesis_backfill_completed
    );
    println!("until_signature: {:?}", until_signature);
    println!("last processed: {:?}", last_processed_tx);

    let mut signatures: Vec<RpcConfirmedTransactionStatusWithSignature>;

    loop {
        signatures = get_signatures_for_tree(
            &tree_address,
            last_processed_tx.as_ref(),
            until_signature.as_ref(),
        )
        .await;

        if signatures.len() == 0 {
            break;
        }

        last_processed_tx = Some(signatures[signatures.len() - 1].signature.clone());
        println!("last_processed_tx: {:?}", last_processed_tx);

        for signature in &signatures {
            println!("backfill tx {:?}", signature.signature);
            push_front(TransactionsQueue {
                transaction_signature: signature.signature.clone(),
                tree_address: tree_address.clone().into(),
            })
        }

        if signatures.len() < 1000 {
            break;
        }
    }
}
