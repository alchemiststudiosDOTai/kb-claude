---
title: CLI Module Root
path: src/cli/mod.rs
type: file
depth: 2
description: Command-line interface definition and argument parsing
exports: [Cli, Command, run, execute, InitArgs, NewArgs, SearchArgs, LinkArgs, ValidateArgs, ManifestArgs]
seams: [M]
---

## Where
`src/cli/mod.rs`

## What
Central entry point for the CLI, defining command structure, parsing arguments via `clap`, and dispatching to subcommand implementations.

## How

### Module Organization
Declares submodules for each command:
```rust
mod init;
mod link;
mod manifest;
mod new;
mod search;
mod validate;
```

### Core Structures

**`Cli` Struct**
- Derived from `clap::Parser`
- Top-level application definition
- Fields:
  - `command: Command`: The invoked subcommand
- Global settings: name, version, description

**`Command` Enum**
- Derived from `clap::Subcommand`
- Variants for each subcommand:
  - `Init(InitArgs)`
  - `New(NewArgs)`
  - `Search(SearchArgs)`
  - `Link(LinkArgs)`
  - `Validate(ValidateArgs)`
  - `Manifest(ManifestArgs)`

### Argument Structs
Each variant holds a specific `*Args` struct defining that command's arguments:

**`InitArgs`**
- `--directory <PATH>`: Custom location
- `--dry-run`: Preview without execution

**`NewArgs`**
- `<TITLE>`: Positional, required
- `-t, --type <TYPE>`: Document category
- `-g, --tag <TAG>`: Repeatable tags
- `--relates-to <LINK>`: Repeatable relations
- `-f, --file <PATH>`: Output path override

**`SearchArgs`**
- `<TERM>...`: Positional, required, one or more
- `-t, --tag <TAG>`: Repeatable tag filter

**`LinkArgs`**
- `<SOURCE>`: First document slug
- `<TARGET>`: Second document slug
- `--force`: Overwrite existing relations

**`ValidateArgs`**
- `--directory <PATH>`: Custom KB location
- `--strict`: Treat warnings as errors

**`ManifestArgs`**
- `--output <PATH>`: Custom output file
- `-d, --directory <PATH>`: KB location

### Execution Flow

**`run()` Function**
- Public entry point
- Calls `Cli::parse()` to deserialize arguments
- Returns `anyhow::Result` for error propagation

**`execute()` Function**
- Takes parsed `Cli` struct
- `match` on `cli.command` to determine subcommand
- Dispatches to appropriate module's `run()`:
  ```rust
  match cli.command {
      Command::Init(args) => init::run(args),
      Command::New(args) => new::run(args),
      // ...
  }
  ```

## Why
**Declarative CLI Definition**: `clap` attributes make the interface self-documenting, reducing boilerplate and ensuring help messages stay in sync with code.

**Modular Architecture**: Each subcommand lives in its own module, preventing monolithic growth and enabling independent development/testing.

**Separation of Concerns**: `mod.rs` handles parsing/dispatch, while subcommand modules handle business logic. This makes the codebase easier to navigate and maintain.

**Type Safety**: Rust's enum variant pattern ensures all possible commands are handled at compile time, preventing runtime dispatch errors.

**Scalability**: Adding new commands requires:
1. Create new module file
2. Define `*Args` struct
3. Add variant to `Command` enum
4. Add match arm in `execute()`

This predictable pattern makes onboarding and code review straightforward.

**Error Handling**: `anyhow::Result` provides flexible error propagation with context, making debugging easier for both developers and users.
