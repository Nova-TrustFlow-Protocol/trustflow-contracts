use soroban_sdk::{contractimpl, token, Address, Env, String};
use crate::{DataKey, EscrowConfig, EscrowState, TrustFlowContract};

#[contractimpl]
impl TrustFlowContract {
    /// Locks the escrow, preventing standard release. Can be called by Buyer or Seller.
    pub fn raise_dispute(env: Env, caller: Address, escrow_id: u64, _reason: String) {
        caller.require_auth();
        let mut config: EscrowConfig = env.storage().persistent().get(&DataKey::Escrow(escrow_id)).expect("Escrow not found");

        assert!(caller == config.buyer || caller == config.seller, "Unauthorized");
        assert!(config.state == EscrowState::Funded, "Escrow must be funded to be disputed");

        config.state = EscrowState::Disputed;
        env.storage().persistent().set(&DataKey::Escrow(escrow_id), &config);

        // TODO: Emit Soroban events for dispute tracking
    }

    /// Arbiter resolves the dispute.
    /// If `award_to_seller` is true, funds go to seller. Otherwise, refunded to buyer.
    pub fn resolve_dispute(env: Env, arbiter: Address, escrow_id: u64, award_to_seller: bool) {
        arbiter.require_auth();
        let mut config: EscrowConfig = env.storage().persistent().get(&DataKey::Escrow(escrow_id)).expect("Escrow not found");

        assert!(arbiter == config.arbiter, "Unauthorized: Only designated arbiter can resolve");
        assert!(config.state == EscrowState::Disputed, "Escrow is not in Disputed state");

        // TODO: Add support for partial Arbiter resolutions (e.g., 50/50 split)

        let token_client = token::Client::new(&env, &config.token);
        let recipient = if award_to_seller { &config.seller } else { &config.buyer };

        token_client.transfer(&env.current_contract_address(), recipient, &config.amount);

        config.state = EscrowState::Resolved;
        env.storage().persistent().set(&DataKey::Escrow(escrow_id), &config);
    }
}