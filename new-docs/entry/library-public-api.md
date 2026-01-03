---
title: Library Public API
path: src/lib.rs
type: file
depth: 1
description: Public library interface and module exports
exports: [cli, fs, model]
seams: [E]
---

# Library Public API

## File: `src/lib.rs`

### Purpose
Defines the public interface of the `claude-kb-cli` library crate, exposing three core modules for internal use and external consumers.

### Public Modules

```rust
pub mod cli;
pub mod fs;
pub mod model;
```

## Module Overview

### 1. `claude_kb_cli::cli`
**Location**: `src/cli/mod.rs`

**Purpose**: Command-line interface definitions and execution logic

**Key Exports**:
- `run() -> Result<()>` - Main CLI entry point
- `execute(cli: Cli) -> Result<()>` - Command dispatcher
- `Cli` struct - Top-level argument parser
- `Command` enum - Subcommand definitions
- `*Args` structs - Per-subcommand arguments

**Use Cases**:
- Binary entry point invocation
- Integration testing of CLI flows
- Programmatic CLI execution

---

### 2. `claude_kb_cli::fs`
**Location**: `src/fs.rs`

**Purpose**: Filesystem utilities for managing `.claude` directory hierarchy

**Key Exports**:
- **Constants**:
  - `CLAUDE_ROOT` - Directory name (".claude")
  - `MANIFEST_FILE` - Manifest filename ("manifest.md")
  - `CLAUDE_DIRECTORIES` - List of valid subdirectories
  - `MD_EXTENSION` - Markdown file extension

- **Structs**:
  - `ClaudePaths` - Path management for `.claude` hierarchy
  - `DocumentEntry` - Parsed document with path and content

- **Functions**:
  - `is_ignored_path()` - Path filtering logic
  - `claude_root_from()` - Construct `.claude` path
  - `find_existing_root()` - Traverse up to find `.claude`
  - `resolve_claude_root_from_cwd()` - Resolve from current directory
  - `resolve_claude_root()` - Resolve from optional base
  - `display_relative()` - Format relative paths
  - `walk_kb_documents()` - Iterate over knowledge documents

**Use Cases**:
- Directory layout management
- Document discovery and traversal
- Path resolution and validation

---

### 3. `claude_kb_cli::model`
**Location**: `src/model.rs`

**Purpose**: Data structures representing knowledge base documents

**Key Exports**:
- **Constants**:
  - `FRONT_MATTER_DELIMITER` - YAML delimiter ("---")

- **Structs**:
  - `OntologicalRelation` - Document relationship (slug, uuid, type)
  - `DocumentFrontMatter` - YAML front matter metadata
  - `Document` - Full document (front matter + body)

- **Functions**:
  - `slugify()` - Convert string to URL-friendly slug
  - `Document::parse()` - Parse Markdown into Document
  - `Document::to_markdown()` - Serialize Document to Markdown

- **Modules**:
  - `iso8601` - DateTime serialization/deserialization

**Use Cases**:
- Document parsing and serialization
- Metadata validation
- Data model for storage and retrieval

## Design Philosophy

The library API is designed for:
1. **CLI-first optimization**: Primary consumer is the binary
2. **Testability**: Public functions enable integration testing
3. **Modularity**: Clear separation between CLI, filesystem, and data model concerns
4. **Reusability**: Could be used as a library by other tools

## No Web/API Layer

This project is a pure CLI application with no HTTP endpoints or web routes. All interaction is through command-line arguments and file system operations.
