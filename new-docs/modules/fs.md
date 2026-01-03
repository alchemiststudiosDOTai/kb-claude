---
title: Filesystem Module
path: src/fs.rs
type: file
depth: 1
description: File system operations for .claude directory management
exports: [ClaudePaths, find_existing_root, walk_kb_documents, CLAUDE_DIRECTORIES]
seams: [M]
---

## Where
`src/fs.rs`

## What
Provides filesystem utilities for managing the `.claude` knowledge base structure, including path resolution, directory creation, and document traversal.

## How

### Constants
- **`CLAUDE_ROOT`**: Root directory name (`.claude`)
- **`MANIFEST_FILE`**: Manifest filename (`manifest.md`)
- **`CLAUDE_DIRECTORIES`**: Array of subdirectory names (`metadata`, `debug_history`, `qa`, `code_index`, `patterns`, `plans`, `other`, `cheatsheets`, `memory_anchors`)
- **`MD_EXTENSION`**: File extension (`md`)

### `ClaudePaths` Struct
Encapsulates all path logic for a `.claude` instance:
- **`root: PathBuf`**: Absolute path to `.claude` directory
- Methods:
  - `new()`: Constructor from base path
  - `root()`: Accessor for root path
  - `manifest_path()`: Path to `manifest.md`
  - `type_directory()`: Path to specific subdirectory
  - `known_types()`: List supported types
  - `is_supported_type()`: Validate type
  - `ensure_layout()`: Create directory structure using `fs::create_dir_all`

### Path Resolution Functions
- **`claude_root_from()`**: Construct `.claude` path from base
- **`find_existing_root()`**: **Critical function**
  - Walks up directory tree from given path
  - Searches for parent containing `.claude` directory
  - Enables CLI execution from any subdirectory
- **`resolve_claude_root_from_cwd()`**: Combine `std::env::current_dir()` with search
- **`resolve_claude_root()`**: Default to current directory if not found
- **`display_relative()`**: Format paths for user-friendly output

### Filtering and Traversal
- **`is_ignored_path()`**: Determines if path should be skipped
  - Checks if within `.claude` but not in `CLAUDE_DIRECTORIES`
  - Ensures only recognized document types are processed

- **`walk_kb_documents()`**: Iterator over all valid KB documents
  - Uses `walkdir::WalkDir` for recursive traversal
  - Filters:
    - Files only (skip directories)
    - Skip `manifest.md`
    - Skip ignored paths
    - Only `.md` extension
  - Returns `DocumentEntry` structs containing:
    - `PathBuf`: File path
    - `Document`: Parsed content
  - Uses `anyhow::Context` for error propagation

## Why
**Structure Enforcement**: Explicitly defines expected directory layout, ensuring consistency across projects.

**Path Abstraction**: `ClaudePaths` centralizes path logic, reducing string manipulation errors and making refactoring easier.

**User Experience**: `find_existing_root` allows users to run commands from any subdirectory without specifying the KB location, improving workflow ergonomics.

**Robust Traversal**: `walk_kb_documents` provides a single, well-tested way to iterate over documents with proper filtering, eliminating duplicate traversal logic across commands.

**Error Context**: Extensive use of `anyhow::Context` provides clear error messages that help users understand what went wrong and where.

**Separation of Concerns**: Isolates filesystem operations from business logic, making the codebase easier to test and maintain.
