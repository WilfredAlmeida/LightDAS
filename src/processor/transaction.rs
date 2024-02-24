use blockbuster::error::BlockbusterError;
use blockbuster::token_metadata::{accounts, ID};
use blockbuster::{
    instruction::InstructionBundle,
    program_handler::ProgramParser,
    programs::{bubblegum::BubblegumParser, ProgramParseResult},
};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use solana_transaction_status::option_serializer::OptionSerializer;
use solana_transaction_status::{EncodedConfirmedTransactionWithStatusMeta, UiInnerInstructions};
use std::str::FromStr;
// use solana_sdk::pubkey::Pubkey;

use plerkle_serialization::{CompiledInstruction, Pubkey as PlerklePubKey, TransactionInfo};

pub fn process_transaction(transaction: EncodedConfirmedTransactionWithStatusMeta) {

    let bubblegum_parser = BubblegumParser {};

    let inner_instructions: Option<Vec<UiInnerInstructions>> = transaction
        .transaction
        .meta
        .to_owned()
        .unwrap()
        .inner_instructions
        .into();
    let tsx = transaction.transaction.transaction;
    let unwrapped_transaction = tsx.decode().unwrap();
    let message = unwrapped_transaction.message;
    let account_keys = message.static_account_keys();
    let instructions = message.instructions();

    let spl_noop_id: String = String::from("noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV");

    instructions.iter().for_each(|instruction| {
        println!("{:#?}", account_keys[instruction.program_id_index as usize].to_string());
        match account_keys[instruction.program_id_index as usize].to_string() {
            MPL_TOKEN_METADATA_ID => {
                println!("bubblegum instruction");
                
            }
            spl_noop_id => {
                println!("noop instruction")
            }
            _ => {
                println!("unknown instruction")
            }
        }
    });

}
