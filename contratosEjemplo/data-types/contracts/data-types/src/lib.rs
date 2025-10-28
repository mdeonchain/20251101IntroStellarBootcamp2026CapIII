#![no_std]
// Esta directiva indica que no usamos la biblioteca estándar de Rust
// Soroban requiere esto porque los contratos se ejecutan en un entorno limitado

// Importamos las herramientas necesarias del SDK de Soroban
use soroban_sdk::{contract, contractimpl, log, Env, Symbol, symbol_short};

// La macro #[contract] marca esta estructura como un contrato inteligente de Soroban
#[contract]
pub struct PrimitivesContract;

// La macro #[contractimpl] indica que las siguientes funciones son parte del contrato
// y pueden ser llamadas desde fuera (por usuarios o otros contratos)
#[contractimpl]
impl PrimitivesContract {
    
    // =============================================================================
    // ENTEROS SIN SIGNO (Unsigned Integers)
    // =============================================================================
    // Los números sin signo solo pueden ser positivos (0 y mayores)
    // Soroban soporta: u32, u64 y u128
    pub fn show_unsigned_integers(env: Env) {
        // u32: Entero de 32 bits sin signo
        // Rango: 0 a 4,294,967,295 (aproximadamente 4.3 mil millones)
        let val_u32: u32 = 4294967295;
        
        // u64: Entero de 64 bits sin signo
        // Rango: 0 a 18,446,744,073,709,551,615 (18 quintillones)
        let val_u64: u64 = 18446744073709551615;
        
        // u128: Entero de 128 bits sin signo
        // Rango: 0 a 340,282,366,920,938,463,463,374,607,431,768,211,455
        // Útil para manejar cantidades muy grandes (ideal para finanzas/tokens)
        let val_u128: u128 = 340282366920938463463374607431768211455;
        
        // log! es una macro que imprime mensajes en los registros del contrato
        // Útil para debugging y seguimiento de ejecución
        log!(&env, "Tipos sin signo:");
        log!(&env, "u32: {}", val_u32);
        log!(&env, "u64: {}", val_u64);
        log!(&env, "u128: {}", val_u128);
    }
    
    // =============================================================================
    // ENTEROS CON SIGNO (Signed Integers)
    // =============================================================================
    // Los números con signo pueden ser negativos o positivos
    // Soroban soporta: i32, i64 e i128
    pub fn show_signed_integers(env: Env) {
        // i32: Entero de 32 bits con signo
        // Rango: -2,147,483,648 a 2,147,483,647
        let val_i32: i32 = -2147483648;
        
        // i64: Entero de 64 bits con signo
        // Rango: -9,223,372,036,854,775,808 a 9,223,372,036,854,775,807
        let val_i64: i64 = -9223372036854775808;
        
        // i128: Entero de 128 bits con signo
        // Rango: aproximadamente -170 undecillones a +170 undecillones
        // Útil para cálculos financieros que requieren precisión y números grandes
        let val_i128: i128 = -170141183460469231731687303715884105728;
        
        log!(&env, "Tipos con signo:");
        log!(&env, "i32: {}", val_i32);
        log!(&env, "i64: {}", val_i64);
        log!(&env, "i128: {}", val_i128);
    }
    
    // =============================================================================
    // TIPO BOOLEANO (Boolean)
    // =============================================================================
    // Los booleanos solo tienen dos valores posibles: true (verdadero) o false (falso)
    // Se usan principalmente para lógica condicional y toma de decisiones
    pub fn show_boolean(env: Env) {
        // Declaramos una variable booleana con valor verdadero
        let is_true: bool = true;
        
        // Declaramos una variable booleana con valor falso
        let is_false: bool = false;
        
        log!(&env, "Tipo booleano:");
        log!(&env, "true: {}", is_true);
        log!(&env, "false: {}", is_false);
    }
    
    // =============================================================================
    // TIPO SYMBOL (Símbolo)
    // =============================================================================
    // Symbol es un tipo especial de Soroban para cadenas de texto cortas
    // Limitado a 32 caracteres y solo puede contener: a-z, A-Z, 0-9 y _
    // Es más eficiente que usar String para identificadores cortos
    pub fn show_symbol(env: Env) {
        // symbol_short! es una macro que crea un Symbol en tiempo de compilación
        // Esto es más eficiente que crearlo en tiempo de ejecución
        let sym1: Symbol = symbol_short!("hello");
        let sym2: Symbol = symbol_short!("world123");
        let sym3: Symbol = symbol_short!("symbol");
        
        log!(&env, "Tipo Symbol:");
        log!(&env, "Symbol 1: {}", sym1);
        log!(&env, "Symbol 2: {}", sym2);
        log!(&env, "Symbol 3: {}", sym3);
    }
    
    // =============================================================================
    // OPERACIONES MATEMÁTICAS BÁSICAS
    // =============================================================================
    // Esta función demuestra operaciones aritméticas básicas con enteros
    // Nota: Toma dos parámetros (a y b) y retorna un resultado (u32)
    pub fn basic_math(env: Env, a: u32, b: u32) -> u32 {
        // Operador +: suma dos números
        let sum = a + b;
        
        // Operador *: multiplica dos números
        // Otros operadores disponibles: - (resta), / (división), % (módulo)
        let product = a * b;
        
        log!(&env, "Matemáticas básicas:");
        log!(&env, "a: {}, b: {}", a, b);
        log!(&env, "Suma: {}", sum);
        log!(&env, "Producto: {}", product);
        
        // La última expresión sin punto y coma es el valor de retorno
        sum
    }
    
    // =============================================================================
    // OPERADORES DE COMPARACIÓN
    // =============================================================================
    // Esta función demuestra cómo comparar números y obtener resultados booleanos
    pub fn compare_numbers(env: Env, x: i64, y: i64) -> bool {
        // Operador ==: verifica si dos valores son iguales
        let is_equal = x == y;
        
        // Operador >: verifica si x es mayor que y
        let is_greater = x > y;
        
        // Operador <: verifica si x es menor que y
        // Otros operadores: >= (mayor o igual), <= (menor o igual), != (diferente)
        let is_less = x < y;
        
        log!(&env, "Comparaciones:");
        log!(&env, "x: {}, y: {}", x, y);
        log!(&env, "x == y: {}", is_equal);
        log!(&env, "x > y: {}", is_greater);
        log!(&env, "x < y: {}", is_less);
        
        // Retornamos el resultado de la comparación de igualdad
        is_equal
    }
    
    // =============================================================================
    // FUNCIONES QUE RETORNAN CADA TIPO PRIMITIVO
    // =============================================================================
    // Estas funciones simples muestran cómo retornar cada tipo de dato primitivo
    
    // Función que retorna un número sin signo de 32 bits
    pub fn get_u32(env: Env) -> u32 {
        let value: u32 = 42;  // El número 42 es solo un ejemplo
        log!(&env, "Retornando u32: {}", value);
        value  // Retornamos el valor
    }
    
    // Función que retorna un número con signo de 128 bits
    pub fn get_i128(env: Env) -> i128 {
        let value: i128 = -123456789;  // Número negativo de ejemplo
        log!(&env, "Retornando i128: {}", value);
        value
    }
    
    // Función que retorna un valor booleano
    pub fn get_bool(env: Env) -> bool {
        let value: bool = true;  // Retornamos verdadero
        log!(&env, "Retornando bool: {}", value);
        value
    }
    
    // Función que retorna un Symbol
    pub fn get_symbol(env: Env) -> Symbol {
        let value: Symbol = symbol_short!("result");  // Símbolo de ejemplo
        log!(&env, "Retornando Symbol: {}", value);
        value
    }
}

// =============================================================================
// MÓDULO DE PRUEBAS
// =============================================================================
// El módulo 'test' contendrá las pruebas unitarias del contrato
// Las pruebas se escriben en un archivo separado (test.rs)
mod test;
