 #![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _},
    Address, Env,
};

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register(ZapContract, ());
    let client = ZapContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let router = Address::generate(&env);
    
    client.initialize(&router, &admin, &100, &1000);
    
    let config = client.get_config();
    assert_eq!(config.admin, admin);
    assert_eq!(config.router, router);
}

#[test]
fn test_preview_zap_in() {
    let env = Env::default();
    let contract_id = env.register(ZapContract, ());
    let client = ZapContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let router = Address::generate(&env);
    client.initialize(&router, &admin, &100, &1000);
    
    let token_in = Address::generate(&env);
    let pool = Address::generate(&env);
    
    let result = client.preview_zap_in(&token_in, &10000, &pool);
    assert_eq!(result.token_a_amount, 5000);
    assert_eq!(result.token_b_amount, 5000);
    assert_eq!(result.lp_tokens_expected, 5000);
}

#[test]
fn test_zap_in_amount_too_low() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(ZapContract, ());
    let client = ZapContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let router = Address::generate(&env);
    client.initialize(&router, &admin, &100, &1000);
    
    let token_in = Address::generate(&env);
    let pool = Address::generate(&env);
    let recipient = Address::generate(&env);
    
    let result = client.try_zap_in(
        &token_in,
        &100,
        &pool,
        &0,
        &recipient,
    );
    
    assert!(result.is_err());
}

#[test]
fn test_zap_in_success() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(ZapContract, ());
    let client = ZapContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let router = Address::generate(&env);
    client.initialize(&router, &admin, &100, &1000);
    
    let token_in = Address::generate(&env);
    let pool = Address::generate(&env);
    let recipient = Address::generate(&env);
    
    let result = client.zap_in(
        &token_in,
        &5000,
        &pool,
        &50,
        &recipient,
    );
    
    assert_eq!(result, 2500);
}

#[test]
fn test_renounce_admin() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register(ZapContract, ());
    let client = ZapContractClient::new(&env, &contract_id);
    
    let admin = Address::generate(&env);
    let router = Address::generate(&env);
    client.initialize(&router, &admin, &100, &1000);
    
    // Renounce admin - deve mudar admin para o endereço do contrato
    client.renounce_admin();
    
    // Verificar que admin foi alterado para o endereço do contrato
    let config = client.get_config();
    assert_eq!(config.admin, contract_id);
}