use std::cell::RefCell;
use borsh::BorshDeserialize;
use mpl_bubblegum::{
    get_instruction_type,
    instructions::{
        Burn, BurnInstructionArgs, MintToCollectionV1, MintToCollectionV1InstructionArgs, MintV1,
        MintV1InstructionArgs, Transfer, TransferInstructionArgs,
    },
    InstructionName, ID as MPL_BUBBLEGUM_ID,
};
use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey;

// #[derive(Clone)]
pub enum BubblegumInstruction {
    MintV1 {
        accounts: MintV1,
        args: MintV1InstructionArgs,
    },
    Transfer {
        accounts: Transfer,
        args: TransferInstructionArgs,
    },
    Burn {
        accounts: Burn,
        args: BurnInstructionArgs,
    },
    MintToCollectionV1 {
        accounts: MintToCollectionV1,
        args: MintToCollectionV1InstructionArgs,
    },
}

impl BubblegumInstruction {
    pub fn parse(account_metas: &[AccountMeta], data: &[u8]) -> BubblegumInstruction {
        let accounts_iter = RefCell::new(account_metas.into_iter());
        let next_account_meta = || {
            accounts_iter
                .borrow_mut()
                .next()
                .expect("incorrect number of accounts for instruction")
        };
        let next_account = || next_account_meta().clone().pubkey.clone();
        let next_account_option = || match next_account() {
            MPL_BUBBLEGUM_ID => None,
            key => Some(key),
        };
        let next_account_with_signer = || {
            let &AccountMeta {
                pubkey, is_signer, ..
            } = next_account_meta();
            (pubkey, is_signer)
        };

        let mut arg_data = &data[8..];

        match get_instruction_type(data) {
            InstructionName::MintV1 => BubblegumInstruction::MintV1 {
                accounts: MintV1 {
                    tree_config: next_account(),
                    leaf_owner: next_account(),
                    leaf_delegate: next_account(),
                    merkle_tree: next_account(),
                    payer: next_account(),
                    tree_creator_or_delegate: next_account(),
                    log_wrapper: next_account(),
                    compression_program: next_account(),
                    system_program: next_account(),
                },
                args: MintV1InstructionArgs::deserialize(&mut arg_data)
                    .expect("could not parse args data"),
            },
            InstructionName::Transfer => BubblegumInstruction::Transfer {
                accounts: Transfer {
                    tree_config: next_account(),
                    leaf_owner: next_account_with_signer(),
                    leaf_delegate: next_account_with_signer(),
                    new_leaf_owner: next_account(),
                    merkle_tree: next_account(),
                    log_wrapper: next_account(),
                    compression_program: next_account(),
                    system_program: next_account(),
                },
                args: TransferInstructionArgs::deserialize(&mut arg_data)
                    .expect("could not parse args data"),
            },
            InstructionName::Burn => BubblegumInstruction::Burn {
                accounts: Burn {
                    tree_config: next_account(),
                    leaf_owner: next_account_with_signer(),
                    leaf_delegate: next_account_with_signer(),
                    merkle_tree: next_account(),
                    log_wrapper: next_account(),
                    compression_program: next_account(),
                    system_program: next_account(),
                },
                args: BurnInstructionArgs::deserialize(&mut arg_data)
                    .expect("could not parse args data"),
            },
            InstructionName::MintToCollectionV1 => BubblegumInstruction::MintToCollectionV1 {
                accounts: MintToCollectionV1 {
                    tree_config: next_account(),
                    leaf_owner: next_account(),
                    leaf_delegate: next_account(),
                    merkle_tree: next_account(),
                    payer: next_account(),
                    tree_creator_or_delegate: next_account(),
                    collection_authority: next_account(),
                    collection_authority_record_pda: next_account_option(),
                    collection_mint: next_account(),
                    collection_metadata: next_account(),
                    collection_edition: next_account(),
                    bubblegum_signer: next_account(),
                    log_wrapper: next_account(),
                    compression_program: next_account(),
                    token_metadata_program: next_account(),
                    system_program: next_account(),
                },
                args: MintToCollectionV1InstructionArgs::deserialize(&mut arg_data)
                    .expect("could not parse args data"),
            },

            _ => panic!("unknown instruction"),
        }
    }
}
