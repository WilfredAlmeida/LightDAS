use std::env;

use solana_client::nonblocking::pubsub_client::PubsubClient;
use solana_client::nonblocking::rpc_client::RpcClient;
use std::sync::OnceLock;

pub static PUBSUB_CLIENT: OnceLock<PubsubClient> = OnceLock::new();
pub static RPC_CLIENT: OnceLock<RpcClient> = OnceLock::new();

pub async fn initialize_clients() {
    PUBSUB_CLIENT.set(
        PubsubClient::new(&env::var("WS_URL").expect("WS_URL not found"))
            .await
            .unwrap()
    }).unwrap_or_else(|_| panic!("pubsub client already set"));

    RPC_CLIENT
        .set(RpcClient::new(env::var("RPC_URL").expect("RPC_URL not found")))
		.unwrap_or_else(|_| panic!("rpc client already set"));;
}

pub fn get_pubsub_client() -> &'static PubsubClient {
    PUBSUB_CLIENT.get().expect("failed to get pubsub client")
}

pub fn get_rpc_client() -> &'static RpcClient {
    RPC_CLIENT.get().expect("failed to get rpc client")
}
