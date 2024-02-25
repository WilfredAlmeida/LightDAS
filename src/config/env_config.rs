use std::env;

pub struct EnvConfig{
    rpc_url: String,
    websocket_url: String,
    database_url: String,
}

impl EnvConfig {
    pub fn get_websocket_url(&self) -> &String {
        &self.websocket_url
    }

    pub fn get_rpc_url(&self) -> &String {
        &self.rpc_url
    }

    pub fn get_database_url(&self) -> &String {
        &self.database_url
    }
}

pub fn setup_env_config() -> EnvConfig {
    let env_ws_url = env::var("WS_URL").expect("WS_URL not found");
    let env_rpc_url = env::var("RPC_URL").expect("RPC_URL not found");
    let env_db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found");

    EnvConfig {
        websocket_url: env_ws_url,
        rpc_url: env_rpc_url,
        database_url: env_db_url
    }
}
