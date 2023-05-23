#![no_std]
use soroban_sdk::{contractimpl, log, Env, Symbol};

const COUNTER: Symbol = Symbol::short("COUNTER");

pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments an internal counter, and returns the value.
    pub fn increment(env: Env, n: i128) -> i128 {
        // Get the current count.
        let mut count: i128 = env
            .storage()
            .get(&COUNTER)
            .unwrap_or(Ok(0)) // If no value set, assume 0.
            .unwrap(); // Panic if the value of COUNTER is not i128.
        log!(&env, "count: {}", count);

        // Increment the count.
        count += n;

        // Save the count.
        env.storage().set(&COUNTER, &count);

        // Return the count to the caller.
        count
    }

    /// Increment increments an internal counter, and returns the value.
    pub fn checked_increment(env: Env, n: i128) -> i128 {
        // Get the current count.
        let mut count: i128 = env
            .storage()
            .get(&COUNTER)
            .unwrap_or(Ok(0)) // If no value set, assume 0.
            .unwrap(); // Panic if the value of COUNTER is not i128.
        log!(&env, "count: {}", count);

        // Increment the count.
        count = count.checked_add(n).unwrap();

        // Save the count.
        env.storage().set(&COUNTER, &count);

        // Return the count to the caller.
        count
    }

    /// Decrement decrements an internal counter, and returns the value.
    pub fn decrement(env: Env, n: i128) -> i128 {
        // Get the current count.
        let mut count: i128 = env
            .storage()
            .get(&COUNTER)
            .unwrap_or(Ok(0)) // If no value set, assume 0.
            .unwrap(); // Panic if the value of COUNTER is not i128.
        log!(&env, "count: {}", count);

        // Decrement the count.
        count -= n;

        // Save the count.
        env.storage().set(&COUNTER, &count);

        // Return the count to the caller.
        count
    }

    /// Decrement decrements an internal counter, and returns the value.
    pub fn checked_decrement(env: Env, n: i128) -> i128 {
        // Get the current count.
        let mut count: i128 = env
            .storage()
            .get(&COUNTER)
            .unwrap_or(Ok(0)) // If no value set, assume 0.
            .unwrap(); // Panic if the value of COUNTER is not i128.
        log!(&env, "count: {}", count);

        // Decrement the count.
        count = count.checked_sub(n).unwrap();

        // Save the count.
        env.storage().set(&COUNTER, &count);

        // Return the count to the caller.
        count
    }

    /// Mul multiplies an internal counter, and returns the value.
    pub fn mul(env: Env, n: i128) -> i128 {
        // Get the current count.
        let mut count: i128 = env
            .storage()
            .get(&COUNTER)
            .unwrap_or(Ok(0)) // If no value set, assume 0.
            .unwrap(); // Panic if the value of COUNTER is not i128.
        log!(&env, "count: {}", count);

        // Decrement the count.
        count = count * n;

        // Save the count.
        env.storage().set(&COUNTER, &count);

        // Return the count to the caller.
        count
    }

    /// Mul multiplies with CHECKED_MUL
    pub fn checked_mul(env: Env, n: i128) -> i128 {
        // Get the current count.
        let mut count: i128 = env
            .storage()
            .get(&COUNTER)
            .unwrap_or(Ok(0)) // If no value set, assume 0.
            .unwrap(); // Panic if the value of COUNTER is not i128.
        log!(&env, "count: {}", count);

        // Decrement the count.
        count = count.checked_mul(n).unwrap();

        // Save the count.
        env.storage().set(&COUNTER, &count);

        // Return the count to the caller.
        count
    }
}

mod test;
