use solana_transaction_status::{
    EncodedConfirmedTransactionWithStatusMeta, InnerInstruction, InnerInstructions,
    UiInnerInstructions, UiInstruction,
};

use std::panic;

use program_transformers::{ProgramTransformer, TransactionInfo};

use solana_sdk::instruction::CompiledInstruction;

pub async fn process_transaction(
    program_transformer: &ProgramTransformer,
    transaction: EncodedConfirmedTransactionWithStatusMeta,
) {
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

    let res = program_transformer
        .handle_transaction(&TransactionInfo {
            slot: transaction.slot,
            signature: &unwrapped_transaction.signatures[0],
            account_keys: &message.static_account_keys(),
            message_instructions: &message.instructions(),
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
                .collect::<Vec<_>>()
                .as_slice(),
        })
        .await
        .unwrap();

    println!("HANDLED TX")
}
