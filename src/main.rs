use std::pin::Pin;

use std::str::FromStr;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::config::database::setup_database_config;
use crate::config::env_config::{setup_env_config, EnvConfig};
use anyhow::Result;
use config::rpc_config::{get_pubsub_client, setup_rpc_clients};
use das_bubblegum_backfill::worker::{
    GapWorkerArgs, ProgramTransformerWorkerArgs, SignatureWorkerArgs,
};
use das_bubblegum_backfill::{
    start_bubblegum_backfill, BubblegumBackfillArgs, BubblegumBackfillContext,
};
use das_core::{MetadataJsonDownloadWorkerArgs, Rpc, SolanaRpcArgs};
use dotenv::dotenv;

use futures::prelude::*;

use log::info;
use mpl_token_metadata::types::Data;
use processor::transactions_channel_processor::process_transactions_channel;
use program_transformers::ProgramTransformer;

use sea_orm::{ConnectionTrait, DatabaseConnection, SqlxPostgresConnector, Statement};
use solana_client::rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter};
use solana_client::rpc_response::{Response, RpcLogsResponse};
use solana_sdk::commitment_config::CommitmentConfig;

use solana_sdk::pubkey::Pubkey;
use sqlx::{Pool, Postgres};

use tokio::task::{self};

use signal_hook::{consts::signal::SIGHUP, iterator::Signals};

mod config;
mod processor;
mod rpc;

struct State {
    tree_addresses: Vec<String>,
    tasks: Vec<(String, task::JoinHandle<()>)>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let env_config = setup_env_config();

    setup_rpc_clients(&env_config).await;

    let database_pool = setup_database_config(&env_config).await;

    if let Err(e) = configure_database(SqlxPostgresConnector::from_sqlx_postgres_pool(
        database_pool.clone(),
    ))
    .await
    {
        panic!("Error configuring database: {:?}", e);
    }

    let tree_addresses = match get_trees(SqlxPostgresConnector::from_sqlx_postgres_pool(
        database_pool.clone(),
    ))
    .await
    {
        Ok(tree_addresses) => tree_addresses,
        Err(e) => {
            eprintln!("Error getting trees: {:?}", e);
            return Err(e);
        }
    };

    if tree_addresses.is_empty() {
        eprintln!("No trees found. Exiting...");
    }

    let state = Arc::new(std::sync::Mutex::new(State {
        tree_addresses,
        tasks: vec![],
    }));

    let state_clone = Arc::clone(&state);

    let (signal_tx, mut signal_rx) = tokio::sync::mpsc::channel(1);

    // thread to handle SIGHUP
    thread::spawn(move || {
        let mut signals = Signals::new(&[SIGHUP]).unwrap();
        for _ in signals.forever() {
            let _ = signal_tx.blocking_send(());
        }
    });

    let mut state = state_clone.lock().unwrap();
    reload_tasks(&mut *state, database_pool.clone(), env_config);

    loop {
        tokio::select! {
            _ = tokio::time::sleep(Duration::from_secs(1)) => {
                // publish metrics
            },
            _ = signal_rx.recv() => {
                println!("Received SIGHUP, reloading...");

                let trees = get_trees(SqlxPostgresConnector::from_sqlx_postgres_pool(
                    database_pool.clone(),
                ))
                .await
                .unwrap();

                state.tree_addresses = trees;

                reload_tasks(
                    &mut state,
                    database_pool.clone(),
                    setup_env_config(),
                );
            }
        }
    }
}

async fn handle_stream(
    mut stream: Pin<Box<dyn Stream<Item = Response<RpcLogsResponse>> + Send + 'static>>,
    sender: tokio::sync::mpsc::UnboundedSender<RpcLogsResponse>,
) {
    loop {
        if let Some(logs) = stream.next().await {
            if let Err(e) = sender.send(logs.value) {
                eprintln!("Error sending logs to transaction processing: {:?}", e);
            }
        }
    }
}

async fn get_trees(database_connection: DatabaseConnection) -> Result<Vec<String>> {
    let res = database_connection
        .query_all(Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            String::from("SELECT * FROM ld_merkle_trees WHERE should_index IS TRUE;"),
        ))
        .await;

    let mut tree_addresses: Vec<String> = Vec::new();
    match res {
        Ok(rows) => {
            if rows.len() == 0 {
                panic!("Trees to index not found in the database");
            }

            rows.iter().for_each(|row| {
                let tree_address: Option<String> = row.try_get("", "address").unwrap();
                match tree_address {
                    Some(s) => {
                        if let Ok(_) = Pubkey::from_str(s.as_str()) {
                            tree_addresses.push(s);
                        } else {
                            eprintln!("Invalid tree address {:?}", s)
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

    Ok(tree_addresses)
}

fn reload_tasks(state: &mut State, database_pool: Pool<Postgres>, env_config: EnvConfig) {
    state.tasks.retain(|(s, handle)| {
        if !state.tree_addresses.contains(s) {
            handle.abort();
            false
        } else {
            true
        }
    });

    let context = BubblegumBackfillContext::new(
        database_pool.clone(),
        Rpc::from_config(&SolanaRpcArgs {
            solana_rpc_url: env_config.get_rpc_url().to_string(),
        }),
    );

    let tree_addresses = state.tree_addresses.clone();

    for address in tree_addresses {
        let address_clone = address.clone();

        let program_transformer = ProgramTransformer::new(
            database_pool.clone(),
            Box::new(|_info| futures::future::ready(Ok(())).boxed()),
            false,
        );

        let context = context.clone();

        let args = BubblegumBackfillArgs {
            only_trees: Some(vec![address.clone().to_string()]),
            tree_crawler_count: 4,
            tree_worker: das_bubblegum_backfill::worker::TreeWorkerArgs {
                metadata_json_download_worker: MetadataJsonDownloadWorkerArgs {
                    metadata_json_download_worker_count: 100,
                    metadata_json_download_worker_request_timeout: 200,
                },
                signature_worker: SignatureWorkerArgs {
                    signature_channel_size: 100,
                    signature_worker_count: 100,
                },
                gap_worker: GapWorkerArgs {
                    gap_channel_size: 100,
                    gap_worker_count: 100,
                },
                program_transformer_worker: ProgramTransformerWorkerArgs {
                    program_transformer_channel_size: 100,
                },
            },
        };

        let task_handle = task::spawn(async move {
            let address = address.clone();
            let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<RpcLogsResponse>();

            let stream: Pin<Box<dyn Stream<Item = Response<RpcLogsResponse>> + Send>> =
                get_pubsub_client()
                    .logs_subscribe(
                        RpcTransactionLogsFilter::Mentions(vec![address.clone().to_string()]),
                        RpcTransactionLogsConfig {
                            commitment: Some(CommitmentConfig::processed()),
                        },
                    )
                    .await
                    .unwrap()
                    .0;

            task::spawn(async move {
                handle_stream(stream, tx).await;
            });

            println!("Backfill started for tree: {:}", address);

            if let Err(e) = start_bubblegum_backfill(context.clone(), args).await {
                eprintln!("Error backfilling tree {:?}: {:?}", address.clone(), e);
            }

            println!("Backfill finished and for tree: {:}", address);
            println!("Starting live indexing for tree: {:}", address);

            process_transactions_channel(rx, &program_transformer).await;
        });

        state.tasks.push((address_clone, task_handle));
    }
}

async fn configure_database(
    database_connection: DatabaseConnection,
) -> Result<sea_orm::ExecResult, sea_orm::DbErr> {
    // Refer https://github.com/WilfredAlmeida/LightDAS/issues/4 to understand why this is needed
    database_connection
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
                    CREATED_AT TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                    UPDATED_AT TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                );",
            ),
        ))
        .await
}
