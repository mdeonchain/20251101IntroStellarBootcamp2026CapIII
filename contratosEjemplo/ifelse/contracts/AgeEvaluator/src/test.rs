#![cfg(test)]

use super::*;
use soroban_sdk::{vec, Env, String};

#[test]
fn test_evaluate_age() {
    
    let env = Env::default();
    let contract_id = env.register_contract(None, AgeEvaluatorContract);
    let client = AgeEvaluatorContractClient::new(&env, &contract_id);

    let result = client.evaluate_age(&15);
    assert_eq!(result, String::from_str(&env, "Adolescente"));
}

#[test]
fn test_can_vote() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AgeEvaluatorContract);
    let client = AgeEvaluatorContractClient::new(&env, &contract_id);
    let result = client.can_vote(&20);
    assert_eq!(result, true);
}

#[test]
fn test_get_privileges() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AgeEvaluatorContract);
    let client = AgeEvaluatorContractClient::new(&env, &contract_id);

    let result = client.get_privileges(&19);
    assert_eq!(result, String::from_str(&env, "Puede votar"));
}
