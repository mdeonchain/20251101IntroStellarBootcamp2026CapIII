## 🦀 Fundamentos de RUST

> **¡Bienvenido al mundo de Rust!** Un lenguaje de programación moderno, seguro y eficiente.

---

## 🚀 Entorno de Desarrollo

Todo el código lo correremos en el playground de Rust, **¡no necesitas instalar nada!**

### 🔗 [Playground de Rust](https://play.rust-lang.org/)

---

## 📝 Primeros Pasos en Rust

### 👋 Tu Primer Hola Mundo

```plaintext
fn main() {  
   println!("Hello, world!");
}
```

---

## 🔤 Declaración de Variables

En Rust, las variables son **inmutables por defecto**. Esto ayuda a escribir código más seguro.

```plaintext
fn main(){    
 // Variable inmutable (por defecto)   
 let nombre = "Juan";   
 println!("nombre {}",nombre);    
 // Variable mutable    
 let mut edad = 25;  
 println!("edad: {}",edad);   
 edad = 26; // ✅ Permitido    
 println!("nueva edad: {}",edad);    
 // Constante
 const PI: f64 = 3.14159;  
 println!("PI: {}",PI);
}
```

**💡 Tip:** Usa `mut` solo cuando realmente necesites cambiar el valor de una variable.

---

## 🎯 Tipos Básicos

Rust es un lenguaje **fuertemente tipado**, lo que significa que cada variable tiene un tipo específico.

```plaintext
fn main(){   
  // Números enteros   
 let numero: i32 = 42;
 let grande: u64 = 1000000;
 // Números decimales 
 let precio: f64 = 19.99;  
 // Texto  
 let mensaje: String = String::from("Hola Soroban");  
 let letra: char = 'A';
 // Booleano   
 let activo: bool = true;   
 println!("numero: {} grande: {} precio: {}",numero,grande,precio);
 println!("mensaje: {} letra: {} activo: {}",mensaje,letra,activo);
}
```

---

## ⚙️ Funciones

Las funciones son bloques de código reutilizables que realizan tareas específicas.

```plaintext
// Función simple  
fn saludar() { 
  println!("¡Hola mundo!"); 
}  
// Función con parámetros  
fn sumar(a: i32, b: i32) -> i32 {  
 // Sin punto y coma = return 
 a + b 
}  

fn main() { 
   saludar(); 
   let resultado = sumar(5, 3); 
   println!("5 + 3 = {}", resultado); 
}
```

**⚠️ Importante:** En Rust, omitir el punto y coma en la última expresión es equivalente a un `return`.

---

## 🔀 Control de Flujo: If/Else

Toma decisiones en tu código según diferentes condiciones.

 

```plaintext
fn evaluar_edad(edad: u32) {    
   if edad < 13{
      println!("Eres menor de edad"); 
   } 
   else if edad >= 13 && edad < 18{ 
      println!("Eres adolescente"); 
   } 
   else { 
        println!("Eres mayor de edad");  
   }  
}
fn main(){ 
  evaluar_edad(18);
}
```

---

## 🏗️ Structs (Estructuras)

Las estructuras te permiten **agrupar datos relacionados** en un solo tipo personalizado.

```plaintext
struct Persona {    
       nombre: String,
       edad: u32
}

fn main() { 
   // Crear una instancia de Persona    
   let juan = Persona {
                       nombre: String::from("Juan"),
                       edad: 30
                      };
   println!("Nombre: {}, Edad: {}", juan.nombre, juan.edad); 
}
```

---

## 🎨 Enums (Enumeraciones)

Define un **conjunto de opciones posibles** para representar estados o categorías.

```plaintext
enum Estado {   
       Activo, 
       Inactivo,
       Pendiente
     }

fn imprimir_estado(estado: Estado) {   
           match estado { 
                          Estado::Activo => println!("El estado es ACTIVO ✅"), 
                          Estado::Inactivo => println!("El estado es INACTIVO ❌"),
                          Estado::Pendiente => println!("El estado está PENDIENTE ⏳")
                        } 
} 
fn main() {  
       let estado_usuario = Estado::Pendiente;
       imprimir_estado(estado_usuario);
}
```

---

## 📦 Colecciones: Vec y HashMap

### 📋 Vec - Listas Dinámicas

Un `Vec` es como un array que puede **crecer o reducirse** según sea necesario.

### 🗂️ HashMap - Pares Clave-Valor

Un `HashMap` almacena datos en formato **clave-valor**, perfecto para búsquedas rápidas.

 

```plaintext
use std::collections::HashMap; 
fn main() {  
  // Vector  
 let mut numeros: Vec<i32> = Vec::new(); 
 numeros.push(10);
 numeros.push(20);
 numeros.push(30);
 println!("Vector: {:?}", numeros); 
 // HashMap    
 let mut edades: HashMap<&str, u32> = HashMap::new();  
 edades.insert("Juan", 30); 
 edades.insert("Ana", 25);
 println!("Edad de Juan: {:?}", edades.get("Juan")); 
}
```

---

## ⚠️ Manejo de Errores

### 🛡️ Result\<T, E>

El tipo `Result` se usa para operaciones que **pueden fallar**. Es la forma idiomática de Rust para manejar errores.

```plaintext
fn dividir(a: i32, b: i32) -> Result<i32, String> {  
    if b == 0 {
       Err(String::from("No se puede dividir entre 0")) 
    } 
    else {       
        Ok(a / b)   
    } 
}
fn main() { 
   match dividir(10, 2) {  
         Ok(resultado) => println!("Resultado: {}", resultado),
         Err(error) => println!("Error: {}", error)
   } 
   match dividir(5, 0) {  
         Ok(resultado) => println!("Resultado: {}", resultado),   
         Err(error) => println!("Error: {}", error)
   }
}
```

---

## 🤔 Valores Opcionales

### ❓ Option\<T>

El tipo `Option` representa un valor que **puede existir o no**. ¡Adiós a los valores null!

```plaintext
fn buscar_usuario(id: u32) -> Option<&'static str> {   
   if id == 1 { 
      Some("Juan") 
   } 
   else {
        None   
   }
} 
fn main() {   
   match buscar_usuario(1) { 
         Some(nombre) => println!("Usuario encontrado: {}", nombre), 
         None => println!("Usuario no encontrado")
   }
   match buscar_usuario(2) {
         Some(nombre) => println!("Usuario encontrado: {}", nombre),
         None => println!("Usuario no encontrado")
   } 
}
```

---

## 📚 Recursos Adicionales

### 🌟 ¿Quieres profundizar más en Rust?

Consulta **The Rust Book** en español, la guía oficial y más completa para aprender Rust:

### 📖 [The Rust Book en español](https://book.rustlang-es.org)

---

## 🧭 Navegación

🏘️ [**Volver al Principal**](./README.md)
