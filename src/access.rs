use soroban_sdk::{xdr::ToXdr, Address, Bytes, Env, String};

use crate::data::{DataKey, BUMP_AMOUNT, LIFETIME_THRESHOLD};
use crate::sub_account;
use crate::sub_account_ids::read_account_profile;
//Admin data stored at index 0
pub fn has_owner(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().instance().has(&key)
}

pub fn read_administrator(e: &Env) -> Address {
    let key = DataKey::Admin;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_administrator(e: &Env, id: &Address) {
    let key = DataKey::Admin;
    e.storage().instance().set(&key, id);
}

pub fn read_excecutor_count(e: &Env) -> u32 {
    let key = DataKey::ExcecutorCount;
    if let Some(count) = e.storage().persistent().get::<DataKey, u32>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
        count
    } else {
        0
    }
}

pub fn write_excecutor_count(e: &Env, new_count: u32) {
    let key = DataKey::ExcecutorCount;
    e.storage().persistent().set(&key, &new_count);
    e.storage()
        .persistent()
        .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
}

pub fn write_executor(e: &Env, index: u32, executor_pub_key: Address) {
    let key = DataKey::Excecutors(index);
    e.storage().instance().set(&key, &executor_pub_key);
    write_excecutor_count(e, index);
}

pub fn read_executor(e: &Env, index: u32) -> Address {
    let key = DataKey::Excecutors(index);
    e.storage().instance().get(&key).unwrap()
}

pub fn validate_tx(
    e: &Env,
    executor_index: u32,
    profile_id: String,
    entered_passkey_hash: String,
    spender: Address,
    token_id: Address,
    amount: i128,
) -> bool {
    let executor = read_executor(e, executor_index);
    let account_id = read_account_profile(e, profile_id).unwrap();
    let account_contract = sub_account::Client::new(&e, &account_id);
    let signed_nonce = account_contract.get_tx_nonce();
    let seq_nonce = account_contract.get_nonce();

    let mut salt = Bytes::new(&e);
    salt.append(&executor.to_xdr(&e));
    salt.append(&seq_nonce.to_xdr(&e));
    salt.append(&entered_passkey_hash.to_xdr(&e));
    salt.append(&spender.to_xdr(&e));
    salt.append(&token_id.to_xdr(&e));
    salt.append(&amount.to_xdr(&e));
    let created_nonce = e.crypto().sha256(&salt).to_xdr(&e);
    let validation_status = signed_nonce == created_nonce;
    if validation_status == true {
        account_contract.clear_tx_nonce();
    }

    validation_status
}
