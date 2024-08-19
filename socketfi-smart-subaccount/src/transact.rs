use soroban_sdk::{token, Address, Env};

pub fn take_token(env: &Env, from: &Address, token_id: &Address, amount: i128) {
    let token = token::Client::new(env, token_id);
    let contract_address = env.current_contract_address();

    token.transfer(from, &contract_address, &amount);
}

pub fn send_token(env: &Env, to: &Address, token_id: &Address, amount: i128) {
    let token = token::Client::new(env, token_id);
    let contract_address = env.current_contract_address();

    token.transfer(&contract_address, to, &amount);
}
