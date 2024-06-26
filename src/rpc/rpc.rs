use std::str::FromStr;
use std::time::Duration;

use crate::config::rpc_config::get_rpc_client;
use solana_client::client_error::{ClientError, ClientErrorKind};
use solana_rpc_client_api::config::RpcTransactionConfig;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Signature;
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
                eprintln!("Error: {:?}", e);

                delay += 100;
                eprintln!("Retrying in {}ms", delay);
                tokio::time::sleep(Duration::from_millis(delay)).await;
            }
        }
    }

    Err(ClientError {
        kind: ClientErrorKind::Custom("Failed to get transaction".to_string()),
        request: None,
    })
}
