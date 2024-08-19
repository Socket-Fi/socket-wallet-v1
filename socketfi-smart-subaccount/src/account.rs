use soroban_sdk::{
    contract, contractimpl,
    crypto::Hash,
    xdr::{FromXdr, MuxedAccount, ScAddress, ToXdr, Uint256},
    Address, Bytes, BytesN, Env, String, Vec,
};

use crate::{
    access::{
        has_owner, read_controller, read_is_initialized, read_owner, read_passkey,
        write_controller, write_is_initialized, write_max_allowance, write_owner, write_passkey,
    },
    owner_id::{read_id, write_id},
    transact::{send_token, take_token},
};

use crate::concat::strings_concatenator;

pub trait SubAccountTrait {
    fn init_with_address(e: Env, controller_id: Address, owner_id: Address);
    fn init_with_profile(
        e: Env,
        controller_id: Address,
        platform: String,
        profile_id: String,
        encrypted_token: String,
        encrypted_index: String,
        passkey_index_encrypted: String,
        passkey_encrypted: String,
        max_allowance: u32,
    );
    fn set_allowance_with_addr(e: Env, caller: Address, allowance: u32);
    fn set_allowance_with_id(e: Env, user_token: String, passkey: String, allowance: u32);
    fn receive(e: Env, from: Address, token_id: Address, amount: i128);
    fn send_auth_addr(e: Env, invoker: Address, to: Address, token_id: Address, amount: i128);
    fn send_auth_passkey(
        e: Env,
        user_token: String,
        user_passkey: String,
        to: Address,
        token_id: Address,
        amount: i128,
    );
    fn upgrade(e: Env, new_wasm_hash: BytesN<32>);
}

#[contract]
pub struct SubAccount;

#[contractimpl]
impl SubAccountTrait for SubAccount {
    fn init_with_address(e: Env, controller_id: Address, owner_id: Address) {
        let is_initialized = read_is_initialized(&e);

        if is_initialized {
            panic!("has already been initilized")
        }
        write_owner(&e, &owner_id);
        write_controller(&e, &controller_id);
        write_is_initialized(&e)
    }

    fn init_with_profile(
        e: Env,
        controller_id: Address,
        platform: String,
        profile_id: String,
        encrypted_token: String,
        encrypted_index: String,
        passkey_index_encrypted: String,
        passkey_encrypted: String,
        max_allowance: u32,
    ) {
        let is_initialized = read_is_initialized(&e);

        if is_initialized {
            panic!("has already been initilized")
        }
        write_id(&e, platform, profile_id, encrypted_token, encrypted_index);
        write_controller(&e, &controller_id);
        write_passkey(&e, passkey_index_encrypted, passkey_encrypted);
        write_max_allowance(&e, max_allowance);
        write_is_initialized(&e)
    }

    fn set_allowance_with_addr(e: Env, caller: Address, allowance: u32) {
        if caller != read_owner(&e) {
            panic!("You are unauthorized")
        }
        write_max_allowance(&e, allowance);
    }

    fn set_allowance_with_id(e: Env, user_token: String, user_passkey: String, allowance: u32) {
        let smart_id = read_id(&e);
        let passkey = read_passkey(&e);
        if user_token != smart_id.user_token_encrypted || user_passkey != passkey.key {
            panic!("You are unauthorized")
        }
        write_max_allowance(&e, allowance);
    }

    fn receive(e: Env, from: Address, token_id: Address, amount: i128) {
        from.require_auth();
        take_token(&e, &from, &token_id, amount);
    }

    fn send_auth_addr(e: Env, invoker: Address, to: Address, token_id: Address, amount: i128) {
        let owner = read_owner(&e);
        let controller_contract = read_controller(&e);
        if invoker != owner && invoker != controller_contract {
            panic!("Unauthorized: Only the owner or controller can call this function")
        }
        send_token(&e, &to, &token_id, amount);
    }

    fn send_auth_passkey(
        e: Env,
        user_token: String,
        user_passkey: String,
        to: Address,
        token_id: Address,
        amount: i128,
    ) {
        let smart_id = read_id(&e);
        let passkey = read_passkey(&e);
        if user_token != smart_id.user_token_encrypted || user_passkey != passkey.key {
            panic!("Unauthorized: Only the owner or controller can call this function")
        }

        send_token(&e, &to, &token_id, amount);
    }

    fn upgrade(e: Env, new_wasm_hash: BytesN<32>) {
        e.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}
