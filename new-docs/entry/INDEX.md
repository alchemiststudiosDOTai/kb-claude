---
title: Entry Points Documentation Index
path: new-docs/entry/
type: directory
depth: 0
description: Index of all entry points and public interfaces
exports: []
seams: []
---

# Entry Points Documentation Index

This directory contains comprehensive documentation of all entry points, CLI commands, and public interfaces for the `kb-claude` project.

## Documentation Files

### 1. [Binary Entry Point](binary-entry-point.md)
**File**: `src/main.rs`
- Binary bootstrap that calls the library CLI
- Thin wrapper following binary-as-thin-wrapper pattern
- Minimal responsibility - forwards to `claude_kb_cli::cli::run()`

### 2. [CLI Module Entry Point](cli-module-entry.md)
**File**: `src/cli/mod.rs`
- Main CLI execution and command dispatch
- Uses `clap` v4 for argument parsing
- Defines `Cli`, `Command` enum, and all `*Args` structs
- Routes to individual subcommand modules

### 3. [CLI Subcommands](cli-subcommands.md)
**Directory**: `src/cli/`
- Individual command implementations
- Six subcommands: `init`, `new`, `search`, `link`, `validate`, `manifest`
- Each has dedicated module with `run()` function
- Consistent pattern: `*Args` struct → `run()` → `Result<()>`

### 4. [Library Public API](library-public-api.md)
**File**: `src/lib.rs`
- Exposes three public modules: `cli`, `fs`, `model`
- No web/API layer - pure CLI application
- Designed for CLI-first optimization and testability

### 5. [Filesystem Module API](filesystem-module-api.md)
**File**: `src/fs.rs`
- Path management and directory layout
- Document discovery and traversal
- Constants for `.claude` structure
- `ClaudePaths` struct for path operations
- `walk_kb_documents()` iterator for document traversal

### 6. [Data Model API](data-model-api.md)
**File**: `src/model.rs`
- Core data structures for knowledge documents
- `DocumentFrontMatter` - YAML metadata
- `Document` - Full document (front matter + body)
- `OntologicalRelation` - Cross-references
- Serialization/deserialization with serde and YAML

## Entry Point Architecture

```
kb-claude (binary)
└── main.rs
    └── claude_kb_cli::cli::run() [lib.rs → cli/mod.rs]
        ├── Parse CLI arguments (clap)
        └── cli::execute()
            └── Dispatch to subcommands:
                ├── init::run()
                ├── new::run()
                ├── search::run()
                ├── link::run()
                ├── validate::run()
                └── manifest::run()
```

## Public API Structure

```rust
// Library modules
pub mod cli;      // CLI definitions and execution
pub mod fs;       // Filesystem utilities
pub mod model;    // Data structures

// Main CLI entry
claude_kb_cli::cli::run()

// Key types
claude_kb_cli::cli::Cli
claude_kb_cli::cli::Command
claude_kb_cli::fs::ClaudePaths
claude_kb_cli::fs::DocumentEntry
claude_kb_cli::model::Document
claude_kb_cli::model::DocumentFrontMatter
```

## CLI Commands Overview

| Command | Purpose | Module |
|---------|---------|--------|
| `init` | Initialize `.claude/` directory | `src/cli/init.rs` |
| `new` | Create knowledge entry | `src/cli/new.rs` |
| `search` | Search entries | `src/cli/search.rs` |
| `link` | Create cross-references | `src/cli/link.rs` |
| `validate` | Check metadata consistency | `src/cli/validate.rs` |
| `manifest` | Rebuild manifest.md | `src/cli/manifest.rs` |

## Key Design Patterns

1. **Binary-as-Thin-Wrapper**: Minimal binary, logic in library
2. **Command Pattern**: Each subcommand is independent module
3. **Iterator Pattern**: Lazy document traversal
4. **Builder Pattern**: Struct constructors with sensible defaults
5. **Serialization First**: All data structures support serde

## Technologies Used

- **CLI Framework**: `clap` v4 (derive macros)
- **Serialization**: `serde`, `serde_yaml`
- **Error Handling**: `anyhow`
- **Date/Time**: `chrono` with custom ISO8601 serializers
- **UUIDs**: `uuid` crate

## Integration Points

- **Filesystem**: `.claude/` directory structure
- **Data Format**: Markdown with YAML front matter
- **Entry Types**: 9 supported types (metadata, debug_history, qa, code_index, patterns, plans, cheatsheets, memory_anchors, other)
