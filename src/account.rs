use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, String};

use crate::{
    access::{read_administrator, write_administrator},
    sub_account::{self, create_with_addr, create_with_user_id},
    sub_account_ids::{
        read_account_addr, read_account_profile, write_account_addr, write_account_profile,
    },
};

pub trait SmartWalletTrait {
    fn init(e: Env, admin: Address);
    fn create_account_with_addr(
        e: Env,
        owner: Address,
        subaccount_wasm_hash: BytesN<32>,
    ) -> Address;
    fn create_account_with_id(
        e: Env,
        platform: String,
        profile_id: String,
        owner_token: String,
        token_encryption_index: String,
        owner_passkey: String,
        passkey_encryption_index: String,
        max_allowance: u32,
        subaccount_wasm_hash: BytesN<32>,
    ) -> Address;
    fn get_account_addr(e: Env, invoker: Address) -> Option<Address>;
    fn get_account_profile(e: Env, owner_token: String) -> Option<Address>;
    // fn send_with_addr(e: Env, invoker: Address, to: Address, token_id: Address, amount: i128);
    fn upgrade(e: Env, invoker: Address, new_wasm_hash: BytesN<32>);
}

#[contract]
pub struct SmartWallet;

#[contractimpl]
impl SmartWalletTrait for SmartWallet {
    fn init(e: Env, admin: Address) {
        write_administrator(&e, &admin)
    }

    fn create_account_with_addr(
        e: Env,
        owner: Address,
        subaccount_wasm_hash: BytesN<32>,
    ) -> Address {
        let account_id = create_with_addr(&e, &owner, subaccount_wasm_hash);
        let account_contract = sub_account::Client::new(&e, &account_id);
        account_contract.init_with_address(&e.current_contract_address(), &owner);
        write_account_addr(&e, owner, account_id.clone());
        account_id
    }

    fn create_account_with_id(
        e: Env,
        platform: String,
        profile_id: String,
        owner_token: String,
        token_encryption_index: String,
        owner_passkey: String,
        passkey_encryption_index: String,
        max_allowance: u32,
        subaccount_wasm_hash: BytesN<32>,
    ) -> Address {
        let account_id = create_with_user_id(
            &e,
            owner_token.clone(),
            owner_passkey.clone(),
            subaccount_wasm_hash,
        );
        let account_contract = sub_account::Client::new(&e, &account_id);

        account_contract.init_with_profile(
            &e.current_contract_address(),
            &platform,
            &profile_id,
            &owner_token,
            &token_encryption_index,
            &passkey_encryption_index,
            &owner_passkey,
            &max_allowance,
        );
        write_account_profile(&e, owner_token, account_id.clone());
        account_id
    }

    fn get_account_addr(e: Env, invoker: Address) -> Option<Address> {
        read_account_addr(&e, invoker)
    }

    fn get_account_profile(e: Env, owner_token: String) -> Option<Address> {
        read_account_profile(&e, owner_token)
    }

    fn upgrade(e: Env, invoker: Address, new_wasm_hash: BytesN<32>) {
        let admin = read_administrator(&e);
        if invoker != admin {
            panic!("Not authorized to invoke this function")
        }
        e.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}
