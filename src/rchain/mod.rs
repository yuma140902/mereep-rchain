use std::{collections::HashMap, time::SystemTime};

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub accounts: HashMap<String, Account>,
    pending_transactions: Vec<Transaction>,
}

pub trait WorldState {
    fn get_user_ids(&self) -> Vec<String>;
    fn get_account_by_id_mut(&self, id: &str) -> Option<&mut Account>;
    fn get_account_by_id(&self, id: &str) -> Option<&Account>;
    fn create_account(&mut self, id: String, account_type: AccountType)
        -> Result<(), &'static str>;
}

#[derive(Debug, Clone)]
pub struct Block {
    pub(crate) transactions: Vec<Transaction>,
    prev_hash: Option<String>,
    hash: Option<String>,
    nonce: u128,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    nonce: u128,
    from: String,
    created_at: SystemTime,
    pub(crate) record: TransactionData,
    signature: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionData {
    CreateUserAccount(String),
    ChangeStoreValue { key: String, value: String },
    TransferTokens { to: String, amount: u128 },
    CreateTokens { receiver: String, amount: u128 },
}

#[derive(Debug, Clone)]
pub struct Account {
    store: HashMap<String, String>,
    acc_type: AccountType,
    tokens: u128,
}

#[derive(Debug, Clone)]
pub enum AccountType {
    User,
    Contract,
}
