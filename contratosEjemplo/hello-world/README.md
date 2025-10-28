**Creación del proyecto**

```plaintext
stellar contract init hello-world --name hello-world
```

**Compilación**
```plaintext
stellar contract build
```

⚠️ **Nota**

<table><tbody><tr><td><strong>Caracter</strong></td><td><strong>Sistema</strong></td><td><strong>se reemplaza por</strong></td></tr><tr><td>*</td><td>Mac y Linux<strong>&nbsp;</strong></td><td><strong>\</strong></td></tr><tr><td>*</td><td>Windows</td><td>`</td></tr><tr><td>@</td><td>Mac y Linux</td><td>/</td></tr><tr><td>@</td><td>Windows</td><td>\</td></tr></tbody></table>

**Despliegue genérico**

```plaintext
stellar contract deploy *
 --wasm target@wasm32v1-none@release@hello_world.wasm *
  --source <entity> *
  --network testnet *
  --alias hello_world
```
**En el caso de github codespace**
```plaintext
stellar contract deploy \
 --wasm target/wasm32v1-none/release/hello_world.wasm \
  --source <entity> \
  --network testnet \
  --alias hello_world
  ```

**Prueba en testnet genérico**

```plaintext
stellar contract invoke *
--id hello_world*
--source <entity> *
--network testnet *
-- *
hello *
--to "Stellar"
```

**En el caso de github codespace**
```plaintext
stellar contract invoke \
--id hello_world \
--source <entity> \
--network testnet \
-- \
hello \
--to "Stellar"
```
---
⬅️[**Contratos Ejemplo** ](../README.md) 
---

# Soroban Project

## Project Structure

This repository uses the recommended structure for a Soroban project:
```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.