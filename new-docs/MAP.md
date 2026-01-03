# Codebase Map - kb-claude

**Generated**: 2025-01-03
**Project**: kb-claude (claude-kb-cli v0.3.4)
**Type**: Rust CLI Application
**Purpose**: Knowledge base management tool for Markdown + YAML front matter

---

## Statistics

| Metric | Value |
|--------|-------|
| Total files analyzed | 32 |
| Directories documented | 8 |
| Documentation files generated | 40 |
| SEAMS agents spawned | 5 |
| Documentation size | 352 KB |
| Source LOC | ~1,437 lines |

---

## Structure Overview

```
kb-claude/
├── src/                          # Source code (11 files, 1,437 LOC)
│   ├── main.rs                   # Binary entry point (2 LOC)
│   ├── lib.rs                    # Library root (4 LOC)
│   ├── model.rs                  # Domain models (163 LOC)
│   ├── fs.rs                     # Filesystem utilities (183 LOC)
│   └── cli/                      # CLI commands (6 subcommands)
│       ├── mod.rs                # Command dispatcher (171 LOC)
│       ├── init.rs               # Initialize KB (91 LOC)
│       ├── new.rs                # Create entries (211 LOC)
│       ├── search.rs             # Search content (125 LOC)
│       ├── link.rs               # Link documents (115 LOC)
│       ├── validate.rs           # Validate integrity (232 LOC)
│       └── manifest.rs           # Generate index (140 LOC)
│
├── tests/                        # Integration tests (2 files)
│   ├── smoke.rs                  # Basic functionality tests
│   └── command_matrix.rs         # Comprehensive command tests
│
├── .claude/                      # Knowledge base (8 categories)
│   ├── metadata/                 # Component summaries
│   ├── debug_history/            # Debugging timelines
│   ├── qa/                       # Q&A and learning notes
│   ├── code_index/               # File references
│   ├── patterns/                 # Reusable solutions
│   ├── plans/                    # Project plans
│   ├── cheatsheets/              # Quick references
│   ├── memory_anchors/           # Core concepts (UUIDs)
│   └── manifest.md               # Auto-generated index
│
├── memory-bank/                  # Agent workflow tracking
│   ├── research/                 # Investigation phase
│   ├── plan/                     # Planning phase
│   └── execute/                  # Execution phase
│
└── new-docs/                     # This documentation (40 files)
    ├── structure/                # Directory organization (8 files)
    ├── entry/                    # Entry points (8 files)
    ├── architecture/             # Design patterns (7 files)
    ├── modules/                  # Module documentation (12 files)
    └── state/                    # State management (5 files)
```

---

## Detailed Mapping

| Path | Type | Depth | Purpose | Key Exports |
|------|------|-------|---------|-------------|
| **Source Code** |||||
| `src/main.rs` | file | 1 | Binary entry point | `main` |
| `src/lib.rs` | file | 1 | Library root | `cli`, `fs`, `model` |
| `src/model.rs` | file | 1 | Domain models | `Document`, `DocumentFrontMatter`, `OntologicalRelation` |
| `src/fs.rs` | file | 1 | Filesystem utilities | `ClaudePaths`, `walk_kb_documents`, `resolve_claude_root` |
| `src/cli/mod.rs` | file | 1 | CLI dispatcher | `Cli`, `Command`, `run`, `execute` |
| `src/cli/init.rs` | file | 2 | Initialize KB | `run` |
| `src/cli/new.rs` | file | 2 | Create entries | `run` |
| `src/cli/search.rs` | file | 2 | Search content | `run` |
| `src/cli/link.rs` | file | 2 | Link documents | `run` |
| `src/cli/validate.rs` | file | 2 | Validate KB | `run` |
| `src/cli/manifest.rs` | file | 2 | Generate index | `run` |
| **Tests** |||||
| `tests/smoke.rs` | file | 1 | Basic functionality tests | Test functions |
| `tests/command_matrix.rs` | file | 1 | Command interaction tests | Test functions |
| **Knowledge Base** |||||
| `.claude/metadata/` | dir | 1 | Component summaries | - |
| `.claude/debug_history/` | dir | 1 | Debugging timelines | - |
| `.claude/qa/` | dir | 1 | Q&A notes | - |
| `.claude/code_index/` | dir | 1 | File references | - |
| `.claude/patterns/` | dir | 1 | Reusable solutions | - |
| `.claude/plans/` | dir | 1 | Project plans | - |
| `.claude/cheatsheets/` | dir | 1 | Quick references | - |
| `.claude/memory_anchors/` | dir | 1 | Core concepts | - |

---

## SEAMS Summary

### **S**tructure

**Organization Pattern**: Clean layered architecture with clear separation of concerns.

**Key Features**:
- **Thin Binary**: `main.rs` is minimal (~3 lines), all logic in library
- **One File Per Command**: Each CLI subcommand gets dedicated module
- **Integration-First Testing**: Tests drive actual binary via `assert_cmd`
- **Type-Based Organization**: Knowledge base categorized by semantic type

**Directory Relationships**:
```
main.rs → lib.rs → cli/mod.rs → [init|new|search|link|validate|manifest].rs
                    ↓
                  fs.rs + model.rs
```

**Naming Conventions**:
- Rust: `snake_case` functions, `PascalCase` types
- Files: `snake_case.rs` for modules, `kebab-case.md` for KB entries
- Directories: `lowercase` semantic names
- CLI: `lowercase` commands, `--kebab-case` flags

---

### **E**ntry Points

**Binary Entry**: `src/main.rs:main()` → `claude_kb_cli::cli::run()`

**CLI Commands** (6 total):

| Command | Module | Purpose |
|---------|--------|---------|
| `init` | `cli/init.rs` | Initialize `.claude/` directory structure |
| `new` | `cli/new.rs` | Create new knowledge entries with UUID + timestamp |
| `search` | `cli/search.rs` | Search by keywords and tags |
| `link` | `cli/link.rs` | Create bidirectional `ontological_relations` |
| `validate` | `cli/validate.rs` | Check metadata consistency |
| `manifest` | `cli/manifest.rs` | Generate `manifest.md` index |

**Public Library API** (`src/lib.rs`):
```rust
pub mod cli;      // CLI definitions and execution
pub mod fs;       // Filesystem utilities
pub mod model;    // Data structures
```

**Key Public Interfaces**:
- `claude_kb_cli::cli::run()` - Main CLI entry
- `claude_kb_cli::fs::ClaudePaths` - Path management
- `claude_kb_cli::fs::walk_kb_documents()` - Document iterator
- `claude_kb_cli::model::Document` - Domain object
- `claude_kb_cli::model::DocumentFrontMatter` - Metadata schema

---

### **A**rchitecture

**Architecture Quality**: 9.2/10 (Excellent)

**Style**: Layered Architecture with unidirectional dependencies

```
┌─────────────────────────────────────┐
│  Presentation (CLI)                 │  main.rs, cli/mod.rs
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│  Application                        │  cli/*.rs (commands)
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│  Domain                             │  model.rs (zero deps)
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│  Infrastructure                     │  fs.rs (persistence)
└─────────────────────────────────────┘
```

**Design Patterns Identified**:
1. **Command Pattern** - CLI subcommands
2. **Repository Pattern** - `fs.rs` data access
3. **DTO Pattern** - `DocumentFrontMatter` serialization
4. **Strategy Pattern** - Enum-based behavior
5. **Factory Pattern** - Document constructors
6. **Iterator Pattern** - Lazy document traversal

**Key Architectural Decisions**:
1. Markdown + YAML front matter for VC-friendly storage
2. Integration-only testing (no unit tests)
3. `anyhow` for ergonomic error handling
4. Type-based directory organization
5. Slugified human-readable links
6. Strict validation with warnings
7. Auto-initialization for low friction

**Dependencies** (7 production crates):
- `clap` v4.5 - CLI parsing
- `serde` + `serde_yaml` - Serialization
- `chrono` - Timestamps
- `uuid` - Unique identifiers
- `walkdir` - File traversal
- `anyhow` - Error handling

---

### **M**odules

**Module Structure** (11 modules, 1,437 LOC):

| Module | LOC | Purpose | Dependencies |
|--------|-----|---------|--------------|
| `model.rs` | 163 | Domain models, YAML schema | None |
| `fs.rs` | 183 | Path operations, document discovery | model |
| `cli/mod.rs` | 171 | Command parser, dispatcher | subcommands |
| `cli/init.rs` | 91 | Initialize KB | fs, model |
| `cli/new.rs` | 211 | Create documents | fs, model |
| `cli/search.rs` | 125 | Search content | fs, model |
| `cli/link.rs` | 115 | Link documents | fs, model |
| `cli/validate.rs` | 232 | Validate integrity | fs, model |
| `cli/manifest.rs` | 140 | Generate index | fs, model |
| `main.rs` | 2 | Binary entry | cli |
| `lib.rs` | 4 | Library root | cli, fs, model |

**Dependency Graph**:
```
main.rs
 └─> lib.rs
      ├─> cli/mod.rs
      │    ├─> init.rs ──┐
      │    ├─> new.rs ───┤
      │    ├─> search.rs ┤
      │    ├─> link.rs ───┼──> fs.rs ──> model.rs
      │    ├─> validate.rs│
      │    └─> manifest.rs┘
      ├─> fs.rs ──────────────────────> model.rs
      └─> model.rs (zero dependencies)
```

**Separation of Concerns**:
- **model**: What (data structures)
- **fs**: Where (persistence)
- **cli**: How (user interface)

---

### **S**tate

**State Management Philosophy**: Minimalist, explicit

| Aspect | Approach | Rationale |
|--------|----------|-----------|
| Global State | Immutable constants only | Thread safety |
| State Containers | Struct-based ownership | Rust idioms |
| Data Persistence | Filesystem (Markdown + YAML) | Human-readable |
| Caching | Lazy iterators | Simplicity |
| Concurrency | None (CLI pattern) | Single process |

**Global Constants** (immutable):
- `CLAUDE_ROOT` - `.claude/` directory name
- `CLAUDE_DIRECTORIES` - Whitelist of 8 valid subdirectories
- `DEFAULT_EXTENSION` - `.md` file extension
- `FRONT_MATTER_DELIMITER` - `---` for YAML

**State Containers**:
- `Document` - Full document (front matter + body)
- `DocumentFrontMatter` - Metadata with UUID, timestamp, tags
- `ClaudePaths` - Path resolution context
- CLI argument structs - Runtime configuration

**Data Flow**:
```
CLI Args → Path Resolution → Filesystem Ops → YAML Parse
→ Domain Objects → Processing → Serialize → File Write
```

**Performance Characteristics**:
- Create: O(1) - Single file write
- Search: O(d × t × m) - d=docs, t=terms, m=size
- Validate: O(d) - Linear scan
- Manifest: O(d log d) - Sorting overhead

---

## Technology Stack

### Language & Tooling
- **Language**: Rust 2021 edition
- **Package Manager**: Cargo
- **Testing**: `assert_cmd`, `assert_fs`, `predicates`

### Core Dependencies
| Crate | Version | Purpose |
|-------|---------|---------|
| clap | 4.5 | CLI parsing |
| serde | 1.0 | Serialization framework |
| serde_yaml | 0.9 | YAML parsing |
| chrono | 0.4 | Timestamps |
| uuid | 1.6 | Unique identifiers |
| walkdir | 2.4 | File traversal |
| anyhow | 1.0 | Error handling |

### Data Formats
- **Markdown**: Document body (human-readable)
- **YAML**: Front matter metadata
- **ISO 8601**: Timestamp format

---

## Quick Reference

### For Developers
1. **Start here**: `src/model.rs` - Understand data structures
2. **Filesystem**: `src/fs.rs` - Path operations and traversal
3. **CLI structure**: `src/cli/mod.rs` - Command definitions
4. **Guidelines**: `CLAUDE.md` - Development practices

### For Contributors
1. **Adding commands**: Create `src/cli/<command>.rs`, update `mod.rs`
2. **Adding types**: Update `model.rs`, `fs.rs`, `validate.rs`
3. **Testing**: Add to `tests/` using `assert_cmd`
4. **Conventions**: Follow `snake_case` files, `PascalCase` types

### For Users
1. **Quick start**: `README.md`
2. **Command help**: `kb-claude --help` or `kb-claude <command> --help`
3. **Cheatsheets**: `.claude/cheatsheets/`
4. **Search**: `kb-claude search <query>`

---

## Documentation Index

### Structure Documentation (8 files)
- [overview.md](structure/overview.md) - Comprehensive project overview
- [root-directory.md](structure/root-directory.md) - Root organization
- [src-directory.md](structure/src-directory.md) - Source code layout
- [cli-subdirectory.md](structure/cli-subdirectory.md) - CLI commands
- [claude-directory.md](structure/claude-directory.md) - Knowledge base
- [tests-directory.md](structure/tests-directory.md) - Test suite
- [memory-bank-directory.md](structure/memory-bank-directory.md) - Workflow tracking
- [new-docs-directory.md](structure/new-docs-directory.md) - This documentation

### Entry Points Documentation (8 files)
- [INDEX.md](entry/INDEX.md) - Master navigation index
- [ARCHITECTURE.md](entry/ARCHITECTURE.md) - Visual architecture diagrams
- [binary-entry-point.md](entry/binary-entry-point.md) - `src/main.rs`
- [cli-module-entry.md](entry/cli-module-entry.md) - `src/cli/mod.rs`
- [cli-subcommands.md](entry/cli-subcommands.md) - All 6 commands
- [library-public-api.md](entry/library-public-api.md) - `src/lib.rs`
- [filesystem-module-api.md](entry/filesystem-module-api.md) - `src/fs.rs`
- [data-model-api.md](entry/data-model-api.md) - `src/model.rs`

### Architecture Documentation (7 files)
- [README.md](architecture/README.md) - Navigation hub
- [overview.md](architecture/overview.md) - High-level architecture
- [design-decisions.md](architecture/design-decisions.md) - 10 decision records
- [module-structure.md](architecture/module-structure.md) - Module breakdown
- [data-flow.md](architecture/data-flow.md) - Execution flows
- [dependencies.md](architecture/dependencies.md) - External dependencies
- [ANALYSIS-SUMMARY.md](architecture/ANALYSIS-SUMMARY.md) - Executive summary

### Modules Documentation (12 files)
- [SUMMARY.md](modules/SUMMARY.md) - Module index and architecture
- [lib.md](modules/lib.md) - Library root
- [model.md](modules/model.md) - Data structures
- [fs.md](modules/fs.md) - Filesystem utilities
- [main.md](modules/main.md) - Binary entry
- [cli-mod.md](modules/cli-mod.md) - CLI router
- [cli-init.md](modules/cli-init.md) - Init command
- [cli-new.md](modules/cli-new.md) - New command
- [cli-search.md](modules/cli-search.md) - Search command
- [cli-link.md](modules/cli-link.md) - Link command
- [cli-validate.md](modules/cli-validate.md) - Validate command
- [cli-manifest.md](modules/cli-manifest.md) - Manifest command

### State Documentation (5 files)
- [README.md](state/README.md) - Comprehensive state analysis
- [global-state-stores.md](state/global-state-stores.md) - Immutable constants
- [struct-based-state-containers.md](state/struct-based-state-containers.md) - State ownership
- [caching-mechanisms.md](state/caching-mechanisms.md) - Lazy evaluation
- [data-persistence-patterns.md](state/data-persistence-patterns.md) - Filesystem storage

---

## Generated By

**Method**: SEAMS Codebase Mapping with Gemini MCP
**Agents Spawned**: 5 (Structure, Entry, Architecture, Modules, State)
**Analysis Date**: 2025-01-03
**Models Used**:
- Gemini 3 Flash Preview (discovery, modules, state)
- Gemini 2.5 Flash (entry points)
- Gemini 3 Pro Preview (architecture)

**Total Documentation**: 40 files, 352 KB
