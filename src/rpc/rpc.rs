use std::str::FromStr;
use std::time::Duration;

use solana_client::client_error::{ClientError, ClientErrorKind};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signature::Signature;
use solana_transaction_status::{EncodedConfirmedTransactionWithStatusMeta, UiTransactionEncoding};

pub async fn get_transaction_with_retries(
    signature: &String,
) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
    let rpc_client = RpcClient::new(
        "".to_string(),
    );

    const MAX_RETRIES: usize = 30;
    let mut delay = 10;

    for _ in 0..MAX_RETRIES {
        let transaction = rpc_client
            .get_transaction(
                &Signature::from_str(&signature).expect("Invalid transaction signature"),
                UiTransactionEncoding::JsonParsed,
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
