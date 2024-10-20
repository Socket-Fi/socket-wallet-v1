#![allow(unused)]
use crate::types::UserPoints;
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

pub fn create_with_pkey(
    e: &Env,
    profile_id: String,
    passkey_hash: String,
    subaccount_wasm_hash: BytesN<32>,
) -> Address {
    let mut salt = Bytes::new(e);
    salt.append(&profile_id.to_xdr(e));
    salt.append(&passkey_hash.to_xdr(e));
    let salt = e.crypto().sha256(&salt);
    e.deployer()
        .with_current_contract(salt)
        .deploy(subaccount_wasm_hash)
}
