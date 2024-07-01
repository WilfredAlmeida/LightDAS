use std::panic;
use std::str::FromStr;

use anyhow::Error;
use solana_sdk::{instruction::CompiledInstruction, pubkey::Pubkey};
use solana_transaction_status::{
    EncodedConfirmedTransactionWithStatusMeta, InnerInstruction, InnerInstructions,
    UiInnerInstructions, UiInstruction, UiLoadedAddresses,
};

use program_transformers::{ProgramTransformer, TransactionInfo};

pub async fn process_transaction(
    program_transformer: &ProgramTransformer,
    transaction: EncodedConfirmedTransactionWithStatusMeta,
) -> Result<(), Error> {
    let meta = transaction
        .transaction
        .meta
        .to_owned()
        .expect("transaction does not have meta");
    let inner_instructions: Option<Vec<UiInnerInstructions>> = meta.inner_instructions.into();
    let tsx = transaction.transaction.transaction;
    let unwrapped_transaction = tsx.decode().unwrap();
    let message = unwrapped_transaction.message;

    let loaded_addresses =
        Into::<Option<UiLoadedAddresses>>::into(meta.loaded_addresses).unwrap_or_default();
    let mut account_keys = Vec::from(message.static_account_keys());
    account_keys.extend(
        loaded_addresses
            .writable
            .into_iter()
            .chain(loaded_addresses.readonly.into_iter())
            .map(|key| {
                Pubkey::from_str(&key).map_err(|e| format!("could not parse pubkey: {key}: {e}"))
            })
            .collect::<Result<Vec<_>, _>>()
            .expect("could not read additional addresses"),
    );

    let res = program_transformer
        .handle_transaction(&TransactionInfo {
            slot: transaction.slot,
            signature: unwrapped_transaction.signatures[0],
            account_keys: account_keys,
            message_instructions: message.instructions().into(),
            meta_inner_instructions: inner_instructions
                .unwrap_or_default()
                .into_iter()
                .map(|i| InnerInstructions {
                    index: i.index,
                    instructions: i
                        .instructions
                        .into_iter()
                        .map(|ix| match ix {
                            UiInstruction::Compiled(instruction) => InnerInstruction {
                                instruction: CompiledInstruction {
                                    program_id_index: instruction.program_id_index,
                                    accounts: instruction.accounts,
                                    data: bs58::decode(&instruction.data).into_vec().unwrap(),
                                },
                                stack_height: instruction.stack_height,
                            },
                            _ => panic!("Not compiled"),
                        })
                        .collect(),
                })
                .collect::<Vec<_>>(),
        })
        .await;

    if let Err(e) = res {
        eprintln!("tx handling error: {:?}", e);
        return Ok(());
    }

    Ok(())
}
