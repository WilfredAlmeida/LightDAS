use program_transformers::ProgramTransformer;
use sea_orm::{ConnectionTrait, SqlxPostgresConnector, Statement, Value};
use sqlx::PgPool;

use crate::{config::transaction_queue::pop_front, rpc::rpc::get_transaction_with_retries};

use super::transaction::process_transaction;
use futures::future::{ready, FutureExt};

pub async fn process_transactions_queue(database_pool: PgPool) {
    let program_transformer = ProgramTransformer::new(
        database_pool.clone(),
        Box::new(|_info| ready(Ok(())).boxed()),
        false,
    );
    loop {
        let transaction = pop_front();
        // println!("TX: {:?}", transaction.clone());
        let transaction_signature: &String;

        match transaction {
            Some(txs) => {
                transaction_signature = &txs.transaction_signature;
                let tree_address = txs.tree_address;
                let transaction = get_transaction_with_retries(&transaction_signature)
                    .await
                    .unwrap();

                if let Ok(_) = process_transaction(&program_transformer, transaction).await {
                    if let Some(tree_address_string) = tree_address {
                        let db_connection =
                            SqlxPostgresConnector::from_sqlx_postgres_pool(database_pool.clone());

                        let query: &str="UPDATE ld_merkle_trees SET last_processed_signature=$1 WHERE address=$2;";

                        let res = db_connection
                            .execute(Statement::from_sql_and_values(
                                sea_orm::DatabaseBackend::Postgres,
                                query,
                                vec![
                                    Value::from(transaction_signature.as_str()),
                                    Value::from(tree_address_string.as_str()),
                                ],
                            ))
                            .await;

                        match res {
                            Ok(_) => {}
                            Err(e) => {
                                println!("Failed to update `ld_merkle_trees` column `last_processed_signature` with error {:?}", e);
                            }
                        }
                    }
                }
            }
            None => {
                // println!("No transactions in queue");
            }
        }
    }
}
