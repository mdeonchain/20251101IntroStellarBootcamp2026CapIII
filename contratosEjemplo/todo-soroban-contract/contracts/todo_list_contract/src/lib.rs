#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, Vec};

#[contract]
pub struct TodoContract;

#[contractimpl]
impl TodoContract {
    pub fn add_task(env: Env, description: Symbol) {
        let mut tasks: Vec<Symbol> = env
            .storage()
            .get(&Symbol::short("tasks"))
            .unwrap_or(Vec::new(&env));
        tasks.push_back(description);
        env.storage().set(&Symbol::short("tasks"), &tasks);
    }

    pub fn get_tasks(env: Env) -> Vec<Symbol> {
        env.storage()
            .get(&Symbol::short("tasks"))
            .unwrap_or(Vec::new(&env))
    }
}
