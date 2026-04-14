use soroban_sdk::{contractimpl, token, Address, Env};
use crate::{DataKey, EscrowConfig, EscrowState, TrustFlowContract};

#[contractimpl]
impl TrustFlowContract {
    /// Creates a new escrow agreement. Returns the new escrow_id.
    pub fn initialize_escrow(
        env: Env,
        buyer: Address,
        seller: Address,
        arbiter: Address,
        token: Address,
        amount: i128,
    ) -> u64 {
        buyer.require_auth();
        assert!(amount > 0, "Amount must be greater than zero");

        // TODO: Validate that the Arbiter address is not the same as the Buyer or Seller
        // TODO: Implement a fee-capture mechanism for the protocol treasury

        let mut counter: u64 = env.storage().instance().get(&DataKey::EscrowCounter).unwrap_or(0);
        counter += 1;

        let config = EscrowConfig {
            buyer,
            seller,
            arbiter,
            token,
            amount,
            state: EscrowState::Initialized,
        };

        env.storage().persistent().set(&DataKey::Escrow(counter), &config);
        env.storage().instance().set(&DataKey::EscrowCounter, &counter);

        counter
    }

    /// Buyer deposits the funds into the smart contract.
    pub fn fund_escrow(env: Env, escrow_id: u64) {
        let mut config: EscrowConfig = env.storage().persistent().get(&DataKey::Escrow(escrow_id)).expect("Escrow not found");
        config.buyer.require_auth();
        assert!(config.state == EscrowState::Initialized, "Escrow is not in Initialized state");

        let token_client = token::Client::new(&env, &config.token);
        token_client.transfer(&config.buyer, &env.current_contract_address(), &config.amount);

        config.state = EscrowState::Funded;
        env.storage().persistent().set(&DataKey::Escrow(escrow_id), &config);
    }

    /// Buyer approves the work, releasing funds to the Seller.
    pub fn approve_release(env: Env, escrow_id: u64) {
        let mut config: EscrowConfig = env.storage().persistent().get(&DataKey::Escrow(escrow_id)).expect("Escrow not found");
        config.buyer.require_auth();
        assert!(config.state == EscrowState::Funded, "Escrow is not Funded");

        let token_client = token::Client::new(&env, &config.token);
        token_client.transfer(&env.current_contract_address(), &config.seller, &config.amount);

        config.state = EscrowState::Resolved;
        env.storage().persistent().set(&DataKey::Escrow(escrow_id), &config);
    }
}