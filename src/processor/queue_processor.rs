use program_transformers::ProgramTransformer;
use sqlx::{Pool, Postgres};

use crate::{config::transaction_queue::pop_front, rpc::rpc::get_transaction_with_retries};

use super::transaction::process_transaction;
use futures::future::{ready, FutureExt};

pub async fn process_transactions_queue(database_pool: Pool<Postgres>) {
  let program_transformer = ProgramTransformer::new(database_pool, Box::new(|_info| ready(Ok(())).boxed()), false);
  loop {
    let transaction_signature = pop_front();

    match transaction_signature {
        Some(txs) =>{
          let transaction =
            get_transaction_with_retries(&txs.transaction_signature)
                .await
                .unwrap();

            process_transaction(&program_transformer, transaction).await;
        }
        None => {
            println!("No transactions in queue");
        }
    }
  }
}
