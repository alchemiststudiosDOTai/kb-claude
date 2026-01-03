---
title: CLI Subdirectory
path: src/cli/
type: directory
depth: 2
description: Command-line interface implementations for all kb-claude subcommands
seams: [S]
---

# CLI Subdirectory (`src/cli/`)

## Purpose
Implements the command-line interface layer, parsing user arguments and dispatching to appropriate business logic. Each subcommand lives in its own module following the "one file per action" pattern from `CLAUDE.md`.

## Organization

### Module Root
- **`mod.rs`** - CLI orchestration and argument definition
  - `Cli` struct: Top-level clap-derived argument parser
  - `Command` enum: Variants for each subcommand (Init, New, Link, Manifest, Validate, Search)
  - `run()` function: Main entry point that matches on `Command` and calls appropriate handler
  - Defines global flags like `--dry-run`, `-v` (verbose)

### Subcommand Modules

#### `init.rs` - Knowledge Base Initialization
- Creates `.claude/` directory structure with all required subdirectories
- Sets up initial `manifest.md` if needed
- Validates directory doesn't already exist
- **User-facing**: `kb-claude init`

#### `new.rs` - Create New Entry
- Prompts for entry type (metadata, debug_history, qa, code_index, pattern, plan, cheatsheet, memory_anchor)
- Generates YAML front matter with UUID, timestamp, title
- Opens editor for user input
- Creates file in appropriate `.claude/` subdirectory
- **User-facing**: `kb-claude new [-t <type>]`

#### `link.rs` - Create Cross-References
- Adds `ontological_relations` to front matter
- Links related documents by UUID or path
- Updates bidirectional relationships
- **User-facing**: `kb-claude link <doc> --relates-to <target>`

#### `manifest.rs` - Generate Manifest
- Walks all `.claude/` directories
- Parses front matter from each markdown file
- Generates `manifest.md` with searchable index
- **User-facing**: `kb-claude manifest [--update]`

#### `validate.rs` - Schema Validation
- Checks YAML front matter structure
- Verifies required fields (title, type, created_at, uuid)
- Validates ontological relations reference existing documents
- **User-facing**: `kb-claude validate [--strict]`

#### `search.rs` - Search Knowledge Base
- Searches document titles, content, and tags
- Filters by entry type
- Displays relative paths and excerpts
- **User-facing**: `kb-claude search <query> [-t <type>] [-d <dir>]`

## Naming Conventions

### File Naming
- Each file matches its subcommand name: `init.rs`, `new.rs`, `link.rs`, etc.
- All lowercase with underscores
- `mod.rs` for module root (standard Rust pattern)

### Code Patterns
- Handler functions typically named `run_command_name()` or similar
- Argument structs named `<Command>Args` (e.g., `InitArgs`, `NewArgs`)
- Error handling via `anyhow::Result<T>` or custom error types

## Architectural Patterns

### Command Handler Pattern
Each subcommand file typically contains:
1. Command-specific argument struct (if complex)
2. Handler function with signature like `fn run(args: &Args) -> Result<()>`
3. Filesystem operations via `fs` module
4. Model operations via `model` module

### Error Handling
Commands use `anyhow` for error propagation:
- `bail!()` macro for errors with context
- `context()` for adding information to errors
- User-friendly error messages

### Dry-Run Support
Commands that modify state respect `--dry-run` flag:
- Preview changes without executing
- Useful for validation and testing

## CLI Argument Conventions

### Flag Naming
- Long flags use `kebab-case`: `--dry-run`, `--relates-to`, `--output`
- Short flags where logical: `-t`, `-d`, `-v`
- Consistency with `README.md` documentation

### Subcommand Structure
```
kb-claude <SUBCOMMAND> [FLAGS] [OPTIONS] [ARGS]

SUBCOMMANDS:
    init      Initialize knowledge base
    new       Create new entry
    link      Link related documents
    manifest  Generate manifest
    validate  Validate entries
    search    Search knowledge base

FLAGS:
    -h, --help       Print help
    -v, --verbose    Verbose output
    --dry-run        Preview without changes
```

## Relationships
- **Parent**: `src/` directory
- **Siblings**: `fs.rs`, `model.rs` (utility modules used by CLI)
- **Uses**: `fs.rs` for path operations, `model.rs` for document parsing
- **Tests**: `tests/command_matrix.rs` verifies command interactions
- **Documents**: `.claude/code_index/cli-module-index.md` indexes this module

## Extension Points
Adding a new subcommand requires:
1. Create new file in `src/cli/` (e.g., `export.rs`)
2. Add variant to `Command` enum in `mod.rs`
3. Implement handler function
4. Add match arm in `run()` dispatcher
5. Add integration test in `tests/`
