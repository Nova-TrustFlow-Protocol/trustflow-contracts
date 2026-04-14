#![no_std]

pub mod escrow;
pub mod dispute;
#[cfg(test)]
mod test;

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EscrowState {
    Initialized,
    Funded,
    Disputed,
    Resolved,
}

#[contracttype]
#[derive(Clone)]
pub struct EscrowConfig {
    pub buyer: Address,
    pub seller: Address,
    pub arbiter: Address,
    pub token: Address,
    pub amount: i128,
    pub state: EscrowState,
}

#[contracttype]
pub enum DataKey {
    Escrow(u64),
    EscrowCounter,
}

#[contract]
pub struct TrustFlowContract;

#[contractimpl]
impl TrustFlowContract {
    /// Returns the current configuration of an escrow.
    pub fn get_escrow(env: Env, escrow_id: u64) -> EscrowConfig {
        env.storage()
            .persistent()
            .get(&DataKey::Escrow(escrow_id))
            .unwrap_or_else(|| panic!("Escrow ID not found"))
    }
}