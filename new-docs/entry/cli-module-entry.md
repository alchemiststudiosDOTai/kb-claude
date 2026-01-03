---
title: CLI Module Entry Point
path: src/cli/mod.rs
type: file
depth: 1
description: Main CLI execution and command dispatch
exports: [run, execute, Cli, Command, InitArgs, NewArgs, SearchArgs, LinkArgs, ValidateArgs, ManifestArgs]
seams: [E, D]
---

# CLI Module Entry Point

## File: `src/cli/mod.rs`

### Purpose
Defines the top-level CLI structure, argument parsing, and command dispatch logic using the `clap` framework.

### Public API

#### Functions
- **`run() -> Result<()>`**: Primary CLI execution entry point called from main.rs
- **`execute(cli: Cli) -> Result<()>`**: Dispatches to appropriate subcommand handler

#### Structs
- **`Cli`**: Top-level CLI argument structure (implements `clap::Parser`)
- **`InitArgs`**: Arguments for `init` subcommand
- **`NewArgs`**: Arguments for `new` subcommand
- **`SearchArgs`**: Arguments for `search` subcommand
- **`LinkArgs`**: Arguments for `link` subcommand
- **`ValidateArgs`**: Arguments for `validate` subcommand
- **`ManifestArgs`**: Arguments for `manifest` subcommand

#### Enums
- **`Command`**: Enum representing all available subcommands
  - `Init(InitArgs)`
  - `New(NewArgs)`
  - `Search(SearchArgs)`
  - `Link(LinkArgs)`
  - `Validate(ValidateArgs)`
  - `Manifest(ManifestArgs)`

### CLI Architecture

```
kb-claude (binary)
    └──> cli::run() [mod.rs]
            ├──> parse arguments
            └──> cli::execute()
                    └──> dispatch to subcommand modules:
                        ├──> init::run()
                        ├──> new::run()
                        ├──> search::run()
                        ├──> link::run()
                        ├──> validate::run()
                        └──> manifest::run()
```

### Framework
Uses `clap` v4 for:
- Declarative argument parsing via derive macros
- Automatic help generation
- Subcommand routing
- Argument validation

### Error Handling
All commands return `anyhow::Result<()>`, allowing flexible error propagation with context.
