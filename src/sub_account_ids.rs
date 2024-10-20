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
    write_is_account(e, account_id);
}

pub fn has_account_profile(e: &Env, profile_id: String) -> bool {
    let key = DataKey::AccountProfileMap(profile_id);
    e.storage().instance().has(&key)
}

pub fn read_account_profile(e: &Env, profile_id: String) -> Option<Address> {
    let key = DataKey::AccountProfileMap(profile_id);
    if let Some(account) = e.storage().instance().get(&key).unwrap() {
        account
    } else {
        None
    }
}

pub fn write_account_profile(e: &Env, profile_id: String, account_id: Address) {
    let key = DataKey::AccountProfileMap(profile_id);
    e.storage().instance().set(&key, &account_id);
    write_is_account(e, account_id);
}

pub fn write_is_account(e: &Env, contract_id: Address) {
    let key = DataKey::IsAccount(contract_id);
    e.storage().instance().set(&key, &true);
}

pub fn read_is_account(e: &Env, contract_id: Address) -> bool {
    let key = DataKey::IsAccount(contract_id);
    e.storage().instance().get(&key).unwrap_or(false)
}

pub fn read_account(e: &Env, owner_id: Option<Address>, owner_token: Option<String>) -> Address {
    let user_acc = if let Some(uid) = owner_id {
        read_account_addr(&e, uid)
    } else if let Some(token) = owner_token {
        read_account_profile(&e, token)
    } else {
        panic!("Neither a valid owner_id not owner_token was provided");
    }
    .unwrap();

    user_acc
}
