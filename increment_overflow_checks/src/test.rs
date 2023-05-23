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
    assert_eq!(client.mul(&2), 4);
    assert_eq!(client.checked_mul(&2), 8);
    assert_eq!(client.checked_increment(&1), 9);
    assert_eq!(client.checked_decrement(&1), 8);

    // As we have i128, we can have negative numbers
    assert_eq!(client.checked_decrement(&100), -92);
    assert_eq!(client.div(&2), -46);
    assert_eq!(client.checked_div(&2), -23);
    assert_eq!(client.checked_div(&3), -7);
    assert_eq!(client.increment(&17), 10);
    assert_eq!(client.checked_div(&3), 3);
    assert_eq!(client.increment(&17), 20);
    assert_eq!(client.checked_div(&3), 6);
}

#[test]
fn test_should_work_with_max() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    let max_value: i128 = i128::MAX;
    assert_eq!(client.increment(&max_value), max_value);
    assert_eq!(client.mul(&1), max_value);
    assert_eq!(client.checked_mul(&1), max_value);
}

#[test]
#[should_panic(expected = "")]
fn increment_should_panic_overflow() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    
    assert_eq!(client.increment(&1), 1);
    let max_value: i128 = i128::MAX;
    client.increment(&max_value);
}

#[test]
#[should_panic(expected = "")]
fn checked_increment_should_panic_overflow() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    
    assert_eq!(client.increment(&1), 1);
    let max_value: i128 = i128::MAX;
    client.checked_increment(&max_value);
}

#[test]
#[should_panic(expected = "")]
fn mul_should_panic_overflow() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    
    let max_value: i128 = i128::MAX;
    assert_eq!(client.increment(&max_value), max_value);
    client.mul(&2);
}

#[test]
#[should_panic(expected = "")]
fn div_zero_should_panic() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    
    assert_eq!(client.increment(&3), 3);
    client.div(&0);
}

#[test]
#[should_panic(expected = "")]
fn checked_div_zero_should_panic() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    
    assert_eq!(client.increment(&3), 3);
    client.checked_div(&0);
}

#[test]
#[should_panic(expected = "")]
fn checked_mul_should_panic_overflow() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    
    let max_value: i128 = i128::MAX;
    assert_eq!(client.increment(&max_value), max_value);
    client.checked_mul(&2);
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
