#![allow(unused)]
use soroban_sdk::{xdr::ToXdr, Address, Bytes, BytesN, Env, String};

soroban_sdk::contractimport!(
    file = "./socketfi-smart-subaccount/target/wasm32-unknown-unknown/release/socketfi_smart_subaccount.wasm"
);

pub fn create_with_addr(e: &Env, caller: &Address, subaccount_wasm_hash: BytesN<32>) -> Address {
    let mut salt = Bytes::new(e);
    salt.append(&caller.to_xdr(e));
    let salt = e.crypto().sha256(&salt);
    e.deployer()
        .with_current_contract(salt)
        .deploy(subaccount_wasm_hash)
}

pub fn create_with_user_id(
    e: &Env,
    user_token: String,
    user_passkey: String,
    subaccount_wasm_hash: BytesN<32>,
) -> Address {
    let mut salt = Bytes::new(e);
    salt.append(&user_token.to_xdr(e));
    salt.append(&user_passkey.to_xdr(e));
    let salt = e.crypto().sha256(&salt);
    e.deployer()
        .with_current_contract(salt)
        .deploy(subaccount_wasm_hash)
}
