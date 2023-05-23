#![cfg(test)]

use super::{IncrementContract, IncrementContractClient};
use soroban_sdk::{Env};

extern crate std;


#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(&1), 1);
    assert_eq!(client.increment(&1), 2);
    assert_eq!(client.increment(&1), 3);
    assert_eq!(client.decrement(&1), 2);
}

#[test]
fn test_should_work_with_max() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    let max_value: i128 = i128::MAX;
    assert_eq!(client.increment(&max_value), max_value);
}

#[test]
#[should_panic(expected = "")]
fn should_panic_overflow() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    
    assert_eq!(client.increment(&1), 1);
    let max_value: i128 = i128::MAX;
    client.increment(&max_value);
    
    
}

#[test]
#[should_panic(expected = "")]
fn should_panic_underflow() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    
    assert_eq!(client.increment(&1), 1);
    let max_value: i128 = i128::MAX;
    assert_eq!(client.decrement(&2), max_value);
}
