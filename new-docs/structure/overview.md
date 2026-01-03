---
title: Codebase Structure Overview
path: new-docs/structure/
type: overview
depth: 0
description: Comprehensive overview of kb-claude project organization and directory structure
seams: [S]
---

# Codebase Structure Overview

## Project Summary

**`kb-claude`** is a Rust CLI application for managing markdown-based knowledge bases with structured YAML front matter. The project demonstrates a "dogfooding" philosophy - it uses its own tooling to manage its development knowledge and institutional memory.

## Directory Tree

```
kb-claude/
├── src/                          # Rust source code
│   ├── main.rs                   # Binary entry point (thin wrapper)
│   ├── lib.rs                    # Library root exposing cli, fs, model
│   ├── model.rs                  # Domain models and front matter schema
│   ├── fs.rs                     # Filesystem utilities and path management
│   └── cli/                      # CLI subcommand implementations
│       ├── mod.rs                # Command parser and dispatcher
│       ├── init.rs               # Initialize knowledge base
│       ├── new.rs                # Create new entries
│       ├── link.rs               # Link related documents
│       ├── manifest.rs           # Generate manifest index
│       ├── validate.rs           # Validate schema integrity
│       └── search.rs             # Search knowledge base
│
├── tests/                        # Integration test suite
│   ├── smoke.rs                  # End-to-end workflow tests
│   └── command_matrix.rs         # Command interaction tests
│
├── .claude/                      # Knowledge base (managed by CLI)
│   ├── metadata/                 # Component summaries and releases
│   ├── debug_history/            # Debugging timelines
│   ├── qa/                       # Q&A and learning notes
│   ├── code_index/               # File and module references
│   ├── patterns/                 # Reusable solutions
│   ├── plans/                    # Project and release plans
│   ├── other/                    # Scratch notes
│   ├── cheatsheets/              # Quick references
│   ├── memory_anchors/           # Core concepts with UUIDs
│   └── manifest.md               # Auto-generated index
│
├── memory-bank/                  # Agent-managed workflow tracking
│   ├── research/                 # Investigation and analysis
│   ├── plan/                     # Implementation plans
│   └── execute/                  # Execution logs and audits
│
├── new-docs/                     # Placeholder for structured docs
│   ├── structure/                # Directory structure analysis
│   ├── architecture/             # (planned) Architecture docs
│   ├── entry/                    # (planned) Entry point docs
│   ├── modules/                  # (planned) Module documentation
│   └── state/                    # (planned) State management docs
│
├── Cargo.toml                    # Rust project manifest
├── Cargo.lock                    # Dependency lock file
├── README.md                     # Quick start and overview
├── CLAUDE.md                     # Project guidelines and workflow
├── AGENTS.md                     # Agent instructions
└── .gitignore                    # Version control exclusions
```

## Directory Purpose Matrix

| Directory | Purpose | Managed By | Content Type | Lifecycle |
|-----------|---------|------------|--------------|-----------|
| `src/` | Source code | Developers | Rust modules | Persistent |
| `tests/` | Integration tests | Developers | Test functions | Persistent |
| `.claude/` | Knowledge base | CLI tool | Markdown + YAML | Dynamic |
| `memory-bank/` | Workflow tracking | Agents | Workflow docs | Temporary |
| `new-docs/` | Structured docs | (in progress) | Markdown | Evolving |
| `target/` | Build output | Rust compiler | Binaries | Generated |

## Architectural Layers

### 1. Application Layer (`src/`)
- **Entry Point**: `main.rs` → `lib.rs` → `cli::run()`
- **CLI Interface**: `src/cli/` parses commands and dispatches
- **Business Logic**: `model.rs` defines domain and schema
- **Filesystem**: `fs.rs` handles all file operations

### 2. Test Layer (`tests/`)
- **Integration Tests**: Drive actual binary via `assert_cmd`
- **Filesystem Assertions**: Verify state via `assert_fs`
- **Smoke Tests**: Critical path validation
- **Regression Tests**: Bug fix verification

### 3. Knowledge Layer (`.claude/`)
- **Structured Knowledge**: Categorized by entry type
- **Cross-References**: UUID-based linking between documents
- **Validation**: Schema-enforced front matter
- **Searchability**: Manifest-generated index

### 4. Workflow Layer (`memory-bank/`)
- **Research**: Investigation and problem analysis
- **Planning**: Detailed implementation plans
- **Execution**: Step-by-step change logs
- **Audit Trail**: Complete workflow traceability

### 5. Documentation Layer (`new-docs/`, `README.md`, `CLAUDE.md`)
- **Quick Start**: `README.md` for getting started
- **Guidelines**: `CLAUDE.md` for development practices
- **Structured**: `new-docs/` for hierarchical reference

## Key Architectural Decisions

### 1. Thin Binary Pattern
`main.rs` is minimal (~3 lines), all logic in library:
- **Benefit**: Library fully testable and reusable
- **Pattern**: `main.rs` → `lib.rs::cli::run()`
- **File**: `src/main.rs`

### 2. One File Per Subcommand
Each CLI command gets its own module:
- **Benefit**: Clear organization, easy to extend
- **Pattern**: `src/cli/<command>.rs`
- **Files**: `init.rs`, `new.rs`, `link.rs`, etc.

### 3. Integration-First Testing
Tests drive the binary, not internal APIs:
- **Benefit**: Tests verify user-facing behavior
- **Tools**: `assert_cmd`, `assert_fs`
- **Location**: `tests/`

### 4. Type-Based Knowledge Organization
Knowledge base categorized by entry type:
- **Benefit**: Clear semantics, targeted search
- **Types**: metadata, debug_history, qa, code_index, patterns, plans, cheatsheet, memory_anchor
- **Location**: `.claude/<type>/`

### 5. Workflow Phase Tracking
Agent work tracked across phases:
- **Benefit**: Complete audit trail of development
- **Phases**: research → plan → execute
- **Location**: `memory-bank/<phase>/`

### 6. YAML Front Matter Schema
Structured metadata in every markdown file:
- **Benefit**: Machine-readable, human-editable
- **Fields**: title, type, uuid, created_at, ontological_relations, tags
- **Module**: `src/model.rs`

## Naming Conventions Summary

### Files and Directories
- **Rust source**: `snake_case.rs` (e.g., `model.rs`, `cli/mod.rs`)
- **Tests**: `snake_case.rs` (e.g., `smoke.rs`)
- **KB entries**: `kebab-case.md` (e.g., `cli-command-cheatsheet.md`)
- **Workflow docs**: `YYYY-MM-DD_HH-MM-SS_topic.md`
- **Directories**: `lowercase` or `snake_case`

### Code
- **Functions**: `snake_case` (e.g., `run`, `slugify`, `claude_root_from`)
- **Structs**: `PascalCase` (e.g., `Document`, `Cli`, `Command`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `CLAUDE_ROOT`, `MANIFEST_FILE`)
- **Enums**: `PascalCase` (e.g., `ReportMode`, `Severity`)

### Command-Line
- **Commands**: lowercase (e.g., `init`, `new`, `manifest`)
- **Long flags**: `kebab-case` (e.g., `--dry-run`, `--relates-to`)
- **Short flags**: Single character (e.g., `-t`, `-d`, `-v`)

## Relationship Map

### Code Relationships
```
main.rs
  └─> lib.rs
       ├─> cli/mod.rs
       │    ├─> init.rs
       │    ├─> new.rs
       │    ├─> link.rs
       │    ├─> manifest.rs
       │    ├─> validate.rs
       │    └─> search.rs
       ├─> fs.rs (path operations)
       └─> model.rs (schema definitions)
```

### Knowledge Flow
```
Developer/Agent
  ├─> CLI Commands (src/cli/)
  │    └─> .claude/ (create/modify entries)
  ├─> Memory Bank (workflow tracking)
  │    ├─> research/
  │    ├─> plan/
  │    └─> execute/
  └─> Documentation (CLAUDE.md, README.md)
```

### Data Flow
```
User Input (CLI)
  └─> cli::run()
       ├─> fs::resolve_claude_root()
       ├─> model::parse() (read front matter)
       ├─> business logic (specific command)
       └─> fs::write() (save changes)
            └─> .claude/<type>/entry.md
```

## Entry Points

### For Users
1. **`README.md`** - Quick start guide
2. **`kb-claude --help`** - CLI reference
3. **`.claude/cheatsheets/`** - Command quick references

### For Developers
1. **`CLAUDE.md`** - Development guidelines
2. **`src/lib.rs`** - Public API
3. **`src/cli/mod.rs`** - Command definitions
4. **`tests/`** - Usage examples via tests

### For Agents
1. **`AGENTS.md`** - Agent instructions
2. **`memory-bank/`** - Workflow tracking
3. **`.claude/`** - Knowledge base for context

## Build and Development Workflow

### Development Commands
```bash
cargo build              # Regular development build
cargo build --release    # Optimized release build
cargo run -- <subcommand> # Test CLI directly
cargo test               # Run integration tests
cargo fmt                # Format code
cargo clippy -- -D warnings  # Lint with strict mode
cargo doc --open         # Generate and view API docs
```

### Typical Workflow
1. **Define**: State problem, inputs, outputs, success criteria
2. **Test**: Outline test cases or integration scenarios
3. **Build**: Implement with Rust, following conventions
4. **Document**: Capture in `.claude/` knowledge base
5. **Review**: Run fmt, clippy, test; verify solution matches definition

## Extension Points

### Adding New CLI Commands
1. Create `src/cli/<command>.rs`
2. Add variant to `Command` enum in `mod.rs`
3. Implement handler function
4. Add match arm in `run()` dispatcher
5. Add integration test in `tests/`

### Adding New Knowledge Types
1. Add type to entry type enum in `model.rs`
2. Add directory to `CLAUDE_DIRECTORIES` in `fs.rs`
3. Update validation logic in `validate.rs`
4. Document in `CLAUDE.md`

### Adding New Documentation
1. Create file in appropriate `new-docs/` subdirectory
2. Add frontmatter with metadata
3. Write structured content
4. Cross-reference related documents

## Dependencies and Tools

### Rust Crates (from `Cargo.toml`)
- **`clap`**: CLI argument parsing
- **`serde`**: Serialization/deserialization
- **`chrono`**: Timestamp handling
- **`anyhow`**: Error handling
- **`uuid`**: Unique identifier generation

### Test Tools
- **`assert_cmd`**: Run and assert CLI commands
- **`assert_fs`**: Temporary filesystem fixtures

### Documentation Tools
- **`cargo doc`**: Rust doc generation
- **Markdown**: Knowledge base format
- **YAML**: Front matter metadata

## File Lifecycle

### Source Files (`src/`)
- **Created**: When implementing features
- **Modified**: During bug fixes and refactoring
- **Deleted**: When removing deprecated features
- **Tracked**: Git version control

### Knowledge Base Files (`.claude/`)
- **Created**: Via `kb-claude new` command
- **Modified**: Via manual editing or `kb-claude link`
- **Validated**: Via `kb-claude validate`
- **Indexed**: Via `kb-claude manifest`
- **Tracked**: Git (with `.gitignore` for personal notes)

### Workflow Files (`memory-bank/`)
- **Created**: By agents during development
- **Linked**: Across phases (research → plan → execute)
- **Archived**: After workflow completion
- **Tracked**: Git version control

## Summary

The `kb-claude` project demonstrates a sophisticated approach to knowledge management and software development:

- **Clean Architecture**: Separation of concerns between CLI, business logic, and filesystem
- **Self-Documenting**: Uses its own tooling to manage development knowledge
- **Test-Driven**: Integration tests verify actual binary behavior
- **Workflow-Aware**: Agent-managed tracking of research, planning, and execution
- **Developer-Friendly**: Clear conventions, comprehensive guidelines, and helpful tooling

The directory structure reflects these principles, with each directory serving a specific purpose in the overall system architecture.
