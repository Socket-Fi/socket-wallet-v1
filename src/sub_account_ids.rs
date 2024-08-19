use soroban_sdk::{Address, Env, String};

use crate::data::DataKey;

pub fn read_account_addr(e: &Env, owner: Address) -> Option<Address> {
    let key = DataKey::AccountOwnerMap(owner);
    if let Some(account) = e.storage().instance().get(&key).unwrap() {
        account
    } else {
        None
    }
}

pub fn write_account_addr(e: &Env, owner: Address, account_id: Address) {
    let key = DataKey::AccountOwnerMap(owner);
    e.storage().instance().set(&key, &account_id);
}

pub fn read_account_profile(e: &Env, profile_token: String) -> Option<Address> {
    let key = DataKey::AccountProfileMap(profile_token);
    if let Some(account) = e.storage().instance().get(&key).unwrap() {
        account
    } else {
        None
    }
}

pub fn write_account_profile(e: &Env, profile_token: String, account_id: Address) {
    let key = DataKey::AccountProfileMap(profile_token);
    e.storage().instance().set(&key, &account_id);
}
