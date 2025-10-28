// Indicamos que no usaremos la biblioteca estándar de Rust
// Esto es común en contratos inteligentes para reducir el tamaño del código
#![no_std]

// Importamos los tipos y macros necesarios del SDK de Soroban
use soroban_sdk::{contract, contractimpl, contracterror, contracttype, Env, String, Symbol, Vec, symbol_short};

// =============================================================================
// DEFINICIÓN DE TIPOS DE DATOS
// =============================================================================

/// Enum que representa los posibles estados de un libro en la biblioteca
/// Cada variante representa un estado diferente que puede tener un libro
#[contracttype]  // Esta macro permite que el enum sea usado en el storage del contrato
#[derive(Clone, Debug, Eq, PartialEq)]  // Implementamos traits útiles para comparación y clonación
pub enum BookStatus {
    Available,   // El libro está disponible para préstamo
    Borrowed,    // El libro está actualmente prestado
    Reserved,    // El libro está reservado
}

/// Struct que representa un libro en nuestro sistema de biblioteca
/// Contiene toda la información básica de un libro
#[contracttype]  // También marcamos este struct para que pueda ser almacenado
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Book {
    pub id: u32,        // Identificador único del libro
    pub title: String,  // Título del libro
    pub author: String, // Autor del libro
    pub status: BookStatus, // Estado actual del libro
}

/// Enum para manejar errores personalizados del contrato
/// Esto nos permite tener mensajes de error específicos y claros
#[contracterror]  // Macro especial para errores en contratos Soroban
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]  // Representamos cada error como un número u32
pub enum LibraryError {
    BookNotFound = 1,     // Cuando no se encuentra un libro por su ID
    BookNotAvailable = 2, // Cuando el libro no está disponible para una operación
    InvalidBookData = 3,  // Cuando los datos del libro son inválidos (vacíos)
}

// =============================================================================
// DEFINICIÓN DEL CONTRATO PRINCIPAL
// =============================================================================

/// Esta macro marca el struct como un contrato Soroban
#[contract]
pub struct LibraryContract;  // Nuestro contrato principal (sin campos)

// Constante para la clave donde guardaremos el próximo ID disponible
// symbol_short! crea un Symbol eficiente para claves de storage
const NEXT_ID_KEY: Symbol = symbol_short!("next_id");

// =============================================================================
// IMPLEMENTACIÓN DEL CONTRATO
// =============================================================================

/// Implementamos todas las funciones del contrato para LibraryContract
#[contractimpl]
impl LibraryContract {

    /// 🆕 AGREGAR NUEVO LIBRO
    /// Crea un nuevo libro en la biblioteca con estado "Available"
    /// 
    /// Parámetros:
    /// - env: El entorno de ejecución de Soroban (proporcionado automáticamente)
    /// - title: Título del libro
    /// - author: Autor del libro
    /// 
    /// Retorna: Result<u32, LibraryError> - El ID del nuevo libro o un error
    pub fn add_book(env: Env, title: String, author: String) -> Result<u32, LibraryError> {
        // 🔍 VALIDACIÓN: Verificamos que los datos no estén vacíos
        if title.len() == 0 || author.len() == 0 {
            return Err(LibraryError::InvalidBookData);
        }

        // 📊 OBTENER PRÓXIMO ID: Usamos nuestra función helper
        let next_id = Self::get_next_book_id(&env);

        // 📖 CREAR NUEVO LIBRO: Construimos la estructura Book
        let new_book = Book {
            id: next_id,                    // Asignamos el ID automáticamente
            title: title.clone(),           // Clonamos el título (String es dueño de sus datos)
            author: author.clone(),         // Clonamos el autor
            status: BookStatus::Available,  // Estado inicial: Disponible
        };

        // 💾 GUARDAR EN STORAGE: Usamos instance storage (datos persistentes del contrato)
        // Guardamos el libro usando su ID como clave
        env.storage().instance().set(&next_id, &new_book);

        // 🔢 ACTUALIZAR CONTADOR: Incrementamos el próximo ID disponible
        env.storage().instance().set(&NEXT_ID_KEY, &(next_id + 1));

        // ✅ RETORNAR ÉXITO: Devolvemos el ID del libro creado
        Ok(next_id)
    }

    /// 🔍 OBTENER LIBRO POR ID
    /// Busca un libro en el storage usando su ID
    /// 
    /// Parámetros:
    /// - env: Entorno de Soroban
    /// - book_id: ID del libro a buscar
    /// 
    /// Retorna: Option<Book> - Some(book) si existe, None si no existe
    pub fn get_book(env: Env, book_id: u32) -> Option<Book> {
        // 📚 BUSCAR EN STORAGE: Intentamos obtener el libro del storage
        // Si no existe, retorna None automáticamente
        env.storage().instance().get(&book_id)
    }

    /// 🔄 CAMBIAR ESTADO DE LIBRO
    /// Función interna para actualizar el estado de un libro
    /// 
    /// Parámetros:
    /// - env: Entorno de Soroban
    /// - book_id: ID del libro a modificar
    /// - new_status: Nuevo estado a asignar
    /// 
    /// Retorna: Result<(), LibraryError> - Éxito o error si el libro no existe
    pub fn change_book_status(
        env: Env,
        book_id: u32,
        new_status: BookStatus,
    ) -> Result<(), LibraryError> {
        // 📖 OBTENER LIBRO ACTUAL: Buscamos el libro o retornamos error
        // ok_or convierte Option<> en Result<> con nuestro error personalizado
        let mut book: Book = env
            .storage()
            .instance()
            .get(&book_id)
            .ok_or(LibraryError::BookNotFound)?;

        // ✏️ ACTUALIZAR ESTADO: Modificamos el campo status
        book.status = new_status;

        // 💾 GUARDAR CAMBIOS: Persistimos el libro actualizado
        env.storage().instance().set(&book_id, &book);

        Ok(())  // Retornamos éxito sin datos
    }

    /// 📚 PRESTAR LIBRO
    /// Permite tomar prestado un libro si está disponible
    /// 
    /// Parámetros:
    /// - env: Entorno de Soroban
    /// - book_id: ID del libro a prestar
    /// 
    /// Retorna: Result<(), LibraryError> - Éxito o error si no está disponible
    pub fn borrow_book(env: Env, book_id: u32) -> Result<(), LibraryError> {
        // 📖 OBTENER LIBRO: Verificamos que existe
        let book: Book = env
            .storage()
            .instance()
            .get(&book_id)
            .ok_or(LibraryError::BookNotFound)?;

        // ✅ VERIFICAR DISPONIBILIDAD: Usamos match para ver el estado actual
        match book.status {
            BookStatus::Available => {
                // 🔄 CAMBIAR ESTADO: Si está disponible, lo marcamos como prestado
                Self::change_book_status(env, book_id, BookStatus::Borrowed)
            }
            _ => {
                // ❌ NO DISPONIBLE: Si no está disponible, retornamos error
                Err(LibraryError::BookNotAvailable)
            }
        }
    }

    /// ↩️ DEVOLVER LIBRO
    /// Permite devolver un libro que estaba prestado
    /// 
    /// Parámetros:
    /// - env: Entorno de Soroban
    /// - book_id: ID del libro a devolver
    /// 
    /// Retorna: Result<(), LibraryError> - Éxito o error si no estaba prestado
    pub fn return_book(env: Env, book_id: u32) -> Result<(), LibraryError> {
        // 📖 OBTENER LIBRO: Verificamos que existe
        let book: Book = env
            .storage()
            .instance()
            .get(&book_id)
            .ok_or(LibraryError::BookNotFound)?;

        // ✅ VERIFICAR QUE ESTÉ PRESTADO: Solo podemos devolver libros prestados
        match book.status {
            BookStatus::Borrowed => {
                // 🔄 CAMBIAR ESTADO: Lo marcamos como disponible nuevamente
                Self::change_book_status(env, book_id, BookStatus::Available)
            }
            _ => {
                // ❌ NO ESTABA PRESTADO: No se puede devolver un libro que no estaba prestado
                Err(LibraryError::BookNotAvailable)
            }
        }
    }

    /// 🔧 FUNCIÓN HELPER: OBTENER PRÓXIMO ID
    /// Función interna que obtiene el próximo ID disponible para nuevos libros
    /// 
    /// Parámetros:
    /// - env: Referencia al entorno de Soroban
    /// 
    /// Retorna: u32 - El próximo ID disponible (1 si es el primer libro)
    fn get_next_book_id(env: &Env) -> u32 {
        // 📊 OBTENER DEL STORAGE: Buscamos el contador actual
        // unwrap_or(1) significa: si no existe, usa 1 como valor por defecto
        env.storage().instance().get(&NEXT_ID_KEY).unwrap_or(1)
    }

    /// 📋 OBTENER LIBROS DISPONIBLES
    /// Retorna una lista de todos los libros que están disponibles para préstamo
    /// 
    /// Parámetros:
    /// - env: Entorno de Soroban
    /// 
    /// Retorna: Vec<Book> - Vector con los libros disponibles
    pub fn get_available_books(env: Env) -> Vec<Book> {
        // 🆕 CREAR VECTOR VACÍO: Creamos un vector en el entorno de Soroban
        let mut available_books = Vec::new(&env);
        
        // 🔢 OBTENER LÍMITE: Sabemos hasta qué ID buscar
        let next_id = Self::get_next_book_id(&env);

        // 🔄 ITERAR SOBRE TODOS LOS LIBROS: Desde el ID 1 hasta el último
        for id in 1..next_id {
            // 📖 INTENTAR OBTENER CADA LIBRO: if let es una forma concisa de manejar Options
            if let Some(book) = Self::get_book(env.clone(), id) {
                // ✅ FILTRAR SOLO DISPONIBLES: Verificamos el estado
                if book.status == BookStatus::Available {
                    // ➕ AGREGAR AL RESULTADO: Añadimos al final del vector
                    available_books.push_back(book);
                }
            }
            // Nota: Si el libro no existe, simplemente continuamos
        }

        available_books  // Retornamos el vector resultante
    }

    /// 📚 OBTENER TODOS LOS LIBROS
    /// Retorna una lista completa de todos los libros en la biblioteca
    /// 
    /// Parámetros:
    /// - env: Entorno de Soroban
    /// 
    /// Retorna: Vec<Book> - Vector con todos los libros
    pub fn get_all_books(env: Env) -> Vec<Book> {
        // 🆕 CREAR VECTOR VACÍO
        let mut all_books = Vec::new(&env);
        
        // 🔢 OBTENER LÍMITE
        let next_id = Self::get_next_book_id(&env);

        // 🔄 ITERAR SOBRE TODOS LOS LIBROS
        for id in 1..next_id {
            // 📖 AGREGAR TODOS LOS LIBROS EXISTENTES
            if let Some(book) = Self::get_book(env.clone(), id) {
                all_books.push_back(book);
            }
        }

        all_books  // Retornamos el vector completo
    }

    /// 🔒 RESERVAR LIBRO
    /// Permite reservar un libro disponible para uso futuro
    /// 
    /// Parámetros:
    /// - env: Entorno de Soroban
    /// - book_id: ID del libro a reservar
    /// 
    /// Retorna: Result<(), LibraryError> - Éxito o error si no está disponible
    pub fn reserve_book(env: Env, book_id: u32) -> Result<(), LibraryError> {
        // 📖 OBTENER LIBRO: Verificamos que existe
        let book: Book = env
            .storage()
            .instance()
            .get(&book_id)
            .ok_or(LibraryError::BookNotFound)?;

        // ✅ VERIFICAR DISPONIBILIDAD: Solo podemos reservar libros disponibles
        match book.status {
            BookStatus::Available => {
                // 🔄 CAMBIAR ESTADO: Lo marcamos como reservado
                Self::change_book_status(env, book_id, BookStatus::Reserved)
            }
            _ => {
                // ❌ NO DISPONIBLE: No se puede reservar un libro no disponible
                Err(LibraryError::BookNotAvailable)
            }
        }
    }
}

// Incluimos el archivo de tests (separado para mejor organización)
mod test;
