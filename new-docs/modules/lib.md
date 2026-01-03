---
title: Library Root Module
path: src/lib.rs
type: file
depth: 1
description: Crate root that exposes public API modules
exports: [cli, fs, model]
seams: [L]
---

## Where
`src/lib.rs`

## What
The crate root for the `kb-claude` library, serving as the primary entry point and public API. It exposes three core modules that form the foundation of the knowledge base system.

## How
Declares and makes public the three main modules:
- **`cli`**: Command-line interface layer using `clap` for argument parsing
- **`fs`**: Filesystem utilities for managing `.claude` directory structure
- **`model`**: Data structures for knowledge base entries

The file follows standard Rust crate organization patterns, where `lib.rs` acts as the public facade that external crates can depend on.

## Why
**Separation of Concerns**: By exposing three distinct modules, the library maintains clear boundaries between:
- User interface (`cli`)
- Data persistence (`fs`)
- Data modeling (`model`)

**Modularity**: This design allows the core logic to be reused in different contexts (CLI binary, potential library usage, or future alternative interfaces) while keeping the implementation details encapsulated.

**Testing**: Each module can be tested independently, and the public API remains stable even as internal implementations change.
