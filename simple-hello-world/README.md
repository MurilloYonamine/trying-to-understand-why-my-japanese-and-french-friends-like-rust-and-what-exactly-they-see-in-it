# Simple Hello World
This project demonstrates how to compile and run a simple Rust program.

Compiling manually
Compile the main file directly with rustc:

```bash
rustc main.rs
```

## Using Cargo (recommended)

Cargo is Rust's official project management tool. It is highly recommended to use it.

Create a new project
```bash
cargo new hello_cargo
```

Create a new project without Git
```bash
cargo new hello_cargo --vcs none
```

Build the project
```bash
cargo build
```

Build and run the project
```bash
cargo run
```

Check the project (without generating a binary)
```bash
cargo check
```

Notes about Cargo
- The `cargo new` command creates a new Rust project.
- The `cargo build` command compiles the project.
- The `cargo run` command compiles and runs the project in one step.
- The `cargo check` command checks for errors without generating the binary.
- Compiled files are stored in `target/debug` instead of the source code directory.

## Reference

This README is based on the official Rust Book: [Getting Started](https://doc.rust-lang.org/stable/book/ch01-00-getting-started.html)