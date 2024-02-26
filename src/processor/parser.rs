use std::cell::RefCell;
use std::fmt;
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

impl fmt::Debug for BubblegumInstruction {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        use BubblegumInstruction::*;
        match self {
            MintV1{accounts, args} => fmt.debug_struct("BubblegumInstruction::MintV1")
                .field("accounts::tree_config", &accounts.tree_config)
                .field("accounts::leaf_owner", &accounts.leaf_owner)
                .field("accounts::leaf_delegate", &accounts.leaf_delegate)
                .field("accounts::merkle_tree", &accounts.merkle_tree)
                .field("accounts::payer", &accounts.payer)
                .field("accounts::tree_creator_or_delegate", &accounts.tree_creator_or_delegate)
                .field("accounts::log_wrapper", &accounts.log_wrapper)
                .field("accounts::compression_program", &accounts.compression_program)
                .field("accounts::system_program", &accounts.system_program)
                .field("accounts::args", &args)
                .finish(),
            
            Transfer{accounts, args} => fmt.debug_struct("BubblegumInstruction::Transfer")
                .field("accounts::tree_config", &accounts.tree_config)
                .field("accounts::leaf_owner", &accounts.leaf_owner)
                .field("accounts::leaf_delegate", &accounts.leaf_delegate)
                .field("accounts::new_leaf_owner", &accounts.new_leaf_owner)
                .field("accounts::merkle_tree", &accounts.merkle_tree)
                .field("accounts::log_wrapper", &accounts.log_wrapper)
                .field("accounts::compression_program", &accounts.compression_program)
                .field("accounts::system_program", &accounts.system_program)
                .field("args", &args)
                .finish(),
            
            Burn{accounts, args} => fmt.debug_struct("BubblegumInstruction::Burn")
                .field("accounts::tree_config", &accounts.tree_config)
                .field("accounts::leaf_owner", &accounts.leaf_owner)
                .field("accounts::leaf_delegate", &accounts.leaf_delegate)
                .field("accounts::merkle_tree", &accounts.merkle_tree)
                .field("accounts::log_wrapper", &accounts.log_wrapper)
                .field("accounts::compression_program", &accounts.compression_program)
                .field("accounts::system_program", &accounts.system_program)
                .field("args", &args)
                .finish(),
            
            MintToCollectionV1{accounts, args} => fmt.debug_struct("BubblegumInstruction::MintToCollectionV1")
                .field("accounts::tree_config", &accounts.tree_config)
                .field("accounts::leaf_owner", &accounts.leaf_owner)
                .field("accounts::leaf_delegate", &accounts.leaf_delegate)
                .field("accounts::merkle_tree", &accounts.merkle_tree)
                .field("accounts::payer", &accounts.payer)
                .field("accounts::tree_creator_or_delegate", &accounts.tree_creator_or_delegate)
                .field("accounts::collection_authority", &accounts.collection_authority)
                .field("accounts::collection_authority_record_pda", &accounts.collection_authority_record_pda)
                .field("accounts::collection_mint", &accounts.collection_mint)
                .field("accounts::collection_metadata", &accounts.collection_metadata)
                .field("accounts::collection_edition", &accounts.collection_edition)
                .field("accounts::bubblegum_signer", &accounts.bubblegum_signer)
                .field("accounts::log_wrapper", &accounts.log_wrapper)
                .field("accounts::compression_program", &accounts.compression_program)
                .field("accounts::token_metadata_program", &accounts.token_metadata_program)
                .field("accounts::system_program", &accounts.system_program)
                .field("args", &args)
                .finish(),
        }
    }
}