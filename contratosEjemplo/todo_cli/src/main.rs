// Importamos las librerías necesarias desde crates.io (las instalamos en Cargo.toml)
/*use clap::{Parser, Subcommand}; // Permite crear una CLI (Command Line Interface)
use colored::*;                 // Agrega colores y emojis a la consola
use serde::{Deserialize, Serialize}; // Para serializar/deserialize JSON
use std::fs::{self, OpenOptions};    // Manejo de archivos (abrir, escribir, leer)
use std::io::{self, Read, Write};    // Entrada/salida (lectura/escritura)

/// Archivo donde se guardarán las tareas
const FILE: &str = "todo.json";

/// Definición del CLI principal usando `clap`.
/// Clap genera automáticamente el parser de comandos y argumentos.
#[derive(Parser)]
#[command(name = "ToDo CLI")]
#[command(about = "🦀 To-Do List CLI hecha en Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands, // Aquí se define que el usuario debe ejecutar un subcomando: add, list, done, remove
}

/// Enum para los subcomandos disponibles
/// Representa las acciones que puede ejecutar el usuario.
#[derive(Subcommand)]
enum Commands {
    Add { task: String },     // Agrega una tarea (cargo run -- add "tarea")
    List,                     // Lista todas las tareas
    Done { index: usize },    // Marca una tarea como completada (cargo run -- done 0)
    Remove { index: usize },  // Elimina una tarea (cargo run -- remove 0)
}

/// Estructura que representa una tarea en memoria.
/// `Serialize` y `Deserialize` permiten convertirla a y desde JSON fácilmente.
#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    done: bool,
}

fn main() {
    // Clap analiza los argumentos que escribes en la terminal
    let cli = Cli::parse();

    // `match` selecciona qué comando ejecutar dependiendo de lo ingresado
    match &cli.command {
        Commands::Add { task } => add_task(task),
        Commands::List => list_tasks(),
        Commands::Done { index } => mark_done(*index),
        Commands::Remove { index } => remove_task(*index),
    }
}
fn read_tasks() -> Vec<Task> {
    // Abre (o crea si no existe) el archivo JSON donde se guardan las tareas
    let mut file = OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open(FILE)
        .unwrap(); // unwrap() aborta si hay error (método rápido de manejo de errores)

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    // Si el archivo está vacío, devolvemos un vector vacío (sin tareas)
    if content.is_empty() {
        Vec::new()
    } else {
        // Convierte el contenido JSON a Vec<Task>
        serde_json::from_str(&content).unwrap_or_default()
    }
}
fn save_tasks(tasks: &[Task]) {
    // Serializa el vector de tareas a JSON legible
    let json = serde_json::to_string_pretty(tasks).unwrap();
    // Escribe el JSON completo en el archivo
    fs::write(FILE, json).unwrap();
}
fn add_task(description: &str) {
    let mut tasks = read_tasks();
    tasks.push(Task {
        description: description.to_string(),
        done: false,
    });
    save_tasks(&tasks);
    println!("{}", "✅ Tarea agregada!".green());
}
fn list_tasks() {
    let tasks = read_tasks();
    if tasks.is_empty() {
        println!("{}", "📝 No hay tareas disponibles".yellow());
    } else {
        println!("{}", "📝 Lista de tareas:".yellow());
        for (i, task) in tasks.iter().enumerate() {
            let status = if task.done { "✔️" } else { "❌" };
            println!("{} [{}] {}", i, status, task.description);
        }
    }
}*/
use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};

const FILE: &str = "todo.json";

#[derive(Parser)]
#[command(name = "ToDo CLI")]
#[command(about = "🦀 To-Do List CLI hecha en Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Agrega una nueva tarea
    Add { task: String },
    /// Lista todas las tareas
    List,
    /// Marca una tarea como completada por índice
    Done { index: usize },
    /// Elimina una tarea por índice
    Remove { index: usize },
}

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    done: bool,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Add { task } => add_task(task),
        Commands::List => list_tasks(),
        Commands::Done { index } => mark_done(*index),
        Commands::Remove { index } => remove_task(*index),
    }
}

/// Leer tareas desde el archivo JSON
fn read_tasks() -> Vec<Task> {
    let mut file = OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open(FILE)
        .expect("No se pudo abrir el archivo de tareas");

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap_or_default();

    if content.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&content).unwrap_or_default()
    }
}

/// Guardar tareas en el archivo JSON
fn save_tasks(tasks: &[Task]) {
    let json = serde_json::to_string_pretty(tasks).expect("Error al serializar tareas");
    fs::write(FILE, json).expect("No se pudo escribir el archivo");
}

/// Agregar una nueva tarea
fn add_task(description: &str) {
    let mut tasks = read_tasks();
    tasks.push(Task {
        description: description.to_string(),
        done: false,
    });
    save_tasks(&tasks);
    println!("{}", "✅ Tarea agregada con éxito".green());
}

/// Listar todas las tareas
fn list_tasks() {
    let tasks = read_tasks();
    if tasks.is_empty() {
        println!("{}", "📭 No hay tareas aún".yellow());
    } else {
        for (i, task) in tasks.iter().enumerate() {
            let status = if task.done {
                "✔️".green()
            } else {
                "❌".red()
            };
            println!("{} - {} {}", i, task.description, status);
        }
    }
}

/// Marcar tarea como completada
fn mark_done(index: usize) {
    let mut tasks = read_tasks();
    if let Some(task) = tasks.get_mut(index) {
        task.done = true;
        save_tasks(&tasks);
        println!("{}", "🎉 Tarea marcada como completada".green());
    } else {
        println!("{}", "⚠️ Índice inválido".red());
    }
}

/// Eliminar una tarea
fn remove_task(index: usize) {
    let mut tasks = read_tasks();
    if index < tasks.len() {
        tasks.remove(index);
        save_tasks(&tasks);
        println!("{}", "🗑️ Tarea eliminada".green());
    } else {
        println!("{}", "⚠️ Índice inválido".red());
    }
}
