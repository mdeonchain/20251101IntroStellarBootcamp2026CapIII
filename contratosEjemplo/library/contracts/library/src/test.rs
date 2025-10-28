#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String};

#[test]
fn test_add_and_get_book() {
    let env = Env::default();
    let contract_id = env.register_contract(None, LibraryContract);
    let client = LibraryContractClient::new(&env, &contract_id);

    // Test: Agregar un libro
    let title = String::from_str(&env, "El Quijote");
    let author = String::from_str(&env, "Cervantes");
    
    let result = client.add_book(&title, &author);
    assert_eq!(result, 1); // Primer libro debe tener ID 1

    // Test: Obtener el libro
    let book = client.get_book(&1);
    assert!(book.is_some());
    
    let book = book.unwrap();
    assert_eq!(book.id, 1);
    assert_eq!(book.title, title);
    assert_eq!(book.author, author);
    assert_eq!(book.status, BookStatus::Available);
}

#[test]
fn test_borrow_book() {
    let env = Env::default();
    let contract_id = env.register_contract(None, LibraryContract);
    let client = LibraryContractClient::new(&env, &contract_id);

    // Agregar un libro
    let title = String::from_str(&env, "Cien Años de Soledad");
    let author = String::from_str(&env, "García Márquez");
    client.add_book(&title, &author);

    // Test: Tomar prestado el libro
    client.borrow_book(&1);

    // Verificar que el estado cambió
    let book = client.get_book(&1).unwrap();
    assert_eq!(book.status, BookStatus::Borrowed);

    // Test: Intentar tomar prestado un libro ya prestado
    let result = client.try_borrow_book(&1);
    assert_eq!(result, Err(Ok(LibraryError::BookNotAvailable)));
}

#[test]
fn test_invalid_book_data() {
    let env = Env::default();
    let contract_id = env.register_contract(None, LibraryContract);
    let client = LibraryContractClient::new(&env, &contract_id);

    // Test: Intentar agregar libro con datos vacíos
    let empty_title = String::from_str(&env, "");
    let author = String::from_str(&env, "Algún Autor");
    
    let result = client.try_add_book(&empty_title, &author);
    assert_eq!(result, Err(Ok(LibraryError::InvalidBookData)));
}

#[test]
fn test_book_not_found() {
    let env = Env::default();
    let contract_id = env.register_contract(None, LibraryContract);
    let client = LibraryContractClient::new(&env, &contract_id);

    // Test: Buscar libro que no existe
    let book = client.get_book(&999);
    assert!(book.is_none());

    // Test: Intentar tomar prestado libro que no existe
    let result = client.try_borrow_book(&999);
    assert_eq!(result, Err(Ok(LibraryError::BookNotFound)));
}

#[test]
fn test_get_available_books() {
    let env = Env::default();
    let contract_id = env.register_contract(None, LibraryContract);
    let client = LibraryContractClient::new(&env, &contract_id);

    // Agregar varios libros
    client.add_book(&String::from_str(&env, "Libro 1"), &String::from_str(&env, "Autor 1"));
    client.add_book(&String::from_str(&env, "Libro 2"), &String::from_str(&env, "Autor 2"));
    client.add_book(&String::from_str(&env, "Libro 3"), &String::from_str(&env, "Autor 3"));

    // Tomar prestado uno
    client.borrow_book(&2);

    // Test: Obtener libros disponibles
    let available = client.get_available_books();
    assert_eq!(available.len(), 2); // Solo 2 disponibles

    // Verificar que son los correctos
    let book1 = available.get(0).unwrap();
    let book3 = available.get(1).unwrap();
    assert_eq!(book1.id, 1);
    assert_eq!(book3.id, 3);
}

#[test]
fn test_return_book() {
    let env = Env::default();
    let contract_id = env.register_contract(None, LibraryContract);
    let client = LibraryContractClient::new(&env, &contract_id);

    // Agregar y tomar prestado un libro
    let title = String::from_str(&env, "La Casa de los Espíritus");
    let author = String::from_str(&env, "Isabel Allende");
    client.add_book(&title, &author);
    client.borrow_book(&1);

    // Verificar que está prestado
    let book = client.get_book(&1).unwrap();
    assert_eq!(book.status, BookStatus::Borrowed);

    // Test: Devolver el libro
    client.return_book(&1);

    // Verificar que volvió a estar disponible
    let book = client.get_book(&1).unwrap();
    assert_eq!(book.status, BookStatus::Available);

    // Test: Intentar devolver un libro que no está prestado
    let result = client.try_return_book(&1);
    assert_eq!(result, Err(Ok(LibraryError::BookNotAvailable)));
}

#[test]
fn test_reserve_book() {
    let env = Env::default();
    let contract_id = env.register_contract(None, LibraryContract);
    let client = LibraryContractClient::new(&env, &contract_id);

    // Agregar un libro
    let title = String::from_str(&env, "Rayuela");
    let author = String::from_str(&env, "Julio Cortázar");
    client.add_book(&title, &author);

    // Test: Reservar el libro
    client.reserve_book(&1);

    // Verificar que el estado cambió
    let book = client.get_book(&1).unwrap();
    assert_eq!(book.status, BookStatus::Reserved);

    // Test: Intentar reservar un libro ya reservado
    let result = client.try_reserve_book(&1);
    assert_eq!(result, Err(Ok(LibraryError::BookNotAvailable)));
}

#[test]
fn test_get_all_books() {
    let env = Env::default();
    let contract_id = env.register_contract(None, LibraryContract);
    let client = LibraryContractClient::new(&env, &contract_id);

    // Test: Sin libros inicialmente
    let all_books = client.get_all_books();
    assert_eq!(all_books.len(), 0);

    // Agregar algunos libros
    client.add_book(&String::from_str(&env, "Libro A"), &String::from_str(&env, "Autor A"));
    client.add_book(&String::from_str(&env, "Libro B"), &String::from_str(&env, "Autor B"));
    client.add_book(&String::from_str(&env, "Libro C"), &String::from_str(&env, "Autor C"));

    // Cambiar estados de algunos libros
    client.borrow_book(&1);
    client.reserve_book(&2);

    // Test: Obtener todos los libros (independientemente del estado)
    let all_books = client.get_all_books();
    assert_eq!(all_books.len(), 3);
    
    // Verificar que están en orden y con estados correctos
    let book1 = all_books.get(0).unwrap();
    let book2 = all_books.get(1).unwrap();
    let book3 = all_books.get(2).unwrap();
    
    assert_eq!(book1.id, 1);
    assert_eq!(book1.status, BookStatus::Borrowed);
    
    assert_eq!(book2.id, 2);
    assert_eq!(book2.status, BookStatus::Reserved);
    
    assert_eq!(book3.id, 3);
    assert_eq!(book3.status, BookStatus::Available);
}