## Primitives Contract - Soroban

Contrato inteligente educativo para Soroban que demuestra el uso de tipos de datos primitivos en la blockchain de Stellar.

## 📋 Descripción

Este contrato muestra cómo trabajar con los tipos de datos primitivos disponibles en Soroban, incluyendo enteros con y sin signo, booleanos y símbolos. Es ideal para aprender los fundamentos del desarrollo en Soroban.

## 🔧 Tipos de Datos Soportados

### Enteros sin signo

* `u32`: 0 a 4,294,967,295
* `u64`: 0 a 18,446,744,073,709,551,615
* `u128`: 0 a 340,282,366,920,938,463,463,374,607,431,768,211,455

### Enteros con signo

* `i32`: -2,147,483,648 a 2,147,483,647
* `i64`: -9,223,372,036,854,775,808 a 9,223,372,036,854,775,807
* `i128`: -170,141,183,460,469,231,731,687,303,715,884,105,728 a 170,141,183,460,469,231,731,687,303,715,884,105,727

### Otros tipos

* `bool`: true/false
* `Symbol`: Cadenas cortas de hasta 32 caracteres

## 🚀 Funciones del Contrato

### Funciones de Demostración

#### `show_unsigned_integers()`

Muestra los valores máximos de tipos enteros sin signo (u32, u64, u128).

#### `show_signed_integers()`

Muestra los valores mínimos de tipos enteros con signo (i32, i64, i128).

#### `show_boolean()`

Demuestra el uso del tipo booleano con valores true y false.

#### `show_symbol()`

Muestra ejemplos de uso del tipo Symbol para cadenas cortas.

### Funciones de Operaciones

#### `basic_math(a: u32, b: u32) -> u32`

Realiza operaciones matemáticas básicas (suma y multiplicación) con dos números.

**Parámetros:**

* `a`: Primer número (u32)
* `b`: Segundo número (u32)

**Retorna:** La suma de a + b

#### `compare_numbers(x: i64, y: i64) -> bool`

Compara dos números enteros y retorna si son iguales.

**Parámetros:**

* `x`: Primer número (i64)
* `y`: Segundo número (i64)

**Retorna:** true si x == y, false en caso contrario

### Funciones Getter

#### `get_u32() -> u32`

Retorna un valor u32 de ejemplo (42).

#### `get_i128() -> i128`

Retorna un valor i128 de ejemplo (-123456789).

#### `get_bool() -> bool`

Retorna un valor booleano de ejemplo (true).

#### `get_symbol() -> Symbol`

Retorna un Symbol de ejemplo ("result").

## 🧪 Pruebas

El contrato incluye un conjunto completo de pruebas que verifican todas las funciones.

### Ejecutar las Pruebas

bash

```plaintext
cargo test -- --nocapture
```

El flag `--nocapture` permite ver todos los logs y salidas de las pruebas.

### Cobertura de Tests

El archivo `test.rs` incluye:

1. **Test de enteros sin signo**: Verifica la función `show_unsigned_integers()`
2. **Test de enteros con signo**: Verifica la función `show_signed_integers()`
3. **Test de booleanos**: Verifica la función `show_boolean()`
4. **Test de símbolos**: Verifica la función `show_symbol()`
5. **Test de matemáticas básicas**: Prueba `basic_math()` con valores 15 y 25
6. **Test de comparaciones**: Prueba `compare_numbers()` con valores 100 y 50
7. **Tests de getters**: Verifica todas las funciones getter (u32, i128, bool, symbol)

### Salida Esperada

Al ejecutar las pruebas, verás:

* Llamadas a cada función del contrato
* Resultados retornados por cada función
* Logs generados por el contrato
* Resumen total de logs al final

**Ejemplo:**

```plaintext
=== Iniciando tests de todas las funciones ===
1. Llamando show_unsigned_integers():
2. Llamando show_signed_integers():
...
10. Llamando get_symbol():
Resultado de get_symbol: Symbol(...)
 === TODOS LOS LOGS GENERADOS ===
Log 1: [...]
...
=== Test completado - Total de logs: X ===
```
📦 Estructura del Código

```plaintext
├── lib.rs (Contrato principal)
│   ├── PrimitivesContract
│   ├── show_unsigned_integers()
│   ├── show_signed_integers()
│   ├── show_boolean()
│   ├── show_symbol()
│   ├── basic_math()
│   ├── compare_numbers()
│   ├── get_u32()
│   ├── get_i128()
│   ├── get_bool()
│   └── get_symbol()
└── test.rs
    └── test_all_functions_with_logs()
```
---
⬅️[**Contratos Ejemplo** ](../README.md) 
---
# Soroban Project
