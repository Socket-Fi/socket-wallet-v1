use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, String};

use crate::{
    access::{
        read_administrator, read_excecutor_count, read_executor, validate_tx, write_administrator,
        write_executor,
    },
    // data::UserPoints,
    sub_account::{self, create_with_addr, create_with_pkey},
    sub_account_ids::{
        has_account_profile, read_account, read_account_addr, read_account_profile,
        read_is_account, write_account_addr, write_account_profile,
    },
    types::UserPoints,
};

pub trait SmartWalletTrait {
    fn init(e: Env, admin: Address);
    fn create_account_addr(e: Env, owner: Address, subaccount_wasm_hash: BytesN<32>) -> Address;
    fn create_account_pkey(
        e: Env,
        executor_index: u32,
        platform: String,
        profile_id: String,
        salt: String,
        salt_iv: String,
        key_index: String,
        index_iv: String,
        passkey_hash: String,
        max_allowance: i128,
        subaccount_wasm_hash: BytesN<32>,
    ) -> Address;
    fn check_tx_validation(
        e: &Env,
        executor_index: u32,
        profile_id: String,
        entered_passkey_hash: String,
        token_id: Address,
        amount: i128,
    ) -> bool;
    fn add_executor(e: Env, executor_pub_key: Address);
    fn get_executor(e: Env, index: u32) -> Address;
    fn send_token_pkey(
        e: Env,
        executor_index: u32,
        profile_id: String,
        entered_passkey_hash: String,
        to: Address,
        token_id: Address,
        amount: i128,
    );
    // fn set_owner(
    //     e: Env,
    //     executor_index: u32,
    //     profile_id: String,
    //     entered_passkey_hash: String,
    //     owner_id: Address,
    // );
    fn get_user_points(e: Env, account_id: Address) -> UserPoints;
    fn get_account_addr(e: Env, invoker: Address) -> Option<Address>;
    fn get_account_profile(e: Env, profile_id: String) -> Option<Address>;
    fn get_user_account(e: Env, owner_id: Option<Address>, profile_id: Option<String>) -> Address;
    // fn send_with_addr(e: Env, invoker: Address, to: Address, token_id: Address, amount: i128);
    fn upgrade(e: Env, new_wasm_hash: BytesN<32>);
}

#[contract]
pub struct SmartWallet;

#[contractimpl]
impl SmartWalletTrait for SmartWallet {
    fn init(e: Env, admin: Address) {
        write_administrator(&e, &admin);
    }

    fn add_executor(e: Env, executor_pub_key: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();
        let index = read_excecutor_count(&e) + 1;
        write_executor(&e, index, executor_pub_key);
    }

    fn create_account_addr(e: Env, owner: Address, subaccount_wasm_hash: BytesN<32>) -> Address {
        owner.require_auth();
        let account_id = create_with_addr(&e, &owner, subaccount_wasm_hash);
        let account_contract = sub_account::Client::new(&e, &account_id);

        account_contract.init_with_address(&e.current_contract_address(), &owner);
        let executor_count = read_excecutor_count(&e);
        for index in 1..=executor_count {
            let executor = read_executor(&e, index);
            account_contract.set_executor(&index, &executor);
        }
        account_contract.set_executor_done();
        write_account_addr(&e, owner, account_id.clone());
        account_id
    }

    fn create_account_pkey(
        e: Env,
        executor_index: u32,
        platform: String,
        profile_id: String,
        salt: String,
        salt_iv: String,
        key_index: String,
        index_iv: String,
        passkey_hash: String,
        max_allowance: i128,
        subaccount_wasm_hash: BytesN<32>,
    ) -> Address {
        if has_account_profile(&e, profile_id.clone()) {
            panic!("User with this profile id already exist")
        }
        let executor = read_executor(&e, executor_index);
        executor.require_auth();
        let account_id = create_with_pkey(
            &e,
            profile_id.clone(),
            passkey_hash.clone(),
            subaccount_wasm_hash,
        );
        let account_contract = sub_account::Client::new(&e, &account_id);

        account_contract.init_with_profile(
            &e.current_contract_address(),
            &platform,
            &profile_id,
            &salt,
            &salt_iv,
            &key_index,
            &index_iv,
            &passkey_hash,
            &max_allowance,
        );

        let executor_count = read_excecutor_count(&e);
        for index in 1..=executor_count {
            let executor = read_executor(&e, index);
            account_contract.set_executor(&index, &executor);
        }
        account_contract.set_executor_done();

        write_account_profile(&e, profile_id, account_id.clone());
        // account_contract.update_user_points(&executor, &account_id, &user_points);
        account_id
    }

    fn send_token_pkey(
        e: Env,
        executor_index: u32,
        profile_id: String,
        entered_passkey_hash: String,
        to: Address,
        token_id: Address,
        amount: i128,
    ) {
        let executor = read_executor(&e, executor_index);
        executor.require_auth();
        let sub_account_id = read_account_profile(&e, profile_id.clone()).unwrap();
        let account_contract = sub_account::Client::new(&e, &sub_account_id);
        let tx_is_valid = validate_tx(
            &e,
            executor_index,
            profile_id,
            entered_passkey_hash.clone(),
            to.clone(),
            token_id.clone(),
            amount,
        );
        if tx_is_valid == false {
            panic!("The transaction has not been authenticated")
        }

        account_contract.send_with_pkey(
            &executor_index,
            &entered_passkey_hash,
            &to,
            &token_id,
            &amount,
        );

        let mut data = account_contract.get_user_points();
        data.has_sent = true;
        data.points += 500;
        account_contract.update_user_points(&executor, &data);

        let is_valid_account = read_is_account(&e, to.clone());

        if is_valid_account {
            let account_contract = sub_account::Client::new(&e, &to);
            account_contract.set_dashboard_balance(&executor, &token_id, &amount);
        }
    }

    //Only social media binded account can call the function

    // fn set_owner(
    //     e: Env,
    //     executor_index: u32,
    //     profile_id: String,
    //     entered_passkey_hash: String,
    //     owner_id: Address,
    // ) {
    //     if read_account_addr(&e, owner_id.clone()) != None {
    //         panic!("The wallet ID is already binded to an account")
    //     }

    //     let executor = read_executor(&e, executor_index);
    //     executor.require_auth();
    //     let sub_account_id = read_account_profile(&e, profile_id.clone()).unwrap();
    //     let account_contract = sub_account::Client::new(&e, &sub_account_id);

    //     account_contract.set_owner_pkey(&executor_index, &entered_passkey_hash, &owner_id.clone());

    //     write_account_addr(&e, owner_id, sub_account_id);
    // }

    fn check_tx_validation(
        e: &Env,
        executor_index: u32,
        profile_id: String,
        entered_passkey_hash: String,
        token_id: Address,
        amount: i128,
    ) -> bool {
        let executor = read_executor(e, executor_index);
        executor.require_auth();
        let spender = e.current_contract_address();
        validate_tx(
            e,
            executor_index,
            profile_id,
            entered_passkey_hash,
            spender,
            token_id,
            amount,
        )
    }

    fn get_user_points(e: Env, account_id: Address) -> UserPoints {
        let account_contract = sub_account::Client::new(&e, &account_id.clone());
        // let account_contract = sub_account::Client::new(&e, &account_id.clone());
        account_contract.get_user_points()
        // read_quest_data(&e, account_id)
    }

    fn get_executor(e: Env, index: u32) -> Address {
        read_executor(&e, index)
    }

    fn get_account_addr(e: Env, invoker: Address) -> Option<Address> {
        read_account_addr(&e, invoker)
    }

    fn get_account_profile(e: Env, profile_id: String) -> Option<Address> {
        read_account_profile(&e, profile_id)
    }

    fn get_user_account(e: Env, owner_id: Option<Address>, profile_id: Option<String>) -> Address {
        read_account(&e, owner_id, profile_id)
    }

    fn upgrade(e: Env, new_wasm_hash: BytesN<32>) {
        let admin = read_administrator(&e);
        admin.require_auth();
        e.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}
