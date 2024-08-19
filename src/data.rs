use soroban_sdk::{contracttype, Address, String};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    UserSmartId,
    AccountOwnerMap(Address),
    AccountProfileMap(String),
    PlatformLink(u32),
    UserPlatformId(u32),
    UserSmartWalletID(String),
    UserSmartWallet(u32),
    AuthorizedExecutor,
}

//Stellar account pubkey: 0
// Email: 1
// X (Twitter): 2
// Discord: 3
// Telegram: 4
