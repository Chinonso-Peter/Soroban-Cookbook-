#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec};

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol_short!("Hello"), to]
    }
}

mod test;

#[cfg(test)]
mod smoke_tests {
    use super::*;
    use soroban_sdk::{symbol_short, vec, Env};

    #[test]
    fn smoke_hello_world() {
        let env = Env::default();
        let contract_id = env.register_contract(None, HelloContract);
        let client = HelloContractClient::new(&env, &contract_id);

        let result = client.hello(&symbol_short!("World"));
        assert_eq!(
            result,
            vec![&env, symbol_short!("Hello"), symbol_short!("World")]
        );
    }
}
