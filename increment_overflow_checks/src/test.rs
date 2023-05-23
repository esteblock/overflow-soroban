#![cfg(test)]

use super::{IncrementContract, IncrementContractClient};
use soroban_sdk::{testutils::Logger, Env};

extern crate std;


#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(&1), 1);
    assert_eq!(client.increment(&1), 2);
    assert_eq!(client.increment(&1), 3);
}

#[test]
fn test_should_work_with_max() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    let max_value: u32 = u32::MAX;
    assert_eq!(client.increment(&max_value), max_value);
}

#[test]
#[should_panic(expected = "")]
fn should_panic_overflow() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    
    assert_eq!(client.increment(&1), 1);
    let max_value: u32 = u32::MAX;
    client.increment(&max_value);
    
    
    std::println!("{}", env.logger().all().join("\n"));
}
