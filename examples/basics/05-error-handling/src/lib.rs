#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, symbol_short, Address, Env, Symbol};

/// Public contract error type returned to clients.
///
/// These are the only errors that cross the contract boundary, so we keep them
/// stable and explicit for frontends and indexers.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// Input failed domain validation (for example: zero count).
    InvalidInput = 1,
    /// Input exceeded a business limit enforced by the contract.
    LimitExceeded = 2,
    /// Arithmetic failed due to overflow/underflow.
    MathOverflow = 3,
    /// Division by zero was attempted.
    DivisionByZero = 4,
    /// Caller failed authorization.
    Unauthorized = 5,
}

/// Internal validation errors used only inside this module.
///
/// Keeping this separate from `Error` is useful when multiple internal layers
/// need to express fine-grained failures before mapping to a stable public API.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum ValidationError {
    ZeroCount,
    TooLarge,
}

/// Internal arithmetic errors used by helper functions.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum MathError {
    Overflow,
    ZeroDivisor,
}

/// Error conversion pattern #1:
/// Convert internal validation errors into public contract errors.
impl From<ValidationError> for Error {
    fn from(value: ValidationError) -> Self {
        match value {
            ValidationError::ZeroCount => Error::InvalidInput,
            ValidationError::TooLarge => Error::LimitExceeded,
        }
    }
}

/// Error conversion pattern #2:
/// Convert internal arithmetic errors into public contract errors.
impl From<MathError> for Error {
    fn from(value: MathError) -> Self {
        match value {
            MathError::Overflow => Error::MathOverflow,
            MathError::ZeroDivisor => Error::DivisionByZero,
        }
    }
}

#[contract]
pub struct ErrorHandlingContract;

#[contractimpl]
impl ErrorHandlingContract {
    /// Basic example that demonstrates multi-layer error bubbling.
    ///
    /// The return type is `Result<Symbol, Error>`, so callers get either the
    /// success value or a typed contract error.
    pub fn hello(_env: Env, count: u32) -> Result<Symbol, Error> {
        // `?` will bubble any `ValidationError`/`MathError` after automatic
        // conversion into `Error` through the `From` impls above.
        Self::compute_greeting_score(count)?;
        Ok(symbol_short!("Hello"))
    }

    /// End-to-end propagation example with auth + validation + arithmetic.
    ///
    /// This function shows proper bubbling from nested helpers and a final
    /// contract-facing `Result`.
    pub fn guarded_ratio(
        env: Env,
        caller: Address,
        admin: Address,
        numerator: u32,
        denominator: u32,
    ) -> Result<u32, Error> {
        // Authorization is handled first. If it fails, we return immediately.
        Self::ensure_admin(&caller, &admin)?;

        // Business validation and arithmetic each return internal error types.
        // `?` converts and bubbles them as public `Error`.
        let checked_numerator = Self::validate_limit(numerator)?;
        let scaled = Self::scale_by_two(checked_numerator)?;

        // Use `env` for deterministic behavior in tests and to avoid unused var.
        let _ledger_seq = env.ledger().sequence();

        Self::safe_divide(scaled, denominator).map_err(Error::from)
    }

    /// Validates input constraints for the "count" field.
    fn validate_limit(count: u32) -> Result<u32, ValidationError> {
        if count == 0 {
            return Err(ValidationError::ZeroCount);
        }
        if count > 10 {
            return Err(ValidationError::TooLarge);
        }
        Ok(count)
    }

    /// Performs checked multiplication to avoid overflow panics.
    fn scale_by_two(value: u32) -> Result<u32, MathError> {
        value.checked_mul(2).ok_or(MathError::Overflow)
    }

    /// Performs checked division and maps zero divisor to a typed error.
    fn safe_divide(numerator: u32, denominator: u32) -> Result<u32, MathError> {
        if denominator == 0 {
            return Err(MathError::ZeroDivisor);
        }
        Ok(numerator / denominator)
    }

    /// Authorization helper that returns a contract-level error directly.
    fn ensure_admin(caller: &Address, admin: &Address) -> Result<(), Error> {
        if caller != admin {
            return Err(Error::Unauthorized);
        }
        Ok(())
    }

    /// Multi-step helper that demonstrates bubbling across layers.
    fn compute_greeting_score(count: u32) -> Result<u32, Error> {
        let validated = Self::validate_limit(count)?;
        let doubled = Self::scale_by_two(validated)?;
        Ok(doubled)
    }
}

mod test;
