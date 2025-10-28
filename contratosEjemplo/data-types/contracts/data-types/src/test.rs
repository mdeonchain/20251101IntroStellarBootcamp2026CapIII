// =============================================================================
// IMPORTACIONES NECESARIAS PARA LAS PRUEBAS
// =============================================================================

// `super::*` importa todo del módulo padre (lib.rs)
// Esto nos da acceso a PrimitivesContract y sus funciones
use super::*;

// Importamos herramientas específicas para testing de Soroban
use soroban_sdk::{
    testutils::Logs,  // Logs: nos permite acceder a los logs generados durante la ejecución
    Env               // Env: el entorno de prueba
};

// `extern crate std` nos permite usar la biblioteca estándar de Rust en tests
// Normalmente no está disponible en contratos (#![no_std]), pero sí en tests
extern crate std;

// =============================================================================
// FUNCIÓN DE PRUEBA PRINCIPAL
// =============================================================================

// La macro #[test] marca esta función como una prueba unitaria
// Cargo ejecutará esta función cuando corramos `cargo test`
#[test]
fn test_all_functions_with_logs() {
    
    // -------------------------------------------------------------------------
    // CONFIGURACIÓN DEL ENTORNO DE PRUEBA
    // -------------------------------------------------------------------------
    
    // Env::default() crea un entorno de prueba limpio y aislado
    // Este entorno simula la blockchain de Stellar/Soroban
    let env = Env::default();
    
    // Registramos nuestro contrato en el entorno de prueba
    // - PrimitivesContract: el contrato que queremos probar
    // - (): tupla vacía porque nuestro contrato no necesita argumentos de inicialización
    // Esto retorna un ID único que identifica nuestro contrato en el entorno
    let contract_id = env.register(PrimitivesContract, ());
    
    // Creamos un "cliente" que nos permite llamar las funciones del contrato
    // El cliente es una interfaz autogenerada por Soroban que facilita las llamadas
    let client = PrimitivesContractClient::new(&env, &contract_id);
    
    // std::println! imprime en la consola (solo disponible en tests)
    // Es diferente de log! que imprime en los logs del contrato
    std::println!("=== Iniciando tests de todas las funciones ===");
    
    // -------------------------------------------------------------------------
    // TEST 1: MOSTRAR ENTEROS SIN SIGNO
    // -------------------------------------------------------------------------
    std::println!("\n1. Llamando show_unsigned_integers():");
    // Llamamos a la función del contrato a través del cliente
    // Esta función no retorna nada (void), solo genera logs
    client.show_unsigned_integers();
    
    // -------------------------------------------------------------------------
    // TEST 2: MOSTRAR ENTEROS CON SIGNO
    // -------------------------------------------------------------------------
    std::println!("\n2. Llamando show_signed_integers():");
    // Demostramos el uso de tipos i32, i64 e i128
    client.show_signed_integers();
    
    // -------------------------------------------------------------------------
    // TEST 3: MOSTRAR VALORES BOOLEANOS
    // -------------------------------------------------------------------------
    std::println!("\n3. Llamando show_boolean():");
    // Probamos el tipo de dato booleano (true/false)
    client.show_boolean();
    
    // -------------------------------------------------------------------------
    // TEST 4: MOSTRAR SÍMBOLOS
    // -------------------------------------------------------------------------
    std::println!("\n4. Llamando show_symbol():");
    // Los Symbols son cadenas de texto cortas optimizadas para Soroban
    client.show_symbol();
    
    // -------------------------------------------------------------------------
    // TEST 5: OPERACIONES MATEMÁTICAS BÁSICAS
    // -------------------------------------------------------------------------
    std::println!("\n5. Llamando basic_math(15, 25):");
    // Pasamos dos números como argumentos
    // NOTA: usamos &15 y &25 (referencias) porque el cliente requiere referencias
    let math_result = client.basic_math(&15, &25);
    // La función retorna la suma: 15 + 25 = 40
    std::println!("Resultado de basic_math: {}", math_result);
    
    // -------------------------------------------------------------------------
    // TEST 6: COMPARACIONES ENTRE NÚMEROS
    // -------------------------------------------------------------------------
    std::println!("\n6. Llamando compare_numbers(100, 50):");
    // Comparamos 100 con 50
    // La función retorna true si son iguales, false si no lo son
    let compare_result = client.compare_numbers(&100, &50);
    // Como 100 != 50, el resultado será false
    std::println!("Resultado de compare_numbers: {}", compare_result);
    
    // -------------------------------------------------------------------------
    // TEST 7: OBTENER UN VALOR u32
    // -------------------------------------------------------------------------
    std::println!("\n7. Llamando get_u32():");
    // Llamamos a una función que retorna un número u32 (42)
    let u32_result = client.get_u32();
    std::println!("Resultado de get_u32: {}", u32_result);
    
    // -------------------------------------------------------------------------
    // TEST 8: OBTENER UN VALOR i128
    // -------------------------------------------------------------------------
    std::println!("\n8. Llamando get_i128():");
    // Obtenemos un número con signo de 128 bits (-123456789)
    let i128_result = client.get_i128();
    std::println!("Resultado de get_i128: {}", i128_result);
    
    // -------------------------------------------------------------------------
    // TEST 9: OBTENER UN VALOR BOOLEANO
    // -------------------------------------------------------------------------
    std::println!("\n9. Llamando get_bool():");
    // Retorna un valor booleano (true)
    let bool_result = client.get_bool();
    std::println!("Resultado de get_bool: {}", bool_result);
    
    // -------------------------------------------------------------------------
    // TEST 10: OBTENER UN SYMBOL
    // -------------------------------------------------------------------------
    std::println!("\n10. Llamando get_symbol():");
    // Retorna un Symbol ("result")
    let symbol_result = client.get_symbol();
    // Usamos {:?} para debug formatting (muestra la representación interna)
    std::println!("Resultado de get_symbol: {:?}", symbol_result);
    
    // -------------------------------------------------------------------------
    // VERIFICACIÓN DE LOGS
    // -------------------------------------------------------------------------
    std::println!("\n=== TODOS LOS LOGS GENERADOS ===");
    
    // env.logs().all() obtiene TODOS los logs generados durante la ejecución
    // Esto incluye todos los log! que se llamaron en nuestras funciones
    let logs = env.logs().all();
    
    // Iteramos sobre cada log con enumerate() para obtener el índice
    // .iter() crea un iterador sobre los logs
    // enumerate() añade un contador automático (i) empezando en 0
    for (i, log) in logs.iter().enumerate() {
        // i + 1 porque queremos contar desde 1, no desde 0
        // {:?} es debug formatting, muestra la estructura completa del log
        std::println!("Log {}: {:?}", i + 1, log);
    }
    
    // Mostramos un resumen final con el total de logs generados
    // logs.len() retorna la cantidad total de elementos en el vector
    std::println!("\n=== Test completado - Total de logs: {} ===", logs.len());
}

// =============================================================================
// NOTAS ADICIONALES PARA PRINCIPIANTES
// =============================================================================

/*
CONCEPTOS CLAVE:

1. **Entorno de prueba (Env)**:
   - Es una simulación de la blockchain
   - Aislado y limpio para cada test
   - Permite probar sin gastar recursos reales

2. **Cliente del contrato**:
   - Generado automáticamente por Soroban
   - Proporciona una interfaz tipo-segura para llamar funciones
   - El nombre siempre es: NombreDelContratoClient

3. **Referencias (&)**:
   - En Rust, & indica una "referencia" (préstamo) de un valor
   - El cliente requiere referencias para los parámetros
   - Es más eficiente que copiar el valor completo

4. **Logs en Soroban**:
   - log! en el contrato genera logs que podemos inspeccionar
   - env.logs().all() recupera todos los logs en los tests
   - Útil para debugging y verificar el comportamiento

5. **Ejecutar los tests**:
   - Comando: `cargo test`
   - Comando con output: `cargo test -- --nocapture`
   - El segundo comando muestra los std::println!

6. **Diferencia entre log! y std::println!**:
   - log!: usado en contratos, va al sistema de logs de Soroban
   - std::println!: solo en tests, imprime en la consola durante testing

PRÓXIMOS PASOS:
- Intenta agregar más tests para casos edge (valores extremos)
- Prueba con diferentes combinaciones de números
- Agrega asserts para verificar automáticamente los resultados
  Ejemplo: assert_eq!(math_result, 40);
*/
