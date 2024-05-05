use lazy_static::lazy_static;
use std::sync::Mutex;
use std::{borrow::BorrowMut, collections::VecDeque, sync::OnceLock};

#[derive(Clone, Debug)]
pub struct TransactionsQueue {
    pub transaction_signature: String,
    pub tree_address: Option<String>,
}

lazy_static! {
    static ref TRANSACTIONS_QUEUE: Mutex<VecDeque<TransactionsQueue>> = Mutex::new(VecDeque::new());
}

pub fn push_front(transaction: TransactionsQueue) {
    TRANSACTIONS_QUEUE.lock().unwrap().push_front(transaction);
}

pub fn pop_front() -> Option<TransactionsQueue> {
    TRANSACTIONS_QUEUE.lock().unwrap().pop_front()
}

pub fn pop_back() -> Option<TransactionsQueue> {
    TRANSACTIONS_QUEUE.lock().unwrap().pop_back()
}

pub fn push_back(transaction: TransactionsQueue) {
    TRANSACTIONS_QUEUE.lock().unwrap().push_back(transaction)
}
