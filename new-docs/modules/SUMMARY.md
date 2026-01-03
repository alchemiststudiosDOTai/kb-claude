# KB-Claude Module Documentation

This directory contains comprehensive documentation for all modules in the `/Users/tuna/kb-claude/src` directory.

## Module Index

### Core Library (Depth 1)

| Module | File | Purpose |
|--------|------|---------|
| **Library Root** | [lib.md](lib.md) | Crate root exposing public API |
| **Model** | [model.md](model.md) | Data structures for KB entries |
| **Filesystem** | [fs.md](fs.md) | File system operations |
| **Binary Entry** | [main.md](main.md) | Application entry point |

### CLI Commands (Depth 2)

| Module | File | Purpose |
|--------|------|---------|
| **CLI Root** | [cli-mod.md](cli-mod.md) | Command definition and dispatch |
| **Init** | [cli-init.md](cli-init.md) | Initialize KB structure |
| **New** | [cli-new.md](cli-new.md) | Create new documents |
| **Search** | [cli-search.md](cli-search.md) | Search by terms and tags |
| **Link** | [cli-link.md](cli-link.md) | Link related documents |
| **Validate** | [cli-validate.md](cli-validate.md) | Validate KB integrity |
| **Manifest** | [cli-manifest.md](cli-manifest.md) | Generate document index |

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                        main.rs                              │
│                    (Binary Entry Point)                     │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                       lib.rs                                │
│              (Crate Root - Public API)                      │
└──┬──────────────┬──────────────┬─────────────────────────────┘
   │              │              │
   ▼              ▼              ▼
┌─────────┐  ┌─────────┐  ┌──────────────┐
│   cli   │  │   fs    │  │    model     │
└────┬────┘  └────┬────┘  └──────────────┘
     │           │
     │           └──────┐ Used by CLI
     │                  │
     ▼                  ▼
┌─────────────────────────────────────┐
│      Command Modules                │
│  ┌───┬───┬───┬───┬───┬───┐         │
│  │I│N│S│L│V│M│                   │
│  │n│e│e│i│a│a│                   │
│  │i│w│a│n│l│n│                   │
│  │t│ │r│k│i│i│                   │
│  │ │ │c│ │d│f│                   │
│  │ │ │h│ │a│e│                   │
│  │ │ │ │ │t│s│                   │
│  │ │ │ │ │e│t│                   │
│  └───┴───┴───┴───┴───┴───┘         │
└─────────────────────────────────────┘
```

## Module Relationships

### Data Flow
1. **main.rs** → Entry point, delegates to CLI
2. **cli/mod.rs** → Parses arguments, dispatches to commands
3. **cli/*.rs** → Command implementations use:
   - **model.rs** → Parse/create documents
   - **fs.rs** → Locate/read/write files

### Key Dependencies
- **CLI commands** depend on **model** and **fs**
- **fs** depends on **model** (for parsing documents during traversal)
- **model** has no dependencies (pure data structures)
- **main** depends only on **cli**

## Design Patterns

### Separation of Concerns
- **model**: What (data structures)
- **fs**: Where (persistence)
- **cli**: How (user interface)

### Modular Commands
Each CLI subcommand is an independent module with:
- Clear arguments via clap structs
- Isolated business logic
- Reusable functions

### Path Abstraction
`ClaudePaths` encapsulates all filesystem logic, preventing string manipulation scattered across the codebase.

## Documentation Format

Each module file contains:
- **Where**: File location
- **What**: Purpose and responsibility
- **How**: Implementation details
- **Why**: Design rationale

Frontmatter includes:
- `title`: Module name
- `path`: Source file path
- `type`: "file"
- `depth`: Hierarchy level (1=core, 2=command)
- `description`: One-line summary
- `exports`: Public API items
- `seams`: SEAMS classification (L=Library, M=Module)

## Quick Reference

### For Developers
- Start with **model.md** to understand data structures
- Read **fs.md** to learn filesystem operations
- Check **cli-mod.md** for command structure
- Refer to individual command docs for implementation details

### For Contributors
- New commands should follow the pattern in **cli-*.md** files
- Use **model** types for all document operations
- Use **fs** utilities for path resolution
- Keep **main.rs** minimal

### For Users
- See individual **cli-*.md** files for command usage
- Refer to **manifest.md** for understanding the generated index
- Check **validate.md** for maintaining KB health
