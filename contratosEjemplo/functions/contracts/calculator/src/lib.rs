// ============================================================================
// CALCULATOR CONTRACT - SOROBAN
// ============================================================================
// Este es un contrato inteligente simple para la blockchain de Stellar
// usando el framework Soroban. Funciona como una calculadora básica.
// ============================================================================

// Esta línea indica que NO usaremos la biblioteca estándar de Rust
// Los contratos de Soroban deben ser ligeros y no pueden usar std
#![no_std]

// Importamos las herramientas necesarias del SDK de Soroban:
// - contract: macro para marcar una estructura como contrato
// - contractimpl: macro para implementar las funciones del contrato
// - vec: tipo de vector compatible con Soroban
// - Env: entorno de ejecución del contrato (acceso a storage, logs, etc.)
// - String: tipo String compatible con Soroban
// - symbol_short: macro para crear identificadores cortos (máx 9 caracteres)
use soroban_sdk::{contract, contractimpl, vec, Env, String, symbol_short};

// ============================================================================
// DEFINICIÓN DEL CONTRATO
// ============================================================================

// El macro #[contract] marca esta estructura como un contrato de Soroban
// Esto genera automáticamente código necesario para que funcione en blockchain
#[contract]
pub struct CalculatorContract;
// Nota: La estructura está vacía porque no necesitamos almacenar datos
// de configuración en la estructura misma (usaremos el storage del entorno)

// ============================================================================
// IMPLEMENTACIÓN DE LAS FUNCIONES DEL CONTRATO
// ============================================================================

// El macro #[contractimpl] indica que esta implementación contiene
// las funciones públicas que pueden ser llamadas desde fuera del contrato
#[contractimpl]
impl CalculatorContract {
    
    // ------------------------------------------------------------------------
    // FUNCIÓN: greet
    // ------------------------------------------------------------------------
    // Propósito: Retorna un mensaje de saludo simple
    // Parámetros:
    //   - env: El entorno de Soroban (siempre es el primer parámetro)
    // Retorna: Un String con el mensaje "Hello Soroban!"
    // ------------------------------------------------------------------------
    pub fn greet(env: Env) -> String {
        // Creamos un String de Soroban a partir de un &str de Rust
        // Nota: No podemos usar String::from() normal porque estamos en no_std
        String::from_str(&env, "Hello Soroban!")
    }
    
    // ------------------------------------------------------------------------
    // FUNCIÓN: add
    // ------------------------------------------------------------------------
    // Propósito: Suma dos números enteros
    // Parámetros:
    //   - env: Entorno de Soroban (requerido en todas las funciones públicas)
    //   - a: Primer número a sumar (entero de 32 bits con signo)
    //   - b: Segundo número a sumar (entero de 32 bits con signo)
    // Retorna: La suma de a + b como i32
    // Ejemplo: add(5, 3) retorna 8
    // ------------------------------------------------------------------------
    pub fn add(env: Env, a: i32, b: i32) -> i32 {
        // Operación simple: suma los dos números y retorna el resultado
        a + b
    }
    
    // ------------------------------------------------------------------------
    // FUNCIÓN: multiply
    // ------------------------------------------------------------------------
    // Propósito: Multiplica dos números enteros
    // Parámetros:
    //   - env: Entorno de Soroban
    //   - a: Primer número a multiplicar
    //   - b: Segundo número a multiplicar
    // Retorna: El producto de a * b como i32
    // Ejemplo: multiply(4, 6) retorna 24
    // ------------------------------------------------------------------------
    pub fn multiply(env: Env, a: i32, b: i32) -> i32 {
        // Operación simple: multiplica los dos números
        a * b
    }
    
    // ------------------------------------------------------------------------
    // FUNCIÓN: calculate_and_store
    // ------------------------------------------------------------------------
    // Propósito: Suma dos números Y guarda el resultado en el storage
    // Parámetros:
    //   - env: Entorno de Soroban (usado para acceder al storage)
    //   - a: Primer número
    //   - b: Segundo número
    // Retorna: El resultado de la suma
    // 
    // IMPORTANTE: Esta función demuestra cómo PERSISTIR datos en blockchain
    // El resultado se guarda y puede ser recuperado después con get_last_calculation()
    // ------------------------------------------------------------------------
    pub fn calculate_and_store(env: Env, a: i32, b: i32) -> i32 {
        // Paso 1: Calculamos la suma
        let result = a + b;
        
        // Paso 2: Guardamos el resultado en el storage
        // - env.storage().instance() accede al almacenamiento de esta instancia
        // - .set() guarda un valor con una clave
        // - symbol_short!("last_calc") crea la clave (máximo 9 caracteres)
        // - &result es el valor que queremos guardar
        env.storage().instance().set(&symbol_short!("last_calc"), &result);
        
        // Paso 3: Retornamos el resultado
        result
    }
    
    // ------------------------------------------------------------------------
    // FUNCIÓN: get_last_calculation
    // ------------------------------------------------------------------------
    // Propósito: Recupera el último cálculo guardado en el storage
    // Parámetros:
    //   - env: Entorno de Soroban (para acceder al storage)
    // Retorna: El último resultado guardado, o 0 si no hay ninguno
    // 
    // NOTA: Esta función lee datos que fueron guardados previamente
    // con calculate_and_store()
    // ------------------------------------------------------------------------
    pub fn get_last_calculation(env: Env) -> i32 {
        // Intentamos obtener el valor guardado con la clave "last_calc"
        // - env.storage().instance().get() intenta recuperar un valor
        // - .unwrap_or(0) significa: "si no existe, retorna 0 como valor por defecto"
        // 
        // ¿Por qué unwrap_or()?
        // Porque .get() puede fallar si nunca se ha guardado nada,
        // así que damos un valor por defecto (0) en lugar de causar un error
        env.storage().instance().get(&symbol_short!("last_calc")).unwrap_or(0)
    }
}

// ============================================================================
// MÓDULO DE TESTS
// ============================================================================
// Esta línea indica que hay un archivo test.rs con las pruebas del contrato
// Los tests verifican que todas las funciones funcionan correctamente
mod test;
