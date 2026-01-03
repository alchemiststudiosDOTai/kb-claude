---
title: Global State Stores and Constants
path: /Users/tuna/kb-claude/new-docs/state/global-state-stores.md
type: state-analysis
depth: 1
description: Analysis of global constants and immutable configuration in kb-claude
seams: []
---

# Global State Stores and Constants

## Overview

The kb-claude application uses **exclusively immutable global constants** for configuration. No mutable global state (`static mut`, `lazy_static!`, `once_cell`) is present in the codebase.

## Global Constants by Module

### `src/fs.rs` - Filesystem Configuration

```rust
pub const CLAUDE_ROOT: &str = ".claude";
pub const MANIFEST_FILE: &str = "manifest.md";
pub const CLAUDE_DIRECTORIES: &[&str] = &[
    "metadata",
    "debug_history",
    "qa",
    "code_index",
    "patterns",
    "plans",
    "cheatsheets",
    "memory_anchors",
];
pub const CURRENT_DIR_ERROR: &str = "Unable to determine current directory";
pub const NO_CLAUDE_DIR_ERROR: &str =
    "No .claude directory found under {}. Run `kb-claude init` first.";
pub const MD_EXTENSION: &str = "md";
```

**Purpose**: These constants define:
- The knowledge base root directory name
- The manifest filename
- Valid document type categories (whitelist approach)
- Reusable error messages
- File extension filter

**Usage**: Referenced throughout the codebase for filesystem operations, validation, and error handling.

### `src/model.rs` - Document Format Constants

```rust
pub const FRONT_MATTER_DELIMITER: &str = "---";
const FRONT_MATTER_BLOCK_BREAK: &str = "\n---\n";
```

**Purpose**: Define YAML front matter delimiters for Markdown documents.

**Usage**: Used in `Document::parse()` and `Document::to_markdown()` for serialization/deserialization.

## Design Implications

### Benefits

1. **Thread Safety**: Immutable constants require no synchronization
2. **Predictability**: Values never change during execution
3. **Simple Dependency**: No complex initialization order issues
4. **Zero Runtime Cost**: Constants are inlined during compilation

### Limitations

1. **No Runtime Configuration**: All structure is hardcoded at compile time
2. **No Extensibility**: Adding new document types requires code changes
3. **No User Customization**: Users cannot define custom directory structures without modifying source

## State Flow Pattern

Instead of global mutable state, the application uses:

1. **Function Arguments**: Explicit data passing through call chains
2. **Struct Instances**: Context objects like `ClaudePaths` that encapsulate resolved state
3. **CLI Argument Structs**: Runtime configuration from user input captured in `Args` structs
4. **Filesystem as Database**: Persistent state stored in `.md` files, not in memory

## Example: Path Resolution Flow

```rust
// No global state - resolution happens per-command
pub fn resolve_claude_root_from_cwd() -> Result<(PathBuf, PathBuf)> {
    let cwd = std::env::current_dir().context(CURRENT_DIR_ERROR)?;
    let claude_root = find_existing_root(&cwd)
        .unwrap_or_else(|| claude_root_from(&cwd));
    Ok((cwd, claude_root))
}
```

Each command explicitly resolves paths rather than relying on a global cache.

## Comparison with Alternative Approaches

### What's NOT Present

- **`lazy_static!`**: No lazy-initialized global state
- **`once_cell::sync::Lazy`**: No one-time synchronization
- **`std::sync::Mutex<T>`**: No global mutable state guarded by locks
- **`std::sync::RwLock<T>`**: No global read-write locks
- **`static mut`**: No unsafe mutable globals

### Rationale

The CLI's short-lived nature (one command per process) makes global state unnecessary. Each invocation:
1. Parses arguments
2. Resolves context
3. Executes operation
4. Exits

This design prioritizes simplicity and explicit data flow over performance optimizations that caching would provide.
