use soroban_sdk::{contracttype, Address, String};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const LIFETIME_THRESHOLD: u32 = BUMP_AMOUNT - DAY_IN_LEDGERS;

// #[derive(Clone)]
// #[contracttype]
// pub struct UserPoints {
//     pub has_set_signer: bool,
//     pub has_set_allowance: bool,
//     pub has_received: bool,
//     pub has_sent: bool,
//     pub points: u32,
// }

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    UserSmartId,
    AccountOwnerMap(Address),
    AccountProfileMap(String),
    IsAccount(Address),
    PlatformLink(u32),
    Excecutors(u32),
    ExcecutorCount,
    Quests(Address),

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
