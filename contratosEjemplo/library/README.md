## Library Contract - Soroban

Un contrato inteligente implementado en Soroban (Stellar) para gestionar una biblioteca digital con funcionalidades de préstamo, reserva y consulta de libros.

## 📋 Descripción

## Library Contract - Soroban

Este contrato demuestra las capacidades fundamentales de Soroban para la gestión de activos digitales, incluyendo:

* Gestión de estados de libros (Disponible, Prestado, Reservado)
* Almacenamiento persistente de datos de libros
* Validación de operaciones
* Consultas y filtrado de datos

## 🚀 Funcionalidades

### `add_book(title: String, author: String)`

Agrega un nuevo libro a la biblioteca.

**Parámetros:**

* `title`: Título del libro (String)
* `author`: Autor del libro (String)

**Retorna:** `Result<u32, LibraryError>` - ID del libro creado o error

```plaintext
let book\_id = client.add\_book(\&title, \&author); // Retorna ID del libro (ej: 1)
```
### `get_book(book_id: u32)`

Obtiene un libro específico por su ID.

**Parámetros:**

* `book_id`: ID del libro (u32)

**Retorna:** `Option<Book>` - Información del libro si existe

```plaintext
let book = client.get_book(&1); // Retorna Some(Book) o None
```

### `borrow_book(book_id: u32)`

Toma prestado un libro disponible.

**Parámetros:**

* `book_id`: ID del libro (u32)

**Retorna:** `Result<(), LibraryError>` - Éxito o error

```plaintext
client.borrow_book(&1); // Cambia estado a Borrowed
```

### `return_book(book_id: u32)`

Devuelve un libro prestado.

**Parámetros:**

* `book_id`: ID del libro (u32)

**Retorna:** `Result<(), LibraryError>` - Éxito o error

```plaintext
client.return_book(&1); // Cambia estado a Available
```

### `reserve_book(book_id: u32)`

Reserva un libro disponible.

**Parámetros:**

* `book_id`: ID del libro (u32)

**Retorna:** `Result<(), LibraryError>` - Éxito o error

```plaintext
client.reserve_book(&1); // Cambia estado a Reserved
```

### `get_available_books()`

Obtiene todos los libros disponibles.

**Parámetros:** Ninguno

**Retorna:** `Vec<Book>` - Lista de libros disponibles

```plaintext
let available\_books = client.get\_available\_books(); // Solo libros Available
```
### `get_all_books()`

Obtiene todos los libros en la biblioteca.

**Parámetros:** Ninguno

**Retorna:** `Vec<Book>` - Lista completa de libros

```plaintext
let all_books = client.get_all_books(); // Todos los libros sin filtrar
```

## 🎯 Estados de Libros

* `Available`: Libro disponible para préstamo
* `Borrowed`: Libro actualmente prestado
* `Reserved`: Libro reservado

## 🚫 Errores Personalizados

* `BookNotFound`: El libro con el ID especificado no existe
* `BookNotAvailable`: El libro no está disponible para la operación
* `InvalidBookData`: Los datos del libro (título/autor) están vacíos

## 🧪 Tests

El contrato incluye una suite completa de tests que validan todas las funcionalidades:

### Ejecutar los tests

bash

```plaintext
cargo test -- --nocapture
```

### Tests incluidos

* `test_add_and_get_book()` - Verifica la creación y consulta de libros
* `test_borrow_book()` - Valida el proceso de préstamo
* `test_invalid_book_data()` - Verifica validación de datos
* `test_book_not_found()` - Manejo de libros inexistentes
* `test_get_available_books()` - Filtrado de libros disponibles
* `test_return_book()` - Proceso de devolución
* `test_reserve_book()` - Funcionalidad de reserva
* `test_get_all_books()` - Consulta completa del inventario

### Estructura de los tests

```plaintext
#[test]
fn test_example() {
    // 1. Crear entorno de prueba
    let env = Env::default();
    
    // 2. Registrar el contrato
    let contract_id = env.register_contract(None, LibraryContract);
    
    // 3. Crear cliente del contrato
    let client = LibraryContractClient::new(&env, &contract_id);
    
    // 4. Ejecutar función y verificar resultado
    let result = client.some_function(&param);
    assert_eq!(result, expected_value);
}
```

}

## 📦 Estructura del Código

```plaintext
├── lib.rs (Contrato principal)
│   ├── LibraryContract
│   ├── Book (struct)
│   ├── BookStatus (enum)
│   ├── LibraryError (enum)
│   ├── add_book()
│   ├── get_book()
│   ├── borrow_book()
│   ├── return_book()
│   ├── reserve_book()
│   ├── get_available_books()
│   └── get_all_books()
└── test.rs
    ├── test_add_and_get_book()
    ├── test_borrow_book()
    ├── test_invalid_book_data()
    ├── test_book_not_found()
    ├── test_get_available_books()
    ├── test_return_book()
    ├── test_reserve_book()
    └── test_get_all_books()
```

## 💾 Almacenamiento

El contrato utiliza **Instance Storage** de Soroban para:

* Almacenar libros con su ID como clave
* Mantener un contador de IDs (`NEXT_ID_KEY`)
* Persistir estados de libros entre transacciones

## 🔒 Validaciones

* Título y autor no pueden estar vacíos
* Solo libros disponibles pueden ser prestados/reservados
* Solo libros prestados pueden ser devueltos
* Verificación de existencia de libros antes de operaciones
---
⬅️[**Contratos Ejemplo** ](../README.md) 
---
