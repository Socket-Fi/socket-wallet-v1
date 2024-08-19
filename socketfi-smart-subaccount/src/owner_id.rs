use soroban_sdk::{Address, Env, String};

use crate::data::{DataKey, SmartID};
//Wallet smart ID

pub fn has_id(e: &Env) -> bool {
    let key = DataKey::SmartIDs;
    e.storage().instance().has(&key)
}

pub fn read_id(e: &Env) -> SmartID {
    let key = DataKey::SmartIDs;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_id(
    e: &Env,
    platform: String,
    profile_id: String,
    encrypted_token: String,
    encrypted_index: String,
) {
    let key = DataKey::SmartIDs;
    let smart_id = SmartID {
        platform,
        user_id: profile_id,
        user_token_encrypted: encrypted_token,
        index_encrypted: encrypted_index,
    };
    e.storage().instance().set(&key, &smart_id);
}
