use std::iter;
use std::pin::Pin;
use std::str::{Bytes, FromStr};
// use std::thread::sleep;
use std::time::Duration;

use crate::config::database::setup_database_config;
use crate::config::env_config::{setup_env_config, EnvConfig};
use anyhow::Result;
use backfill::backfill::backfill_tree;
use config::rpc_config::{get_pubsub_client, setup_rpc_clients};
use dotenv::dotenv;
use futures::future::join;
use futures::prelude::*;
use futures::stream::SelectAll;
use futures::{future::join_all, stream::select_all};
use mpl_bubblegum::accounts::MerkleTree;
use processor::logs::process_logs;
use processor::metadata::fetch_store_metadata;
use processor::queue_processor::process_transactions_queue;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, SqlxPostgresConnector, Statement};
use solana_client::rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter};
use solana_client::rpc_response::{Response, RpcLogsResponse};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use sqlx::{Acquire, PgPool};
use tokio::task;
use tokio::time::sleep;

mod backfill;
mod config;
mod processor;
mod rpc;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let env_config = setup_env_config();

    setup_rpc_clients(&env_config).await;

    let database_pool = setup_database_config(&env_config).await;

    let db_connection = SqlxPostgresConnector::from_sqlx_postgres_pool(database_pool.clone());

    // Refer https://github.com/WilfredAlmeida/LightDAS/issues/4 to understand why this is needed
    let _ = db_connection
        .execute(Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            String::from(
                "CREATE TABLE IF NOT EXISTS LD_MERKLE_TREES (
                    ADDRESS VARCHAR(255),
                    TAG VARCHAR(255) NULL,
                    CAPACITY INT NULL,
                    MAX_DEPTH INT NULL,
                    CANOPY_DEPTH INT NULL,
                    MAX_BUFFER_SIZE INT NULL,
                    SHOULD_INDEX BOOLEAN DEFAULT TRUE,
                    GENESIS_BACKFILL_COMPLETED BOOLEAN DEFAULT FALSE,
                    LAST_PROCESSED_SIGNATURE VARCHAR(255) NULL,
                    CREATED_AT TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    UPDATED_AT TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                );",
            ),
        ))
        .await?;

    let pubsub_client = get_pubsub_client();

    let res = db_connection
        .query_all(Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            String::from("SELECT * FROM ld_merkle_trees;"),
        ))
        .await;

    let mut tree_addresses: Vec<String> = Vec::new();
    match res {
        Ok(rows) => {
            if rows.len() == 0 {
                panic!("Trees to index not found in database");
            }

            rows.iter().for_each(|row| {
                let tree_address: Option<String> = row.try_get("", "address").unwrap();
                match tree_address {
                    Some(s) => {
                        if let Ok(_) = Pubkey::from_str(s.as_str()) {
                            tree_addresses.push(s);
                        } else {
                            println!("Invalid tree address {:?}", s)
                        }
                    }
                    None => {}
                }
            });
        }
        Err(e) => {
            panic!("Error fetching trees to index {:?}", e)
        }
    }

    // let tree_addresses: Vec<String> = vec![
    //     // "GXTXbFwcbNdWbiCWzZc3J2XGofopnhN9T98jnG29D2Yw".to_string(),
    //     // "Aju7YfPdhjaqJbRdow48PqxcWutDDHWww6eoDC9PVY7m".to_string(),
    //     // "43XAHmPkq8Yth3swdqrh5aZvWrmuci5ZhPVLptreaUZ1".to_string(),
    //     // "EQQiiEceUo2uxHQgtRt8W92frLXwMUwdvt7P9Yo26cUM".to_string(),
    //     // "CkSa2n2eyJvsPLA7ufVos94NAUTYuVhaxrvH2GS69f9j".to_string()
    //     // "Dbx2uKULg44XeBR28tNWu2dU4bPpGfuYrd7RntgGXvuT".to_string(),
    //     // "CkSa2n2eyJvsPLA7ufVos94NAUTYuVhaxrvH2GS69f9j".to_string(),
    //     // "EBFsHQKYCn1obUr2FVNvGTkaUYf2p5jao2MVdbK5UNRH".to_string(),
    //     // "14b9wzhVSaiUHB4t8tDY9QYNsGStT8ycaoLkBHZLZwax".to_string(),
    //     // "6kAoPaZV4aB1rMPTPkbgycb9iNbHHibSzjhAvWEroMm".to_string(),
    //     // "FmUjM4YBLK93WSb7AnbuYZy1h2kCcjZM8kHsi9ZU93TP".to_string(),
    //     // "6JTnMcq9a6atrqmsz4rgTWp9EG5YPzxoobD7vg1csNt5".to_string(),
    //     // "HVGMVJ7DyfXLU2H5AJSHvX2HkFrRrHQAoXAHfYUmicYr".to_string(),
    //     // "D8yRakvsjWSR3ihANhwjP8RmNLg3A46EA1V1EbMLDT8B".to_string(),
    //     "B1eWW3tTBb5DHrwVrqJximAYLwucGzvjuJWxkFAe4v2X".to_string(),
    // ];

    println!("TREE ADDRESSES {:?}", tree_addresses);

    let mut stream = select_all(
        join_all(tree_addresses.iter().map(|address| {
            pubsub_client.logs_subscribe(
                RpcTransactionLogsFilter::Mentions(vec![address.to_string()]),
                RpcTransactionLogsConfig {
                    commitment: Some(CommitmentConfig::processed()),
                },
            )
        }))
        .await
        .into_iter()
        .flat_map(|result| match result {
            Ok(subscription) => Some(subscription.0),
            Err(e) => {
                eprintln!("error creating subscription: {e}");
                None
            }
        }),
    );

    let handle = task::spawn(handle_stream(stream));

    task::spawn(handle_metadata_downloads(database_pool.clone()));

    // join_all(tree_addresses.into_iter().map(|tr| {
    //     let db_connection_1 = SqlxPostgresConnector::from_sqlx_postgres_pool(database_pool.clone());
    //     let db_connection_2 = SqlxPostgresConnector::from_sqlx_postgres_pool(database_pool.clone());

    //     let backfill_future = backfill_tree(tr.clone(), db_connection_1);

    //     async move {
    //         let _ = backfill_future.await;

    //         let _ = db_connection_2
    //             .execute(Statement::from_sql_and_values(
    //                 sea_orm::DatabaseBackend::Postgres,
    //                 "UPDATE ld_merkle_trees SET genesis_backfill_completed=$1 WHERE address=$2;",
    //                 vec![
    //                     sea_orm::Value::from(true),
    //                     sea_orm::Value::from(tr.as_str()),
    //                 ],
    //             ))
    //             .await;
    //     }
    // }))
    // .await;

    // tasks spawned to process transactions from queue. depending on your tree and queue sizes, adjust this
    futures::future::join_all(
        iter::repeat_with(|| process_transactions_queue(database_pool.clone()))
            .take(15)
            .collect::<Vec<_>>(),
    )
    .await;

    Ok(())
}

async fn handle_stream(
    mut stream: SelectAll<Pin<Box<dyn Stream<Item = Response<RpcLogsResponse>> + Send>>>,
) {
    loop {
        if let Some(logs) = stream.next().await {
            process_logs(logs.value).await;
        }
    }
}

async fn handle_metadata_downloads(pool: PgPool) {
    let connection = SqlxPostgresConnector::from_sqlx_postgres_pool(pool);
    loop {
        let _ = fetch_store_metadata(&connection).await;
        println!("No metadata to update, sleeping for 5 secs");
        sleep(Duration::from_secs(5)).await;
    }
}
