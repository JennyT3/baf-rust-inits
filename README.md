# Mi Primer Contrato en Rust con Soroban 🦀
Este proyecto forma parte de la **Guía Intensiva de Rust - Día 1**, enfocada en la instalación de Rust y la creación de smart contracts básicos usando Soroban. Incluye un programa simple de suma y un smart contract avanzado para verificar números primos.
# Requisitos Previos
Si quieres seguir mi progreso, necesitarás Rust Lo instalé así (y tú también puedes):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
### Dependencias para Smart Contracts:

```sh
rustup target add wasm32-unknown-unknown
cargo install --locked stellar-cli --features opt
```
### Configurar un nuevo proyecto Rust:

```sh
cargo new mi_primer_contrato
cd mi_primer_contrato
```
Un editor de código VS Code con la extensión de Rust

# Estructura del Proyecto

```sh
mi_primer_contrato/
├── Cargo.toml          # Configuración
├── README.md           # Presentación 
└── src/
    └── lib.rs          # Mi smart contract
    └── main.rs         # Programa de suma

```
# Instalación
1. Inicié un nuevo proyecto Rust:
```sh
cargo new mi_primer_contrato
cd mi_primer_contrato
```
2. Configuré Cargo.toml para Soroban

```toml
[package]
name = "mi_primer_contrato"
version = "0.1.0"
edition = "2021"


[dependencies]
soroban-sdk = "0.8.0"

[lib]
crate-type = ["cdylib"]
```

# Lo Que Construí
1. Programa de Suma
Empecé con algo simple pero sólido: un programa que suma dos números:
Ubicación: src/main.rs





