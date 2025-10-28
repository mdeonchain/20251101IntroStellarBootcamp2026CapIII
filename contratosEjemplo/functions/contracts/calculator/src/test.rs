// Esta línea indica que este módulo solo se compila cuando ejecutamos tests
#![cfg(test)]

// Importamos todos los elementos del módulo padre y tipos de Soroban SDK
use super::*;
use soroban_sdk::{vec, Env, String};

// Prueba para la función de saludo (greet)
#[test]
fn test_greet() {
    // Creamos un entorno virtual de Soroban para pruebas
    let env = Env::default();
    
    // Registramos nuestro contrato en el entorno virtual
    let contract_id = env.register(CalculatorContract, ());
    
    // Creamos un cliente para interactuar con el contrato
    let client = CalculatorContractClient::new(&env, &contract_id);
    
    // Llamamos a la función greet del contrato
    let greeting = client.greet();
    
    // Verificamos que el saludo sea el esperado
    assert_eq!(greeting, String::from_str(&env, "Hello Soroban!"));
}

// Prueba para la función de suma
#[test]
fn test_add() {
    let env = Env::default();
    let contract_id = env.register(CalculatorContract, ());
    let client = CalculatorContractClient::new(&env, &contract_id);

    // Probamos sumar 5 + 3
    let result = client.add(&5, &3);
    
    // Verificamos que el resultado sea 8
    assert_eq!(result, 8);
}

// Prueba para la función de multiplicación
#[test]
fn test_multiply() {
    let env = Env::default();
    let contract_id = env.register(CalculatorContract, ());
    let client = CalculatorContractClient::new(&env, &contract_id);

    // Probamos multiplicar 4 × 6
    let result = client.multiply(&4, &6);
    
    // Verificamos que el resultado sea 24
    assert_eq!(result, 24);
}

// Prueba para calcular y almacenar un resultado
#[test]
fn test_calculate_and_store() {
    let env = Env::default();
    let contract_id = env.register(CalculatorContract, ());
    let client = CalculatorContractClient::new(&env, &contract_id);

    // Probamos la función que calcula y guarda el resultado
    let result = client.calculate_and_store(&10, &20);
    
    // Verificamos que la suma sea correcta (10 + 20 = 30)
    assert_eq!(result, 30);
}

// Prueba para recuperar el último cálculo almacenado
#[test]
fn test_get_last_calculation() {
    let env = Env::default();
    let contract_id = env.register(CalculatorContract, ());
    let client = CalculatorContractClient::new(&env, &contract_id);

    // Primero guardamos un cálculo (15 + 25 = 40)
    client.calculate_and_store(&15, &25);
    
    // Luego recuperamos el último cálculo almacenado
    let result = client.get_last_calculation();
    
    // Verificamos que recuperemos correctamente el valor 40
    assert_eq!(result, 40);
}
