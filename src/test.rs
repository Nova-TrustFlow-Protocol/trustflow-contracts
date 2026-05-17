#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};
use soroban_sdk::token::Client as TokenClient;
use soroban_sdk::token::StellarAssetClient;

fn create_token_contract<'a>(env: &Env, admin: &Address) -> (TokenClient<'a>, StellarAssetClient<'a>) {
    let sac = env.register_stellar_asset_contract_v2(admin.clone());
    (
        TokenClient::new(env, &sac.address()),
        StellarAssetClient::new(env, &sac.address()),
    )
}

#[test]
fn test_happy_path() {
    let env = Env::default();
    env.mock_all_auths();

    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    let arbiter = Address::generate(&env);
    let token_admin = Address::generate(&env);

    let (token, token_admin_client) = create_token_contract(&env, &token_admin);
    let amount: i128 = 1000;
    token_admin_client.mint(&buyer, &amount);

    let contract_id = env.register(TrustFlowContract, ());
    let client = TrustFlowContractClient::new(&env, &contract_id);

    // Initialize
    let escrow_id = client.initialize_escrow(&buyer, &seller, &arbiter, &token.address, &amount);
    assert_eq!(escrow_id, 1);

    // Fund
    client.fund_escrow(&escrow_id);
    assert_eq!(token.balance(&contract_id), amount);
    assert_eq!(token.balance(&buyer), 0);

    // Approve
    client.approve_release(&escrow_id);
    assert_eq!(token.balance(&seller), amount);
    assert_eq!(token.balance(&contract_id), 0);
}