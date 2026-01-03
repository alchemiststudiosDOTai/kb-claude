---
title: Struct-based State Containers
path: /Users/tuna/kb-claude/new-docs/state/struct-based-state-containers.md
type: state-analysis
depth: 1
description: Analysis of state management through Rust structs in kb-claude
seams: []
---

# Struct-based State Containers

## Overview

kb-claude manages state through explicit Rust structs rather than global variables. This promotes clear ownership, predictable lifetimes, and thread-safe operations.

## Core Domain State Containers

### `Document` - Primary Knowledge Unit

**Location**: `src/model.rs`

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    pub front_matter: DocumentFrontMatter,
    pub body: String,
}
```

**Responsibilities**:
- Encapsulates complete KB entry (metadata + content)
- Serialization to Markdown format
- Deserialization from Markdown with YAML front matter
- Validation of document structure

**Lifecycle**:
1. Created via `Document::new()` or `Document::parse()`
2. Passed through functions for validation/processing
3. Serialized via `to_markdown()` for persistence
4. Written to filesystem via `fs::write()`

**Key Methods**:
- `new()`: Constructor from front matter + body
- `parse()`: Deserialize from Markdown string
- `to_markdown()`: Serialize to Markdown string

### `DocumentFrontMatter` - Metadata State

**Location**: `src/model.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DocumentFrontMatter {
    pub title: String,
    pub link: String,
    pub doc_type: String,
    pub ontological_relations: Vec<OntologicalRelation>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub uuid: Uuid,
}
```

**Responsibilities**:
- Stores all document metadata
- Manages timestamps for tracking changes
- Provides UUID-based unique identification
- Maintains cross-references to other documents

**State Mutations**:
- `touch_updated()`: Updates `updated_at` timestamp
- `ensure_link_matches_slug()`: Synchronizes link with title
- `slug_from_title()`: Computes URL-friendly slug
- `is_link_consistent()`: Validates link/title relationship

**Design Pattern**: This struct implements the **State Object pattern** - a mutable container of related state that can be passed by reference for updates.

### `OntologicalRelation` - Cross-Reference State

**Location**: `src/model.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OntologicalRelation {
    pub relates_to: String,
}
```

**Purpose**: Lightweight value type representing links between documents.

**Storage**: Aggregated within `DocumentFrontMatter` as a `Vec`.

## Filesystem Context State

### `ClaudePaths` - Path Resolution Context

**Location**: `src/fs.rs`

```rust
#[derive(Debug, Clone)]
pub struct ClaudePaths {
    root: PathBuf,
}
```

**Responsibilities**:
- Encapsulates resolved `.claude` root path
- Provides methods for computing derived paths
- Ensures consistent path usage across operations

**Key Methods**:
- `new()`: Initialize with root path
- `root()`: Access root path
- `manifest_path()`: Compute manifest file location
- `type_directory()`: Compute document type subdirectory path
- `ensure_layout()`: Create directory structure if missing

**Design Pattern**: **Context Object pattern** - bundles related configuration/state for passing through function calls.

### `DocumentEntry` - Traversal State

**Location**: `src/fs.rs`

```rust
#[derive(Debug)]
pub struct DocumentEntry {
    pub path: PathBuf,
    pub document: crate::model::Document,
}
```

**Purpose**: Temporary container combining file path with parsed document.

**Lifecycle**:
1. Created by `walk_kb_documents()` iterator
2. Consumed by command-specific processing functions
3. Not persisted - exists only during traversal

## CLI Argument State Containers

### Argument Structs - Runtime Configuration

**Location**: `src/cli/mod.rs`

Each CLI subcommand has a dedicated struct capturing user input:

```rust
#[derive(Args, Debug, Clone)]
pub struct InitArgs {
    pub directory: PathBuf,
    pub dry_run: bool,
}

#[derive(Args, Debug, Clone)]
pub struct NewArgs {
    pub title: String,
    pub doc_type: Option<String>,
    pub tags: Vec<String>,
    pub relates_to: Vec<String>,
    pub file_override: Option<PathBuf>,
}

// ... SearchArgs, LinkArgs, ValidateArgs, ManifestArgs
```

**Responsibilities**:
- Capture command-line configuration
- Validate user input at parse time
- Provide type-safe access to CLI arguments
- Document command interface via struct definitions

**Lifecycle**:
1. Parsed by `clap` at program start
2. Passed to subcommand `run()` functions
3. Consumed for operation execution
4. Dropped when command completes

## Transient Processing State

### `SearchMatch` - Search Results

**Location**: `src/cli/search.rs`

```rust
#[derive(Debug)]
struct SearchMatch {
    title: String,
    doc_type: String,
    tags: Vec<String>,
    path: PathBuf,
    score: usize,
}
```

**Purpose**: Aggregates search results for sorting and display.

**Lifecycle**:
1. Created by `filter_match()` during search
2. Collected into `Vec<SearchMatch>`
3. Sorted by score and title
4. Consumed for output formatting

### `Finding` - Validation Results

**Location**: `src/cli/validate.rs`

```rust
struct Finding {
    path: PathBuf,
    message: String,
    severity: Severity,
}
```

**Purpose**: Represents validation errors/warnings.

**Lifecycle**:
1. Created during validation checks
2. Collected into `Vec<Finding>`
3. Printed to stdout/stderr
4. Not persisted

### `ManifestEntry` - Manifest Generation

**Location**: `src/cli/manifest.rs`

```rust
#[derive(Debug)]
struct ManifestEntry {
    title: String,
    doc_type: String,
    relative_path: PathBuf,
    tags: Vec<String>,
    relations: Vec<String>,
    updated_at: chrono::NaiveDate,
}
```

**Purpose**: Simplified document representation for manifest table.

**Lifecycle**:
1. Extracted from `DocumentEntry` during collection
2. Sorted alphabetically by title
3. Rendered to Markdown table format
4. Written to `manifest.md`

## State Ownership Patterns

### Stack-Allocated Short-Lived State

Most state is stack-allocated and short-lived:
- CLI argument structs
- Transient processing containers
- Function-local variables

**Advantages**:
- Automatic memory management
- No heap allocation overhead
- Clear ownership semantics
- Thread-safe by default

### Clone-Based State Sharing

Many structs derive `Clone` for explicit state copying:

```rust
#[derive(Debug, Clone)]
pub struct ClaudePaths { ... }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DocumentFrontMatter { ... }
```

**Usage Pattern**:
- Original owned by one context
- Clones passed to functions that need ownership
- Expensive for large structs, but acceptable for CLI

### Borrow-Based State Access

State is frequently passed by mutable reference for updates:

```rust
fn touch_updated(&mut self) {
    self.updated_at = Utc::now();
}
```

**Advantages**:
- Zero-copy state sharing
- Explicit mutation sites
- Compiler-enforced aliasing rules
- Prevents simultaneous mutable access

## Data Flow Examples

### Document Creation Flow

```rust
// 1. Create front matter state
let mut front_matter = DocumentFrontMatter::new(&args.title, doc_type);

// 2. Mutate state
front_matter.tags = tags;
front_matter.ontological_relations = relations;
front_matter.ensure_link_matches_slug();

// 3. Compose document state
let document = Document::new(front_matter, body);

// 4. Serialize state
let content = document.to_markdown()?;

// 5. Persist state
fs::write(&output_path, content)?;
```

### Document Traversal Flow

```rust
// 1. Create iterator
for entry in walk_kb_documents(&claude_root) {
    // 2. Access temporary state
    let entry = entry?;
    let front = &entry.document.front_matter;

    // 3. Process state
    println!("{}: {}", front.title, front.doc_type);
}
```

## State Mutation Patterns

### Builder Pattern (Implicit)

`DocumentFrontMatter::new()` acts as a builder:

```rust
let mut fm = DocumentFrontMatter::new("Title", "type");
fm.tags = vec!["tag1".into()];
fm.touch_updated();
```

### Read-Modify-Write Pattern

Used in `link` command:

```rust
// 1. Read state
let mut doc = load_document(&source_slug)?;

// 2. Modify state
doc.front_matter.ontological_relations.push(relation);
doc.front_matter.touch_updated();

// 3. Write state
let content = doc.to_markdown()?;
fs::write(&path, content)?;
```

## Comparison with Global State Approaches

### What's Gained

1. **Explicit Dependencies**: Functions declare required state via parameters
2. **Testability**: State can be easily constructed and passed to functions
3. **Thread Safety**: No shared mutable state requiring synchronization
4. **Predictability**: State mutations are visible in function signatures

### What's Sacrificed

1. **Convenience**: Must pass state through call chains explicitly
2. **Performance**: Cloning state can be expensive (though acceptable for CLI)
3. **Caching**: No global cache to avoid recomputation

## Design Philosophy

The struct-based approach embodies Rust's ownership philosophy:
- **Explicit state ownership**: Clear who owns what data
- **Explicit mutability**: `mut` keyword marks mutation sites
- **Explicit lifetimes**: Borrow checker ensures validity
- **Zero-cost abstractions**: No runtime overhead for safety

This design prioritizes correctness, clarity, and maintainability over convenience optimizations.
