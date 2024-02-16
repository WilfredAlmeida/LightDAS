use std::env;

use solana_client::nonblocking::pubsub_client::PubsubClient;
use solana_client::nonblocking::rpc_client::RpcClient;
use std::sync::OnceLock;

pub static PUBSUB_CLIENT: OnceLock<PubsubClient> = OnceLock::new();
pub static RPC_CLIENT: OnceLock<RpcClient> = OnceLock::new();

pub async fn initialize_clients() {
    PUBSUB_CLIENT
        .set(
            PubsubClient::new(&env::var("WS_URL").expect("WS_URL not found"))
                .await
                .unwrap(),
        )
        .expect("Failed to set pubsub client");

    RPC_CLIENT.set(RpcClient::new(
        env::var("RPC_URL").expect("RPC_URL not found"),
    ));
}

pub fn get_rpc_client() -> &'static RpcClient {
    let rpc_client = RPC_CLIENT.get();

    match rpc_client {
        Some(client) => return client,
        None => {
            panic!("Failed to get rpc client");
        }
    }
}

pub fn get_pubsub_client() -> &'static PubsubClient {
    let pubsub_client = PUBSUB_CLIENT.get();

    match pubsub_client {
        Some(client) => return client,
        None => {
            panic!("Failed to get pubsub client");
        }
    }
}
