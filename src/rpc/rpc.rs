use std::str::FromStr;
use std::time::Duration;

use crate::config::rpc_config::get_rpc_client;
use solana_client::client_error::{ClientError, ClientErrorKind};
use solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config;
use solana_rpc_client_api::config::RpcTransactionConfig;
use solana_rpc_client_api::response::RpcConfirmedTransactionStatusWithSignature;
use solana_sdk::signature::Signature;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use solana_transaction_status::{EncodedConfirmedTransactionWithStatusMeta, UiTransactionEncoding};

pub async fn get_transaction_with_retries(
    signature: &str,
) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
    let rpc_client = get_rpc_client();

    const MAX_RETRIES: usize = 30;
    let mut delay = 10;

    for _ in 0..MAX_RETRIES {
        let transaction = rpc_client
            .get_transaction_with_config(
                &Signature::from_str(signature).expect("Invalid transaction signature"),
                RpcTransactionConfig {
                    max_supported_transaction_version: Some(0),
                    encoding: Some(UiTransactionEncoding::Base58),
                    commitment: Some(CommitmentConfig::confirmed()),
                },
            )
            .await;

        // TODO: Handle rate limits & retries
        match transaction {
            Ok(transaction) => {
                return Ok(transaction);
            }
            Err(e) => {
                println!("Error: {:?}", e);

                delay += 100;
                println!("Retrying in {}ms", delay);
                tokio::time::sleep(Duration::from_millis(delay)).await;
            }
        }
    }

    Err(ClientError {
        kind: ClientErrorKind::Custom("Failed to get transaction".to_string()),
        request: None,
    })
}

pub async fn get_signatures_for_tree(
    tree_address: &str,
    last_processed_tx: Option<&String>,
) -> Vec<RpcConfirmedTransactionStatusWithSignature> {
    let tree_address_pubkey = Pubkey::from_str(tree_address).expect("Invalid tree address");

    let last_processed_tx_signature = last_processed_tx
        .map(|signature| Signature::from_str(signature).expect("Invalid signature"));

    let rpc_client = get_rpc_client();

    rpc_client
        .get_signatures_for_address_with_config(
            &tree_address_pubkey,
            GetConfirmedSignaturesForAddress2Config {
                commitment: Some(CommitmentConfig::confirmed()),
                before: last_processed_tx_signature,
                ..Default::default()
            },
        )
        .await
        .expect("Failed to get signatures for tree")
}
