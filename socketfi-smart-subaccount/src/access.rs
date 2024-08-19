use soroban_sdk::{Address, Env, String};

use crate::data::{DataKey, EncryptedPassKey};
//Smart wallet sub account owner
pub fn read_is_initialized(e: &Env) -> bool {
    let key = DataKey::Initialized;
    if let Some(flag) = e.storage().instance().get(&key) {
        flag
    } else {
        false
    }
}

pub fn write_is_initialized(e: &Env) {
    let key = DataKey::Initialized;
    e.storage().instance().set(&key, &true);
}

pub fn has_owner(e: &Env) -> bool {
    let key = DataKey::Owner;
    e.storage().instance().has(&key)
}

pub fn read_owner(e: &Env) -> Address {
    let key = DataKey::Owner;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_owner(e: &Env, owner_id: &Address) {
    let key = DataKey::Owner;
    e.storage().instance().set(&key, owner_id);
}

pub fn read_passkey(e: &Env) -> EncryptedPassKey {
    let key = DataKey::PassKey;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_passkey(e: &Env, index_encrypted: String, passkey_encrypted: String) {
    let key = DataKey::PassKey;
    let user_key = EncryptedPassKey {
        index: index_encrypted,
        key: passkey_encrypted,
    };
    e.storage().instance().set(&key, &user_key);
}

pub fn read_max_allowance(e: &Env) -> u32 {
    let key = DataKey::MaxAllowance;
    if let Some(allowance) = e.storage().instance().get(&key).unwrap() {
        allowance
    } else {
        0
    }
}

pub fn write_max_allowance(e: &Env, allowance: u32) {
    let key = DataKey::MaxAllowance;
    e.storage().instance().set(&key, &allowance)
}

//Smart wallet logic contract

pub fn read_controller(e: &Env) -> Address {
    let key = DataKey::Controller;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_controller(e: &Env, controller_id: &Address) {
    let key = DataKey::Controller;
    e.storage().instance().set(&key, controller_id);
}
