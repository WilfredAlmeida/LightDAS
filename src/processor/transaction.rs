use blockbuster::error::BlockbusterError;
use blockbuster::token_metadata::{accounts, ID};
use blockbuster::{
    instruction::InstructionBundle,
    program_handler::ProgramParser,
    programs::{bubblegum::BubblegumParser, ProgramParseResult},
};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey, pubkey::Pubkey};
use solana_transaction_status::option_serializer::OptionSerializer;
use solana_transaction_status::{EncodedConfirmedTransactionWithStatusMeta, UiInnerInstructions};
use sqlx::{Pool, Postgres};
use std::panic;
use std::str::FromStr;
use tokio::task;

use program_transformers::{
    AccountInfo, DownloadMetadataNotifier, ProgramTransformer, TransactionInfo,
};

use crate::processor::handlers::mint_to_collection_v1::handle_mint_to_collection_v1_instruction;
use crate::processor::parser::BubblegumInstruction;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::message::MessageHeader;
use solana_transaction_status::parse_instruction::parse;

use mpl_bubblegum::ID as MPL_BUBBLEGUM_ID;

const MPL_TOKEN_METADATA_ID: Pubkey = pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
const SPL_NOOP_ID: Pubkey = pubkey!("noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV");

pub async fn process_transaction(
    program_transformer: &ProgramTransformer,
    transaction: EncodedConfirmedTransactionWithStatusMeta,
) {
    let bubblegum_parser = BubblegumParser {};

    let inner_instructions: Vec<UiInnerInstructions> = transaction
        .transaction
        .meta
        .to_owned()
        .unwrap()
        .inner_instructions
        .into()
        .unwrap_or(vec![]);
    let tsx = transaction.transaction.transaction;
    let unwrapped_transaction = tsx.decode().unwrap();
    let message = unwrapped_transaction.message;

    program_transformer
        .handle_transaction(TransactionInfo {
            slot: transaction.slot,
            signature: &unwrapped_transaction.signatures[0],
            account_keys: &message.static_account_keys(),
            message_instructions: &message.instructions(),
            meta_inner_instructions: &inner_instructions.into(),
        })
        .unwrap();
}
