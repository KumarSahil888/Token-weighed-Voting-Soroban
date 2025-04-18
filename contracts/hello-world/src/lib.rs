#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, Map, Address, Vec, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct VoteOption {
    pub name: Symbol,
    pub vote_weight: u64,
}

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {
    pub fn init(env: Env, options: Vec<Symbol>) {
        let mut vote_map: Map<Symbol, u64> = Map::new(&env);
        for option in options.iter() {
            vote_map.set(option.clone(), 0u64);
        }
        env.storage().instance().set(&symbol_short!("OPTIONS"), &vote_map);
    }

    pub fn vote(env: Env, voter: Address, option: Symbol, weight: u64) {
        voter.require_auth();

        let mut vote_map: Map<Symbol, u64> = env
            .storage()
            .instance()
            .get(&symbol_short!("OPTIONS"))
            .unwrap();

        let existing = vote_map.get(option.clone()).unwrap_or(0);
        vote_map.set(option, existing + weight);

        env.storage().instance().set(&symbol_short!("OPTIONS"), &vote_map);
    }

    pub fn view_results(env: Env) -> Map<Symbol, u64> {
        env.storage().instance().get(&symbol_short!("OPTIONS")).unwrap()
    }
}
