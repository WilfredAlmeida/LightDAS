use program_transformers::ProgramTransformer;
use sea_orm::{ConnectionTrait, SqlxPostgresConnector, Statement, Value};
use sqlx::PgPool;

use crate::{config::transaction_queue::pop_front, rpc::rpc::get_transaction_with_retries};

use super::transaction::process_transaction;
use futures::future::{ready, FutureExt};

use tokio::time::{sleep, Duration};

pub async fn process_transactions_queue(database_pool: PgPool) {
    let program_transformer = ProgramTransformer::new(
        database_pool.clone(),
        Box::new(|_info| ready(Ok(())).boxed()),
        false,
    );

    loop {
        if let Some(txs) = pop_front() {
            let transaction_signature = &txs.transaction_signature;
            let tree_address = txs.tree_address;

            if let Ok(transaction) = get_transaction_with_retries(transaction_signature).await {
                if process_transaction(&program_transformer, transaction).await.is_ok() {
                    if let Some(tree_address_string) = tree_address {
                        let db_connection = SqlxPostgresConnector::from_sqlx_postgres_pool(database_pool.clone());
                        let query = "UPDATE ld_merkle_trees SET last_processed_signature=$1 WHERE address=$2;";

                        if let Err(e) = db_connection.execute(Statement::from_sql_and_values(
                            sea_orm::DatabaseBackend::Postgres,
                            query,
                            vec![
                                Value::from(transaction_signature.as_str()),
                                Value::from(tree_address_string.as_str()),
                            ],
                        )).await {
                            println!("Failed to update `ld_merkle_trees` column `last_processed_signature` with error {:?}", e);
                        }
                    }
                }
            }
        } else {
            sleep(Duration::from_millis(100)).await;
        }
    }
}
