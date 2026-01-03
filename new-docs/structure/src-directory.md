---
title: Source Directory
path: src/
type: directory
depth: 1
description: Core Rust source code implementing the kb-claude CLI library and binary
seams: [S]
---

# Source Directory (`src/`)

## Purpose
Contains all Rust source code for the `claude-kb-cli` crate, implementing both the library interface and binary CLI tool for knowledge base management.

## Organization

### Entry Points
- **`main.rs`** - Binary entry point. Thin wrapper that calls `claude_kb_cli::cli::run()`. Delegates all logic to the library.
- **`lib.rs`** - Library root exposing public modules: `cli`, `fs`, and `model`. Acts as orchestrator for the crate's public API.

### Core Modules
- **`model.rs`** - Domain models and business logic
  - Data structures: `OntologicalRelation`, `DocumentFrontMatter`, `Document`
  - YAML front matter parsing with `parse()` and `to_markdown()` methods
  - Utility functions: `slugify()`, custom `iso8601` serde handlers for `chrono`
  - Defines the knowledge base entry schema

- **`fs.rs`** - Filesystem utilities and path management
  - Constants: `CLAUDE_ROOT`, `MANIFEST_FILE`, `CLAUDE_DIRECTORIES`
  - Path resolution: `claude_root_from()`, `find_existing_root()`, `resolve_claude_root_from_cwd()`
  - Layout management: `ClaudePaths` struct for KB directory structure
  - Document walking: `walk_kb_documents()` for traversal
  - Display utilities: `display_relative()` for user-friendly paths

### CLI Subdirectory (`src/cli/`)
Command-line interface implementation with one-file-per-subcommand pattern:

- **`mod.rs`** - CLI module root
  - `Cli` struct using `clap` derive macro for argument parsing
  - `Command` enum defining all subcommands (Init, New, Link, Manifest, Validate, Search)
  - `run()` function dispatching to subcommand implementations

- **Subcommand Implementations** (each in dedicated file):
  - **`init.rs`** - Initialize new `.claude/` knowledge base directory structure
  - **`new.rs`** - Create new knowledge base entries with YAML front matter
  - **`link.rs`** - Create cross-references between related documents
  - **`manifest.rs`** - Generate `manifest.md` summary of all KB entries
  - **`validate.rs`** - Verify front matter schema and document integrity
  - **`search.rs`** - Search KB content and metadata

## Naming Conventions

### Files
- `snake_case.rs` for all module files
- `mod.rs` for module roots
- One file per CLI subcommand matching command name

### Code
- **Functions**: `snake_case` (e.g., `run`, `normalize_workspace`, `slugify`)
- **Structs**: `PascalCase` (e.g., `Cli`, `Command`, `DocumentFrontMatter`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `CLAUDE_ROOT`, `MANIFEST_FILE`)
- **Enums**: `PascalCase` (e.g., `ReportMode`, `Severity`)
- **Traits**: `PascalCase` (implied for trait definitions)

### Naming Patterns
- `_from` suffix: Functions deriving from source (`claude_root_from`, `slug_from_title`)
- `_relative` suffix: Path conversion functions (`display_relative`)
- `_error` suffix: Error constants (`CURRENT_DIR_ERROR`, `NO_CLAUDE_DIR_ERROR`)

## Architectural Decisions

### Separation of Concerns
- Domain logic in `model.rs`
- Filesystem abstraction in `fs.rs`
- CLI orchestration in `cli/mod.rs`
- Command implementations isolated in separate files

### Thin Binary Pattern
`main.rs` is minimal (~3 lines), making the library fully testable and reusable. All actual logic lives in `lib.rs` and submodules.

### Modularity
Each CLI command is an independent module, enabling:
- Easy addition of new commands
- Isolated testing per command
- Clear code organization matching user-facing commands

## Relationships
- **Parent**: Root directory (`/`)
- **Sibling to**: `tests/` (integration tests verify `src/` behavior)
- **Imports from**: External crates defined in `Cargo.toml` (clap, serde, chrono, etc.)
- **Operates on**: `.claude/` knowledge base directory structure
- **Documented in**: `.claude/code_index/cli-module-index.md`
