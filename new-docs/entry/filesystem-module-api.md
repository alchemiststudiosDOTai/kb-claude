---
title: Filesystem Module API
path: src/fs.rs
type: file
depth: 1
description: Filesystem utilities for .claude directory management
exports: [CLAUDE_ROOT, MANIFEST_FILE, CLAUDE_DIRECTORIES, ClaudePaths, DocumentEntry, is_ignored_path, claude_root_from, find_existing_root, resolve_claude_root_from_cwd, resolve_claude_root, display_relative, walk_kb_documents]
seams: [D]
---

# Filesystem Module API

## File: `src/fs.rs`

### Purpose
Provides filesystem utilities for managing the `.claude` directory structure, path resolution, and document traversal.

## Public Constants

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
pub const CURRENT_DIR_ERROR: &str = "...";
pub const NO_CLAUDE_DIR_ERROR: &str = "...";
pub const MD_EXTENSION: &str = "md";
```

## Public Structs

### `ClaudePaths`
Manages paths within the `.claude` hierarchy relative to a base directory.

**Constructor**:
- `pub fn new(base: impl AsRef<Path>) -> Self`

**Methods**:
- `pub fn root(&self) -> &Path` - Get `.claude` root path
- `pub fn manifest_path(&self) -> PathBuf` - Get manifest file path
- `pub fn type_directory(&self, doc_type: &str) -> PathBuf` - Get path for document type
- `pub fn known_types(&self) -> &'static [&'static str]` - List valid types
- `pub fn is_supported_type(&self, doc_type: &str) -> bool` - Validate type
- `pub fn ensure_layout(&self) -> Result<()>` - Create directory structure

---

### `DocumentEntry`
Represents a discovered knowledge base document during filesystem traversal.

**Fields**:
- `path: PathBuf` - Full path to document
- `content: String` - File contents
- `doc_type: String` - Parsed document type
- `slug: String` - Document slug from filename

## Public Functions

### Path Utilities

#### `is_ignored_path`
```rust
pub fn is_ignored_path(path: &Path, claude_root: &Path) -> bool
```
Determines if a path should be ignored (e.g., hidden files, non-markdown).

**Ignores**:
- Hidden files (starting with `.`)
- Non-`.md` files
- Special cases (manifest.md, etc.)

---

#### `claude_root_from`
```rust
pub fn claude_root_from(base: impl AsRef<Path>) -> PathBuf
```
Constructs the full path to the `.claude` directory from a base path.

---

#### `find_existing_root`
```rust
pub fn find_existing_root(start: impl AsRef<Path>) -> Option<PathBuf>
```
Traverses up the directory tree to find an existing `.claude` directory.

**Returns**: `None` if not found, `Some(PathBuf)` if found

---

#### `resolve_claude_root_from_cwd`
```rust
pub fn resolve_claude_root_from_cwd() -> Result<(PathBuf, PathBuf)>
```
Resolves the current working directory and finds the `.claude` root.

**Returns**: Tuple of `(working_dir, claude_root)`

**Error**: Fails if no `.claude` directory found

---

#### `resolve_claude_root`
```rust
pub fn resolve_claude_root(base_dir: Option<&Path>) -> Result<(PathBuf, PathBuf)>
```
Resolves target directory and `.claude` root from optional base directory.

**Behavior**:
- If `base_dir` provided: use it
- If `None`: use current directory
- Searches up tree for `.claude` root

---

#### `display_relative`
```rust
pub fn display_relative(workspace: &Path, path: &Path) -> String
```
Formats a path relative to a given workspace for display purposes.

---

### Document Traversal

#### `walk_kb_documents`
```rust
pub fn walk_kb_documents(claude_root: &Path) -> impl Iterator<Item = Result<DocumentEntry>> + '_
```
Iterates over all valid knowledge base documents in the `.claude` hierarchy.

**Returns**: Iterator of `Result<DocumentEntry>`

**Filters**:
- Only markdown files (`.md`)
- Ignores hidden files
- Skips ignored paths
- Includes all subdirectories

**Usage Example**:
```rust
for entry in walk_kb_documents(&claude_root)? {
    let doc = entry?;
    println!("Found: {}", doc.slug);
}
```

## Design Patterns

1. **Builder Pattern**: `ClaudePaths` provides fluent path construction
2. **Iterator Pattern**: `walk_kb_documents` returns lazy iterator
3. **Validation Layer**: All path operations include validation
4. **Error Handling**: Consistent `anyhow::Result` usage

## Use Cases

- **Initialization**: Creating `.claude` directory structure
- **Discovery**: Finding existing `.claude` roots
- **Traversal**: Walking all knowledge documents
- **Validation**: Checking file and directory existence
- **Path Management**: Constructing type-specific paths
