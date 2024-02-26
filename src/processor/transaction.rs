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
use std::str::FromStr;

use crate::processor::parser::BubblegumInstruction;
use plerkle_serialization::{CompiledInstruction, Pubkey as PlerklePubKey, TransactionInfo};
use solana_sdk::instruction::AccountMeta;
use solana_sdk::message::MessageHeader;
use solana_transaction_status::parse_instruction::parse;

use mpl_bubblegum::ID as MPL_BUBBLEGUM_ID;

const MPL_TOKEN_METADATA_ID: Pubkey = pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
const SPL_NOOP_ID: Pubkey = pubkey!("noopb9bkMVfRPU8AsbpTUg8AQkHtKwMYZiFUjNRtMmV");

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

    instructions.iter().for_each(|instruction| {
        match account_keys[instruction.program_id_index as usize] {
            MPL_BUBBLEGUM_ID => {
                println!("bubblegum instruction");

                let MessageHeader {
                    num_required_signatures,
                    num_readonly_signed_accounts,
                    num_readonly_unsigned_accounts,
                } = message.header();

                let parsed = BubblegumInstruction::parse(
                    instruction
                        .accounts
                        .iter()
                        .map(|index| AccountMeta {
                            pubkey: account_keys[*index as usize],
                            is_signer: index < num_required_signatures,
                            is_writable: if index < num_required_signatures {
                                index < &(num_required_signatures - num_readonly_signed_accounts)
                            } else {
                                index < &(account_keys.len() as u8 - num_readonly_unsigned_accounts)
                            },
                        })
                        .collect::<Vec<_>>()
                        .as_slice(),
                    instruction.clone().data.as_slice(),
                );
                
                println!("PARSED TX");

                match parsed {
                    BubblegumInstruction::MintV1 { accounts, args } => {
                        println!("BUBBLEGUM");
                        println!("{:#?}", accounts.merkle_tree);
                        println!("{:#?}", args);
                    }
                    BubblegumInstruction::Transfer { accounts, args } => {
                        println!("{:#?}", accounts.merkle_tree);
                        println!("{:#?}", args);
                    }
                    BubblegumInstruction::Burn { accounts, args } => {
                        println!("{:#?}", accounts.merkle_tree);
                        println!("{:#?}", args);
                    }
                    BubblegumInstruction::MintToCollectionV1 { accounts, args } => {
                        println!("{:#?}", accounts.merkle_tree);
                        println!("{:#?}", args);
                    }
                }

                // println!("{parsed:#?}");
            }
            SPL_NOOP_ID => {
                println!("noop instruction")
            }
            _ => {
                println!("unknown instruction")
            }
        }
    });
}
