#![cfg(test)]
use super::*;
use soroban_sdk::{symbol_short, testutils::Address as _, Address, Env};

#[test]
fn test_hello_success_path() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ErrorHandlingContract);
    let client = ErrorHandlingContractClient::new(&env, &contract_id);

    // Valid input should pass all internal validation + arithmetic layers.
    assert_eq!(client.hello(&5), symbol_short!("Hello"));
}

#[test]
fn test_hello_bubbles_limit_error_with_question_mark() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ErrorHandlingContract);
    let client = ErrorHandlingContractClient::new(&env, &contract_id);

    // `count > 10` fails in `validate_limit`, then bubbles through
    // `compute_greeting_score` into `hello` via `?`.
    let result = client.try_hello(&11);
    assert_eq!(result, Err(Ok(Error::LimitExceeded)));
}

#[test]
fn test_hello_bubbles_invalid_input_error() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ErrorHandlingContract);
    let client = ErrorHandlingContractClient::new(&env, &contract_id);

    // `count == 0` triggers internal `ValidationError::ZeroCount`.
    // `From<ValidationError> for Error` converts it to `Error::InvalidInput`.
    let result = client.try_hello(&0);
    assert_eq!(result, Err(Ok(Error::InvalidInput)));
}

#[test]
fn test_guarded_ratio_success() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ErrorHandlingContract);
    let client = ErrorHandlingContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    // numerator=6, scaled to 12, divided by 3 => 4
    assert_eq!(client.guarded_ratio(&admin, &admin, &6, &3), 4);
}

#[test]
fn test_guarded_ratio_unauthorized_bubbles_immediately() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ErrorHandlingContract);
    let client = ErrorHandlingContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let caller = Address::generate(&env);

    // Authorization failure is returned before validation/arithmetic runs.
    let result = client.try_guarded_ratio(&caller, &admin, &6, &3);
    assert_eq!(result, Err(Ok(Error::Unauthorized)));
}

#[test]
fn test_guarded_ratio_error_conversion_for_division_by_zero() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ErrorHandlingContract);
    let client = ErrorHandlingContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    // Internal `MathError::ZeroDivisor` is converted into `Error::DivisionByZero`.
    let result = client.try_guarded_ratio(&admin, &admin, &8, &0);
    assert_eq!(result, Err(Ok(Error::DivisionByZero)));
}
