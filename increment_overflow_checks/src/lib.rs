#![no_std]
use soroban_sdk::{contractimpl, log, Env, Symbol};

const COUNTER: Symbol = Symbol::short("COUNTER");

pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments an internal counter, and returns the value.
    pub fn increment(env: Env, n: u32) -> u32 {
        // Get the current count.
        let mut count: u32 = env
            .storage()
            .get(&COUNTER)
            .unwrap_or(Ok(0)) // If no value set, assume 0.
            .unwrap(); // Panic if the value of COUNTER is not u32.
        log!(&env, "count: {}", count);

        // Increment the count.
        count += n;

        // Save the count.
        env.storage().set(&COUNTER, &count);

        // Return the count to the caller.
        count
    }

    /// Decrement decrements an internal counter, and returns the value.
    pub fn decrement(env: Env, n: u32) -> u32 {
        // Get the current count.
        let mut count: u32 = env
            .storage()
            .get(&COUNTER)
            .unwrap_or(Ok(0)) // If no value set, assume 0.
            .unwrap(); // Panic if the value of COUNTER is not u32.
        log!(&env, "count: {}", count);

        // Decrement the count.
        count -= n;

        // Save the count.
        env.storage().set(&COUNTER, &count);

        // Return the count to the caller.
        count
    }
}

mod test;
