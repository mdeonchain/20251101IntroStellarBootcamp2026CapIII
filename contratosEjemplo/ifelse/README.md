## Age Evaluator Smart Contract

Un contrato inteligente desarrollado en Rust para la blockchain Soroban que evalúa la edad de los usuarios y determina sus privilegios según diferentes rangos de edad.

## 📋 Descripción

Este contrato implementa un sistema de clasificación por edad que permite:

* Categorizar usuarios según su edad
* Verificar si pueden votar
* Determinar privilegios específicos según rangos de edad

## 🔧 Funcionalidades

### 1. `evaluate_age(age: u32) -> String`

Clasifica a una persona según su edad en tres categorías:

| Edad       | Categoría       |
| ---------- | --------------- |
| \< 13 años | "Menor de edad" |
| 13-17 años | "Adolescente"   |
| ≥ 18 años  | "Mayor de edad" |

**Ejemplo:**


```plaintext
let result = client.evaluate_age(&15); // Retorna: "Adolescente"
```

### 2. `can_vote(age: u32) -> bool`

Determina si una persona puede ejercer el derecho al voto.

**Criterio:** Edad ≥ 18 años

**Ejemplo:**

```plaintext
let result = client.can_vote(&20); // Retorna: true
```

### 3. `get_privileges(age: u32) -> String`

Retorna los privilegios disponibles según la edad:

| Rango de Edad | Privilegios                  |
| ------------- | ---------------------------- |
| \< 13         | "Sin privilegios especiales" |
| 13-15         | "Puede usar redes sociales"  |
| 16-17         | "Puede conducir con permiso" |
| 18-20         | "Puede votar"                |
| ≥ 21          | "Todos los privilegios"      |

**Ejemplo:**

```plaintext
let result = client.get_privileges(&19); // Retorna: "Puede votar"
```

## 🧪 Tests

El proyecto incluye una suite completa de tests que valida todas las funcionalidades:

### Test 1: Evaluación de Edad

```plaintext
#[test] fn test_evaluate_age()
```

* **Prueba:** Evalúa que un usuario de 15 años sea clasificado correctamente
* **Resultado esperado:** "Adolescente"

### Test 2: Derecho al Voto

```plaintext
#[test] fn test_can_vote()
```

* **Prueba:** Verifica que un usuario de 20 años puede votar
* **Resultado esperado:**`true`

### Test 3: Privilegios por Edad

```plaintext
#[test] fn test_get_privileges()
```

* **Prueba:** Comprueba los privilegios de un usuario de 19 años
* **Resultado esperado:** "Puede votar"

## 🚀 Ejecución de Tests

Para ejecutar los tests:

bash

```plaintext
cargo test -- --nocapture
```

## 📦 Estructura del Código

```plaintext
├── lib.rs (Contrato principal)
│   ├── AgeEvaluatorContract
│   ├── evaluate_age()
│   ├── can_vote()
│   └── get_privileges()
└── tests/
    ├── test_evaluate_age()
    ├── test_can_vote()
    └── test_get_privileges()
```
---
⬅️[**Contratos Ejemplo** ](../README.md) 
---
