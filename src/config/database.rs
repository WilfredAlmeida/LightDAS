use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, PgPool,
};
use crate::config::env_config::EnvConfig;

const BARE_MINIMUM_CONNECTIONS: u32 = 5;
const DEFAULT_MAX: u32 = 125;
pub async fn setup_database_config(config: &EnvConfig) -> PgPool {
    let url = config.get_database_url();
    let mut options: PgConnectOptions = url.parse().unwrap();
    options.log_statements(log::LevelFilter::Trace);

    options.log_slow_statements(
        log::LevelFilter::Debug,
        std::time::Duration::from_millis(500),
    );

    PgPoolOptions::new()
        .min_connections(BARE_MINIMUM_CONNECTIONS)
        .max_connections(DEFAULT_MAX)
        .connect_with(options)
        .await
        .expect("Failed to setup database pool")
}
