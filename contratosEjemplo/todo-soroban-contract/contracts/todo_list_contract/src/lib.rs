#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, String, Vec};

#[derive(Clone)]
pub struct Task {
    pub description: String,
    pub done: bool,
}

#[contract]
pub struct TodoContract;#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Vec};

#[contract]
pub struct TodoContract;

#[contractimpl]
impl TodoContract {
    pub fn add_task(env: Env, description: Symbol) {
        let key = symbol_short!("tasks");
        let mut tasks: Vec<Symbol> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(Vec::new(&env));

        tasks.push_back(description);
        env.storage().persistent().set(&key, &tasks);
    }

    pub fn get_tasks(env: Env) -> Vec<Symbol> {
        let key = symbol_short!("tasks");
        env.storage()
            .persistent()
            .get(&key)
            .unwrap_or(Vec::new(&env))
    }
}


#[contractimpl]
impl TodoContract {
    pub fn add_task(env: Env, description: String) {
        let key = symbol_short!("tasks");
        let mut tasks: Vec<Task> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(Vec::new(&env));

        let task = Task {
            description,
            done: false,
        };

        tasks.push_back(task);
        env.storage().persistent().set(&key, &tasks);
    }

    pub fn mark_done(env: Env, index: u32) {
        let key = symbol_short!("tasks");
        let mut tasks: Vec<Task> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(Vec::new(&env));

        if let Some(mut task) = tasks.get(index) {
            task.done = true;
            tasks.set(index, task);
            env.storage().persistent().set(&key, &tasks);
        }
    }

    pub fn remove_task(env: Env, index: u32) {
        let key = symbol_short!("tasks");
        let mut tasks: Vec<Task> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(Vec::new(&env));

        if index < tasks.len() {
            // reconstruir lista sin el elemento eliminado
            let mut new_tasks: Vec<Task> = Vec::new(&env);
            for i in 0..tasks.len() {
                if i != index {
                    new_tasks.push_back(tasks.get_unchecked(i));
                }
            }
            env.storage().persistent().set(&key, &new_tasks);
        }
    }

    pub fn get_tasks(env: Env) -> Vec<Task> {
        let key = symbol_short!("tasks");
        env.storage()
            .persistent()
            .get(&key)
            .unwrap_or(Vec::new(&env))
    }
}
