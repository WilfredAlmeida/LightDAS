use blockbuster::{instruction::InstructionBundle, programs::bubblegum::BubblegumInstruction};
use mpl_bubblegum::instructions::{MintToCollectionV1, MintToCollectionV1InstructionArgs};
use sqlx::{Pool, Postgres};

use crate::config::database;

pub async fn handle_mint_to_collection_v1_instruction(
    accounts: MintToCollectionV1,
    args: MintToCollectionV1InstructionArgs,
    database_pool: Pool<Postgres>,
) {
}
