#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, String};

#[contract]
pub struct AgeEvaluatorContract;

#[contractimpl]
impl AgeEvaluatorContract {
    pub fn evaluate_age(env: Env, age: u32) -> String {
        if age < 13 {
            String::from_str(&env, "Menor de edad")
        } else if age >= 13 && age < 18 {
            String::from_str(&env, "Adolescente")
        } else {
            String::from_str(&env, "Mayor de edad")
        }
    }
    
    pub fn can_vote(env: Env, age: u32) -> bool {
        age >= 18
    }
    
    pub fn get_privileges(env: Env, age: u32) -> String {
        if age < 13 {
            String::from_str(&env, "Sin privilegios especiales")
        } else if age >= 13 && age < 16 {
            String::from_str(&env, "Puede usar redes sociales")
        } else if age >= 16 && age < 18 {
            String::from_str(&env, "Puede conducir con permiso")
        } else if age >= 18 && age < 21 {
            String::from_str(&env, "Puede votar")
        } else {
            String::from_str(&env, "Todos los privilegios")
        }
    }
}
