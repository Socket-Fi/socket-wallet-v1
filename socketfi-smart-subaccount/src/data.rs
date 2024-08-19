use soroban_sdk::{contracttype, Address, Bytes, Env, String};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const BALANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;

#[derive(Clone)]
#[contracttype]
pub struct SmartID {
    pub platform: String,
    pub user_id: String,
    pub user_token_encrypted: String,
    pub index_encrypted: String,
}

#[derive(Clone)]
#[contracttype]
pub struct EncryptedPassKey {
    pub index: String,
    pub key: String,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Owner,
    PassKey,
    Controller,
    MaxAllowance,
    UserSmartId,
    UserPlatformId(u32),
    SmartIDs,
    Initialized,
}

//Stellar account pubkey: 0
// Email: 1
// X (Twitter): 2
// Discord: 3
// Telegram: 4
