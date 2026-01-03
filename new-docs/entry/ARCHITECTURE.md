---
title: Entry Points Architecture Summary
path: new-docs/entry/ARCHITECTURE.md
type: file
depth: 0
description: Visual architecture overview of all entry points
exports: []
seams: []
---

# Entry Points Architecture Summary

## Quick Reference

### Binary Entry
- **File**: `/Users/tuna/kb-claude/src/main.rs`
- **Function**: `main()`
- **Calls**: `claude_kb_cli::cli::run()`

### Library Entry Points
- **File**: `/Users/tuna/kb-claude/src/lib.rs`
- **Modules**: `cli`, `fs`, `model`

### CLI Entry Points
- **File**: `/Users/tuna/kb-claude/src/cli/mod.rs`
- **Functions**: `run()`, `execute()`
- **Structs**: `Cli`, `Command`, `*Args`

### Subcommand Entry Points
- **Directory**: `/Users/tuna/kb-claude/src/cli/`
- **Pattern**: Each module has `pub fn run(args: *Args) -> Result<()>`
- **Commands**: `init`, `new`, `search`, `link`, `validate`, `manifest`

## Call Flow Diagram

```
User Command
    ↓
kb-claude (binary)
    ↓
main.rs → main()
    ↓
claude_kb_cli::cli::run()
    ↓
parse() → Cli struct
    ↓
execute(Cli)
    ↓
match Command enum
    ↓
┌─────────────────────────────────────────┐
│  Subcommand Dispatch                    │
├─────────────────────────────────────────┤
│  Init(InitArgs)     → init::run()      │
│  New(NewArgs)       → new::run()        │
│  Search(SearchArgs) → search::run()     │
│  Link(LinkArgs)     → link::run()       │
│  Validate(ValidateArgs) → validate::run() │
│  Manifest(ManifestArgs) → manifest::run() │
└─────────────────────────────────────────┘
    ↓
Use fs:: and model:: modules
    ↓
Manipulate .claude/ files
```

## Module Dependency Graph

```
┌─────────────────────────────────────────────────────┐
│                   src/main.rs                       │
│                   (binary only)                     │
└────────────────────┬────────────────────────────────┘
                     │
                     ↓
┌─────────────────────────────────────────────────────┐
│                   src/lib.rs                        │
│              (public library API)                   │
├─────────────────────────────────────────────────────┤
│  pub mod cli    │  pub mod fs  │  pub mod model    │
└──────┬──────────┴──────┬───────┴───────┬───────────┘
       │                 │               │
       ↓                 ↓               ↓
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│  cli/mod.rs  │  │   fs.rs      │  │  model.rs    │
│              │  │              │  │              │
│ - run()      │  │ - paths     │  │ - Document   │
│ - execute()  │  │ - walk      │  │ - FrontMatter│
│ - Cli        │  │ - constants │  │ - Relations  │
│ - Command    │  │ - util fns  │  │ - parse()    │
│ - *Args      │  │              │  │ - serialize │
└──────┬───────┘  └──────────────┘  └──────────────┘
       │
       ↓
┌─────────────────────────────────────────────────────┐
│              Subcommand Modules                     │
├───────────┬───────────┬───────────┬────────────────┤
│ init.rs   │ new.rs    │ search.rs │ link.rs        │
│ validate  │ manifest  │           │                │
└───────────┴───────────┴───────────┴────────────────┘
```

## Public API Surface

### CLI Module (`claude_kb_cli::cli`)
```rust
pub fn run() -> Result<()>
pub fn execute(cli: Cli) -> Result<()>
pub struct Cli         // Top-level args
pub enum Command       // Subcommands
pub struct InitArgs    // Init command args
pub struct NewArgs     // New command args
pub struct SearchArgs  // Search command args
pub struct LinkArgs    // Link command args
pub struct ValidateArgs // Validate command args
pub struct ManifestArgs // Manifest command args
```

### Filesystem Module (`claude_kb_cli::fs`)
```rust
// Constants
pub const CLAUDE_ROOT: &str
pub const MANIFEST_FILE: &str
pub const CLAUDE_DIRECTORIES: &[&str]

// Structs
pub struct ClaudePaths
pub struct DocumentEntry

// Functions
pub fn is_ignored_path(...) -> bool
pub fn claude_root_from(...) -> PathBuf
pub fn find_existing_root(...) -> Option<PathBuf>
pub fn resolve_claude_root_from_cwd() -> Result<(PathBuf, PathBuf)>
pub fn resolve_claude_root(...) -> Result<(PathBuf, PathBuf)>
pub fn display_relative(...) -> String
pub fn walk_kb_documents(...) -> impl Iterator<Item = Result<DocumentEntry>>
```

### Model Module (`claude_kb_cli::model`)
```rust
// Constants
pub const FRONT_MATTER_DELIMITER: &str

// Structs
pub struct OntologicalRelation
pub struct DocumentFrontMatter
pub struct Document

// Functions
pub fn slugify(input: &str) -> String

// Module
pub mod iso8601  // DateTime ser/de
```

## Command-Line Interface

### Usage Pattern
```bash
kb-claude <SUBCOMMAND> [OPTIONS] [ARGS]
```

### Available Subcommands

#### 1. Initialize
```bash
kb-claude init [DIRECTORY] [--dry-run]
```

#### 2. Create Entry
```bash
kb-claude new <TITLE> [--type TYPE] [--tag TAG...] [--relates-to UUID...]
```

#### 3. Search
```bash
kb-claude search [TERMS...] [--tag TAG...]
```

#### 4. Link Documents
```bash
kb-claude link <SOURCE_UUID> <TARGET_UUID> [--force]
```

#### 5. Validate
```bash
kb-claude validate [DIRECTORY] [--strict]
```

#### 6. Rebuild Manifest
```bash
kb-claude manifest [--output PATH] [--directory DIR]
```

## Data Flow

### Creating a New Document
```
user input → NewArgs → new::run()
    → model::DocumentFrontMatter::new()
    → generate uuid, slug, timestamps
    → model::Document::new()
    → fs::ClaudePaths::type_directory()
    → write markdown file
```

### Searching Documents
```
search terms → SearchArgs → search::run()
    → fs::walk_kb_documents()
    → model::Document::parse()
    → filter by terms/tags
    → display results
```

### Validating
```
validate → ValidateArgs → validate::run()
    → fs::walk_kb_documents()
    → model::Document::parse()
    → check front matter consistency
    → verify links
    → report errors
```

## Key Exports Summary

| Category | Exports | Location |
|----------|---------|----------|
| Binary | `main` | `src/main.rs` |
| CLI | `run`, `execute`, `Cli`, `Command`, `*Args` | `src/cli/mod.rs` |
| Filesystem | `ClaudePaths`, `walk_kb_documents`, constants | `src/fs.rs` |
| Model | `Document`, `DocumentFrontMatter`, `OntologicalRelation` | `src/model.rs` |
| Library | `cli`, `fs`, `model` modules | `src/lib.rs` |

## File Locations

- **Binary**: `/Users/tuna/kb-claude/src/main.rs`
- **Library**: `/Users/tuna/kb-claude/src/lib.rs`
- **CLI**: `/Users/tuna/kb-claude/src/cli/mod.rs`
- **Subcommands**: `/Users/tuna/kb-claude/src/cli/*.rs`
- **Filesystem**: `/Users/tuna/kb-claude/src/fs.rs`
- **Model**: `/Users/tuna/kb-claude/src/model.rs`
- **Tests**: `/Users/tuna/kb-claude/tests/*.rs`

## Documentation Files

All entry point documentation is located in:
```
/Users/tuna/kb-claude/new-docs/entry/
├── INDEX.md                      # This file
├── ARCHITECTURE.md               # Architecture overview
├── binary-entry-point.md         # Binary entry details
├── cli-module-entry.md           # CLI module details
├── cli-subcommands.md            # Subcommand details
├── library-public-api.md         # Library API details
├── filesystem-module-api.md      # Filesystem API details
└── data-model-api.md             # Data model details
```

## No Web/API Layer

This is a pure CLI application with:
- No HTTP endpoints
- No web routes
- No REST API
- No server components

All interaction is through:
- Command-line arguments
- File system operations
- Standard input/output
