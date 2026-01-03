---
title: CLI Subcommands
path: src/cli/
type: directory
depth: 2
description: Individual CLI subcommand implementations
exports: []
seams: [D]
---

# CLI Subcommands

## Structure
Each subcommand is implemented in its own module under `src/cli/`, following the pattern of one file per command.

## Available Subcommands

### 1. `init` (`src/cli/init.rs`)
**Purpose**: Initializes a `.claude/` directory structure

**Arguments**:
- `directory: PathBuf` - Target directory to initialize
- `dry_run: bool` - Preview without making changes

**Entry Point**: `pub fn run(args: InitArgs) -> Result<()>`

**Creates**:
- `.claude/metadata/`
- `.claude/debug_history/`
- `.claude/qa/`
- `.claude/code_index/`
- `.claude/patterns/`
- `.claude/plans/`
- `.claude/other/`
- `.claude/cheatsheets/`
- `.claude/memory_anchors/`
- `.claude/manifest.md`

---

### 2. `new` (`src/cli/new.rs`)
**Purpose**: Creates a new knowledge entry with YAML front matter

**Arguments**:
- `title: String` - Document title
- `doc_type: Option<String>` - Entry type (metadata, debug_history, qa, etc.)
- `tags: Vec<String>` - Category tags
- `relates_to: Vec<String>` - Related document UUIDs
- `file_override: Option<PathBuf>` - Custom file path

**Entry Point**: `pub fn run(args: NewArgs) -> Result<()>`

**Outputs**: Markdown file with front matter in appropriate `.claude/` subdirectory

---

### 3. `search` (`src/cli/search.rs`)
**Purpose**: Searches across knowledge base entries

**Arguments**:
- `terms: Vec<String>` - Search terms
- `tags: Vec<String>` - Tag filters

**Entry Point**: `pub fn run(args: SearchArgs) -> Result<()>`

**Searches**: Content and metadata across all `.claude/` documents

---

### 4. `link` (`src/cli/link.rs`)
**Purpose**: Creates cross-references (ontological relations) between documents

**Arguments**:
- `source: String` - Source document UUID
- `target: String` - Target document UUID
- `force: bool` - Skip validation checks

**Entry Point**: `pub fn run(args: LinkArgs) -> Result<()>`

**Updates**: Adds relationship to front matter of both documents

---

### 5. `validate` (`src/cli/validate.rs`)
**Purpose**: Checks metadata consistency and validates knowledge entries

**Arguments**:
- `directory: Option<PathBuf>` - Directory to validate (default: current)
- `strict: bool` - Enable strict validation mode

**Entry Point**: `pub fn run(args: ValidateArgs) -> Result<()>`

**Validates**:
- YAML front matter syntax
- Required fields presence
- UUID consistency
- Link integrity
- Type-specific constraints

---

### 6. `manifest` (`src/cli/manifest.rs`)
**Purpose**: Rebuilds the `manifest.md` summary table

**Arguments**:
- `output: Option<PathBuf>` - Custom output path
- `directory: Option<PathBuf>` - Source directory

**Entry Point**: `pub fn run(args: ManifestArgs) -> Result<()>`

**Generates**: Markdown table summarizing all knowledge base entries

## Pattern
All subcommands follow this consistent pattern:
1. Accept a dedicated `*Args` struct
2. Expose a `pub fn run(args: *Args) -> Result<()>` function
3. Return `anyhow::Result` for error handling
4. Use the `fs` and `model` modules for operations
