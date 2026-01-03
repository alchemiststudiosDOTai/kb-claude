---
title: Architecture Overview
path: /Users/tuna/kb-claude/new-docs/architecture/overview.md
type: metadata
depth: 0
description: High-level architectural patterns and design philosophy of kb-claude
seams: ["architecture/design-decisions", "architecture/module-structure"]
---

# Architecture Overview

## Architectural Style

kb-claude employs a **clean Layered Architecture** suitable for a command-line application. The architecture is intentionally simple, emphasizing separation of concerns and maintainability over complex patterns.

### Layer Structure

```
┌─────────────────────────────────────────────────────────────┐
│                    Presentation Layer                       │
│                  (src/main.rs, cli/mod.rs)                  │
│  - CLI parsing and routing via clap                        │
│  - User interaction and feedback                           │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                   Application Layer                         │
│                  (src/cli/init.rs, etc.)                    │
│  - Command-specific business logic                         │
│  - Workflow orchestration                                  │
│  - User input validation                                   │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                    Domain Layer                             │
│                     (src/model.rs)                          │
│  - Core data structures (Document, DocumentFrontMatter)    │
│  - Business rules and validation                           │
│  - Serialization/deserialization                           │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                 Infrastructure Layer                        │
│                      (src/fs.rs)                            │
│  - Filesystem operations                                   │
│  - Path resolution and management                          │
│  - Document discovery and traversal                        │
└─────────────────────────────────────────────────────────────┘
```

## Key Architectural Principles

### 1. Separation of Concerns
Each module has a single, well-defined responsibility:
- **model.rs**: Defines what a document is
- **fs.rs**: Handles how documents are stored and retrieved
- **cli/** modules**: Orchestrate user-facing operations

### 2. Unidirectional Dependencies
Dependencies flow in one direction: `cli → fs → model`
- The `model` module has zero dependencies on other project modules
- The `fs` module depends only on `model`
- The `cli` modules depend on both `fs` and `model`

This prevents circular dependencies and makes the codebase predictable.

### 3. Text-First Persistence
The knowledge base is stored as human-readable Markdown files with YAML front matter:
- Facilitates version control
- Enables manual editing
- Provides transparency and portability
- Uses standard formats (Markdown, YAML, ISO 8601 timestamps)

### 4. Convention Over Configuration
The tool follows sensible defaults:
- `.claude/` directory as the knowledge root
- Standardized subdirectory structure
- Automatic slug generation for file names
- Type-based file organization

## Design Patterns in Use

### Command Pattern
Each CLI subcommand is encapsulated as a separate module with a `run()` function:
```rust
// src/cli/mod.rs
pub enum Command {
    Init(InitArgs),
    New(NewArgs),
    Search(SearchArgs),
    // ...
}

pub fn execute(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Init(args) => init::run(args),
        Command::New(args) => new::run(args),
        // ...
    }
}
```

### Repository Pattern
`src/fs.rs` acts as a repository, abstracting filesystem operations:
```rust
pub fn walk_kb_documents(claude_root: &Path)
    -> impl Iterator<Item = Result<DocumentEntry>>
```
This provides a clean interface for document discovery without exposing implementation details.

### Data Transfer Object (DTO)
`DocumentFrontMatter` serves as a DTO for serialization:
- Uses `serde` for automatic YAML conversion
- Validates structure on deserialization
- Separates data representation from business logic

### Strategy Pattern (Lightweight)
The `init` command uses enum-based strategy switching:
```rust
enum ReportMode {
    DryRun,
    Execution,
}
```
This allows behavior variation without conditional complexity.

## Architectural Decisions

### Why Layered Architecture?
- **Simplicity**: Easy to understand and navigate
- **Testability**: Each layer can be tested independently
- **Maintainability**: Changes in one layer don't cascade to others
- **Reusability**: Core logic (`model`, `fs`) could be repurposed for other interfaces (GUI, web)

### Why Markdown + YAML?
- **Version control friendly**: Diffs are human-readable
- **Tool ecosystem**: Works with existing Markdown editors
- **Future-proof**: Standard formats with wide support
- **Low friction**: No database setup or migration required

### Why Integration-Only Testing?
The project uses end-to-end integration tests (`tests/`) rather than unit tests:
- **User perspective**: Tests validate actual CLI behavior
- **Refactoring resilience**: Internal changes don't break tests if behavior is preserved
- **Coverage**: Tests the entire stack from CLI to filesystem
- **Simplicity**: No need for complex mocking or test doubles

### Why anyhow for Errors?
- **Ergonomic**: No need to define custom error types
- **Context-rich**: `.with_context()` adds user-friendly messages
- **Flexible**: Any error type can be converted to `anyhow::Error`
- **Sufficient**: For a CLI, detailed error types aren't critical

## Module Independence

### Core Modules (Foundation)
- **model.rs**: Defines domain entities and rules
- **fs.rs**: Provides filesystem abstraction
- **cli/mod.rs**: Defines CLI structure and routing

### Peripheral Modules (Features)
- **cli/*.rs**: Individual command implementations
- **main.rs**: Minimal binary entry point
- **lib.rs**: Library module declarations

## External Dependencies

The project maintains minimal dependencies:

| Crate | Purpose | Layer |
|-------|---------|-------|
| clap | CLI parsing | Presentation |
| anyhow | Error handling | Cross-cutting |
| serde/serde_yaml | Serialization | Domain |
| chrono | Timestamps | Domain |
| uuid | Unique identifiers | Domain |
| walkdir | Filesystem traversal | Infrastructure |

See [Dependencies](./dependencies.md) for detailed analysis.

## Architectural Trade-offs

### Simplicity vs. Performance
The tool loads entire documents into memory during search:
- **Advantage**: Simple implementation, sufficient for small-to-medium knowledge bases
- **Trade-off**: Not optimized for very large document sets
- **Mitigation**: Could be optimized with indexing if needed

### Strictness vs. Flexibility
The validation enforces strict consistency:
- **Advantage**: Prevents data quality issues
- **Trade-off**: May feel restrictive initially
- **Mitigation**: `--strict` flag allows warnings to be non-fatal

## Future Architectural Considerations

### Potential Extensions
While the current architecture is stable, potential enhancements could include:
- **Plugin system**: Allow custom document types or validators
- **Indexing**: Add search acceleration for large knowledge bases
- **Remote sync**: Support for distributed knowledge bases
- **Web interface**: Reuse `model` and `fs` layers for a web UI

### Scalability Limits
Current architecture optimized for:
- Individual developer knowledge bases
- Small team collaboration
- Version-controlled workflows

For enterprise scale, consider:
- Database backend
- Concurrent access handling
- Advanced search capabilities
