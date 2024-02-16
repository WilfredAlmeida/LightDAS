use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;

pub fn process_transaction(transaction: EncodedConfirmedTransactionWithStatusMeta) {
    println!(
        "meta: {:?}",
        transaction.transaction.meta.unwrap().inner_instructions
    );
    println!(
        "encoded transaction: {:?}",
        transaction.transaction.transaction
    );
}
