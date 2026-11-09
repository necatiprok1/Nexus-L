# Nexus-L

Nexus-L is an experimental programming-language project written in **Rust**. It explores the core stages of language implementation: lexical analysis, parsing, an abstract syntax tree, and interpretation.

## Current capabilities

The current source includes:

- A lexer that tokenizes source code
- A parser that builds a program representation
- AST-based execution through an interpreter
- Variable declarations with `let`
- `print(...)` output
- Basic arithmetic expressions and parentheses

Nexus-L is an early-stage language experiment; its feature set is intentionally described according to the implementation currently in this repository.

## Example

```nxs
let name = "Nexus"
let x = 10
let y = 20

print(name)
print(x + y)
print((2 + 3) * 4)
```

## Run an example

### Requirements

- Rust toolchain (edition 2024 compatible)

### Clone and run

```bash
git clone https://github.com/necatiprok1/Nexus-L.git
cd Nexus-L
cargo run -- run examples/hello.nxs
```

Run another included example:

```bash
cargo run -- run examples/arithmetic.nxs
cargo run -- run examples/variables.nxs
```

## Project structure

```text
.
├── examples/
│   ├── arithmetic.nxs
│   ├── hello.nxs
│   └── variables.nxs
└── src/
    ├── ast.rs
    ├── interpreter.rs
    ├── lexer.rs
    ├── main.rs
    └── parser.rs
```

## Roadmap

Nexus-L is an experimental project. Future language features and implementation details will be documented as they are developed.

---

<div align="center">
  <sub>Language design · Rust · Interpreter engineering</sub>
</div>
