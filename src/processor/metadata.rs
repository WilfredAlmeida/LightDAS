use std::time::Duration;

use digital_asset_types::{dao::asset_data, json};
use reqwest::{Client, ClientBuilder};
use sea_orm::{
    ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect, Set,
    Unchanged,
};

pub async fn fetch_store_metadata() -> Result<bool, reqwest::Error> {
    println!("Fetching Metadata");
    let database: DatabaseConnection =
        Database::connect("postgres://solana:solana@localhost:5432/solana")
            .await
            .unwrap();

    let assets = asset_data::Entity::find()
        .columns([asset_data::Column::Id, asset_data::Column::MetadataUrl])
        .filter(asset_data::Column::Reindex.eq(true))
        .all(&database)
        .await
        .unwrap();

    for asset in assets {
        let asset_id = &asset.id;

        let metadata_url = &asset.metadata_url;

        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(4))
            .build()?;

        let response = Client::get(&client, metadata_url).send().await?;

        if response.status() != reqwest::StatusCode::OK {
            println!("Download Metadata Error");
            return Ok(false);
        } else {
            let val: serde_json::Value = response.json().await?;

            let model = asset_data::ActiveModel {
                id: Unchanged(asset_id.clone()),
                metadata: Set(val),
                reindex: Set(Some(false)),
                ..Default::default()
            };

            asset_data::Entity::update(model)
                .filter(asset_data::Column::Id.eq(asset.id))
                .exec(&database)
                .await
                .unwrap();

            println!("Metadata Updated")
        }
    }

    Ok(true)
}
