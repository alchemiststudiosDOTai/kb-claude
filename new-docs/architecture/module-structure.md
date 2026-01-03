---
title: Module Structure and Dependencies
path: /Users/tuna/kb-claude/new-docs/architecture/module-structure.md
type: metadata
depth: 1
description: Detailed breakdown of code organization, module responsibilities, and dependency graph
seams: ["architecture/overview", "architecture/data-flow"]
---

# Module Structure and Dependencies

## Dependency Graph

```
┌──────────────────┐
│    main.rs       │  Binary entry point
│  (2 lines)       │  Delegates to cli::run()
└────────┬─────────┘
         │
         ↓
┌──────────────────┐
│    lib.rs        │  Library root
│  (4 lines)       │  Declares public modules
└────────┬─────────┘
         │
         ↓
┌─────────────────────────────────────────────────────────────┐
│                    cli/mod.rs                               │
│  (CLI router, argument parsing, command dispatch)          │
└─┬───────────┬───────────┬───────────┬───────────┬─────────┘
  │           │           │           │           │
  ↓           ↓           ↓           ↓           ↓
┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐
│ init.rs│ │ new.rs │ │search.rs│ │validate│ │manifest│
│        │ │        │ │        │ │.rs    │ │.rs    │
└────────┘ └────────┘ └────────┘ └────────┘ └────────┘
  │           │           │           │           │
  └───────────┴───────────┴───────────┴───────────┘
                              │
                              ↓
              ┌───────────────────────────────┐
              │          fs.rs                │
              │  (Filesystem abstraction)     │
              └───────────────┬───────────────┘
                              │
                              ↓
              ┌───────────────────────────────┐
              │         model.rs              │
              │  (Domain entities, rules)     │
              └───────────────────────────────┘
```

## Module Details

### main.rs (Binary Entry Point)
**Location**: `/Users/tuna/kb-claude/src/main.rs`
**Lines of Code**: 2
**Responsibility**: Minimal executable bootstrap

```rust
fn main() -> Result<()> {
    claude_kb_cli::cli::run()
}
```

**Dependencies**:
- `claude_kb_cli::cli`

**Design Rationale**:
- Keeps binary entry point minimal
- All logic in library crate
- Allows library use by other tools

---

### lib.rs (Library Root)
**Location**: `/Users/tuna/kb-claude/src/lib.rs`
**Lines of Code**: 4
**Responsibility**: Declare public modules

```rust
pub mod cli;
pub mod fs;
pub mod model;
```

**Dependencies**:
- None (organizational only)

**Design Rationale**:
- Simple module declarations
- No business logic
- Clear public API boundary

---

### cli/mod.rs (CLI Router)
**Location**: `/Users/tuna/kb-claude/src/cli/mod.rs`
**Lines of Code**: 171
**Responsibility**: CLI structure, parsing, and dispatch

**Key Types**:
- `Cli`: Top-level CLI structure
- `Command`: Enum of all subcommands
- `InitArgs`, `NewArgs`, etc.: Argument structs

**Key Functions**:
- `run()`: Entry point, parses CLI
- `execute()`: Dispatches to subcommand

**Dependencies**:
- `clap` (external): CLI parsing
- Subcommand modules: `init`, `new`, `search`, `link`, `validate`, `manifest`

**Design Rationale**:
- Declarative CLI definition via derive macros
- Central routing logic
- Each subcommand isolated in separate module

---

### cli/init.rs (Initialize Knowledge Base)
**Location**: `/Users/tuna/kb-claude/src/cli/init.rs`
**Lines of Code**: 91
**Responsibility**: Create `.claude/` directory structure

**Key Functions**:
- `run()`: Execute init command
- `plan_layout()`: Determine what needs to be created
- `report_changes()`: Display what was done

**Dependencies**:
- `crate::fs`: Path utilities, ClaudePaths
- `std::fs`: Directory creation

**Workflow**:
1. Normalize workspace path
2. Plan required directories
3. If dry-run, report and exit
4. Create directories via `ClaudePaths::ensure_layout()`
5. Report changes

**Design Rationale**:
- Idempotent: Safe to run multiple times
- Dry-run mode: Preview before execution
- Clear feedback: Show what was created

---

### cli/new.rs (Create Document)
**Location**: `/Users/tuna/kb-claude/src/cli/new.rs`
**Lines of Code**: 211
**Responsibility**: Create new knowledge base entries

**Key Functions**:
- `run()`: Execute new command
- `determine_type()`: Get document type (prompt or flag)
- `collect_tags()`: Get tags (prompt or flag)
- `collect_body()`: Interactive body input
- `compute_output_path()`: Determine file location

**Dependencies**:
- `crate::fs`: Path resolution, ClaudePaths
- `crate::model`: Document, DocumentFrontMatter, OntologicalRelation
- `std::fs`: File writing

**Workflow**:
1. Ensure `.claude/` exists (auto-init)
2. Determine document type (interactive or flag)
3. Collect tags, relations, body (interactive or flags)
4. Create DocumentFrontMatter with slugified link
5. Compute output path (or use override)
6. Write file with front matter + body

**Design Rationale**:
- Interactive by default, flags for automation
- Auto-init: Reduces friction
- Validation: Ensures type is supported
- Flexible: Supports both interactive and scripted use

---

### cli/search.rs (Search Documents)
**Location**: `/Users/tuna/kb-claude/src/cli/search.rs`
**Lines of Code**: 125
**Responsibility**: Search knowledge base content

**Key Functions**:
- `run()`: Execute search command
- `collect_documents()`: Load all documents
- `filter_match()`: Score and filter documents
- `build_search_blob()`: Create searchable text

**Dependencies**:
- `crate::fs`: Document walking
- `crate::model`: Document access

**Workflow**:
1. Load all documents via `walk_kb_documents()`
2. For each document:
   - Build search blob (title, link, type, tags, body, relations)
   - Filter by tags if specified
   - Check all search terms present
   - Score by term frequency
3. Sort by score (desc) then title (asc)
4. Display results

**Design Rationale**:
- Simple scoring: Term frequency
- Tag filtering: Pre-filter for efficiency
- Case-insensitive: User-friendly
- All fields searchable: Comprehensive

---

### cli/validate.rs (Validate Documents)
**Location**: `/Users/tuna/kb-claude/src/cli/validate.rs`
**Lines of Code**: 232
**Responsibility**: Check document integrity and consistency

**Key Functions**:
- `run()`: Execute validate command
- `collect_findings()`: Gather all issues
- `validate_document()`: Check single document

**Dependencies**:
- `crate::fs`: Document walking, ClaudePaths
- `crate::model`: Document access

**Workflow**:
1. Load all documents via `walk_kb_documents()`
2. For each document:
   - Check required fields present
   - Validate UUID not nil
   - Check type is supported
   - Verify link matches filename
   - Verify link matches slugified title
   - Verify directory matches type
3. Collect findings (errors and warnings)
4. Print findings
5. Exit with error if errors or strict mode with warnings

**Design Rationale**:
- Error vs Warning: Distinguish severity
- Strict mode: Treat warnings as errors for CI
- Comprehensive: Checks all consistency rules
- Clear messages: Path + problem description

---

### cli/manifest.rs (Generate Manifest)
**Location**: `/Users/tuna/kb-claude/src/cli/manifest.rs`
**Lines of Code**: 140
**Responsibility**: Create summary table of all documents

**Key Functions**:
- `run()`: Execute manifest command
- `collect_entries()`: Gather document metadata
- `render_manifest()`: Generate markdown table

**Dependencies**:
- `crate::fs`: Document walking, ClaudePaths
- `std::fs`: File writing

**Workflow**:
1. Load all documents via `walk_kb_documents()`
2. Extract metadata (title, type, path, tags, relations, updated_at)
3. Sort by title (case-insensitive)
4. Render as markdown table
5. Write to manifest.md

**Design Rationale**:
- Summary view: Quick overview of entire KB
- Markdown table: Easy to read, version control
- Sorted: Predictable order
- Overridable: Custom output path

---

### cli/link.rs (Link Documents)
**Location**: `/Users/tuna/kb-claude/src/cli/link.rs`
**Lines of Code**: 115
**Responsibility**: Create bidirectional links between documents

**Key Functions**:
- `run()`: Execute link command
- `load_document()`: Find document by link
- `insert_relation()`: Add relation (with deduplication)

**Dependencies**:
- `crate::fs`: Document walking
- `crate::model`: Document, OntologicalRelation
- `std::fs`: File writing

**Workflow**:
1. Validate source != target
2. Load source and target documents by link
3. Insert bidirectional relations (with force flag handling)
4. Touch updated_at timestamps
5. Write both documents

**Design Rationale**:
- Bidirectional: Both documents get links
- Deduplication: Prevent duplicate relations
- Force flag: Override deduplication if needed
- Timestamp update: Track modification

---

### fs.rs (Filesystem Abstraction)
**Location**: `/Users/tuna/kb-claude/src/fs.rs`
**Lines of Code**: 183
**Responsibility**: Filesystem operations and path management

**Key Types**:
- `ClaudePaths`: Type-safe path wrapper
- `DocumentEntry`: Path + parsed document

**Key Constants**:
- `CLAUDE_ROOT`: ".claude"
- `CLAUDE_DIRECTORIES`: Known type directories
- `MANIFEST_FILE`: "manifest.md"

**Key Functions**:
- `find_existing_root()`: Search up directory tree for .claude
- `resolve_claude_root_from_cwd()`: Get .claude from current directory
- `walk_kb_documents()`: Iterator over all documents
- `is_ignored_path()`: Check if path should be excluded

**Dependencies**:
- `crate::model`: Document parsing
- `walkdir` (external): Directory traversal
- `std::fs`: File I/O

**Design Rationale**:
- Abstraction: Hide filesystem complexity
- Centralized: All path logic in one place
- Type safety: ClaudePaths prevents string manipulation errors
- Validation: Ensure paths are within .claude

---

### model.rs (Domain Layer)
**Location**: `/Users/tuna/kb-claude/src/model.rs`
**Lines of Code**: 163
**Responsibility**: Core data structures and business rules

**Key Types**:
- `Document`: Front matter + body
- `DocumentFrontMatter`: YAML metadata
- `OntologicalRelation`: Link to other document

**Key Functions**:
- `Document::parse()`: Parse markdown with YAML front matter
- `Document::to_markdown()`: Serialize to markdown
- `slugify()`: Convert title to URL-safe slug
- `DocumentFrontMatter::new()`: Create with defaults
- `DocumentFrontMatter::is_link_consistent()`: Validate link

**Dependencies**:
- None (domain layer is independent)

**Design Rationale**:
- No dependencies: Pure domain logic
- Validation: Business rules in model layer
- Serialization: Serde for automatic YAML conversion
- ISO 8601: Standard timestamp format

---

## Dependency Characteristics

### Coupling Analysis

**Low Coupling** (Good):
- `model.rs` has zero dependencies on other project modules
- Each `cli/*.rs` file is independent of others
- `fs.rs` depends only on `model`

**Appropriate Coupling**:
- `cli` modules depend on `fs` and `model` (their purpose)
- `fs` depends on `model` for domain types

**No Circular Dependencies**:
- Dependency graph is a DAG (directed acyclic graph)
- Unidirectional flow: cli → fs → model

### Cohesion Analysis

**High Cohesion** (Good):
- Each module has single, clear purpose
- Related functions grouped together
- Minimal cross-cutting concerns

**Module Cohesion Scores**:
- `model.rs`: 10/10 (pure domain logic)
- `fs.rs`: 9/10 (all filesystem operations)
- `cli/init.rs`: 10/10 (single command)
- `cli/new.rs`: 9/10 (mostly document creation)
- `cli/search.rs`: 10/10 (single command)
- `cli/validate.rs`: 10/10 (single command)

## Testing Structure

### Integration Tests
**Location**: `/Users/tuna/kb-claude/tests/`

**smoke.rs**: Basic functionality tests
- Test each command executes
- Verify files created/modified
- Check error conditions

**command_matrix.rs**: Comprehensive command tests
- Test all commands with various flags
- Verify output format
- Check edge cases

**Testing Strategy**:
- Black-box testing (treat binary as opaque)
- Temporary directories for isolation
- Real filesystem (no mocking)
- Assert on exit code, stdout, stderr, and file state

**Why No Unit Tests?**:
- Integration tests provide better coverage
- Less brittle during refactoring
- Test actual user-facing behavior
- Sufficient for current scale

## Module Size Metrics

| Module | LOC | Purpose |
|--------|-----|---------|
| main.rs | 2 | Entry point |
| lib.rs | 4 | Module declarations |
| cli/mod.rs | 171 | CLI definition and routing |
| cli/init.rs | 91 | Initialize command |
| cli/new.rs | 211 | Create document command |
| cli/search.rs | 125 | Search command |
| cli/validate.rs | 232 | Validate command |
| cli/manifest.rs | 140 | Manifest generation |
| cli/link.rs | 115 | Link documents command |
| fs.rs | 183 | Filesystem operations |
| model.rs | 163 | Domain models |
| **Total** | **1,437** | |

## Future Module Considerations

### Potential New Modules
If the project grows, consider extracting:

1. **src/search.rs**: Dedicated search logic
   - Currently inlined in `cli/search.rs`
   - Would allow reuse by other commands

2. **src/validation.rs**: Validation rules
   - Currently inlined in `cli/validate.rs`
   - Would allow programmatic validation

3. **src/ontology.rs**: Knowledge graph logic
   - Currently implicit in link structure
   - Would enable graph traversal, relationship queries

4. **src/config.rs**: Configuration management
   - Currently hardcoded constants
   - Would allow customization

### Module Boundaries to Maintain
- **model.rs** must remain dependency-free
- **fs.rs** should never depend on **cli**
- **cli/** modules should not depend on each other
- Keep `walk_kb_documents()` as single data access point

## Dependency Management

### Adding New Dependencies
Before adding external crates, consider:
1. Is it necessary for core functionality?
2. Does it increase binary size significantly?
3. Is it actively maintained?
4. Does it have a compatible license?
5. Can we implement it ourselves simply?

### Removing Unused Dependencies
Periodically audit:
- Check Cargo.toml for unused dependencies
- Use `cargo-udeps` to detect unused dependencies
- **Note**: `glob` dependency appears unused in current codebase

### Version Strategy
- Pin major versions for production
- Allow minor/patch updates for bug fixes
- Test dependency updates thoroughly
- Monitor security advisories
