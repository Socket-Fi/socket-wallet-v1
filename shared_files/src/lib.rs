#![no_std]
use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub struct UserPoints {
    pub has_set_signer: bool,
    pub has_set_allowance: bool,
    pub has_received: bool,
    pub has_sent: bool,
    pub points: u32,
}
