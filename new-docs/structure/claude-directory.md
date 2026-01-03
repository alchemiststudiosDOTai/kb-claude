---
title: Claude Knowledge Base Directory
path: .claude/
type: directory
depth: 1
description: Managed knowledge base with categorized markdown entries and YAML front matter
seams: [S]
---

# Claude Knowledge Base Directory (`.claude/`)

## Purpose
The core knowledge base storage for the `kb-claude` system. Contains categorized markdown documents with structured YAML front matter, enabling searchable, cross-referenced institutional memory. This directory embodies the project's "dogfooding" philosophy - the CLI tool manages its own development knowledge.

## Organization

### Subdirectories by Entry Type

#### `metadata/` - Component Summaries
High-level information about project components, releases, and architecture.

**Examples:**
- `release-0-3-0-prep.md` - Release planning and preparation
- `validation-whitelist-architecture.md` - Architecture decisions
- `claude-md-added-as-main-project-guidelines.md` - Process documentation

**Use Case:** Project meta-information, release notes, architectural decisions

#### `debug_history/` - Debugging Timelines
Records of debugging sessions with context, symptoms, investigations, and resolutions.

**Examples:**
- `first-run-initialization.md` - Initial setup debugging

**Use Case:** Capturing debugging context for future reference, preventing repeat investigations

#### `qa/` - Q&A and Learning Notes
Questions, answers, and lessons learned during development.

**Examples:**
- `how-to-search-the-kb.md` - Usage documentation

**Use Case:** Quick reference for common questions and procedural knowledge

#### `code_index/` - File and Module References
Indexes of significant code files, modules, or architectural components.

**Examples:**
- `cli-module-index.md` - Documentation of `src/cli/mod.rs`

**Use Case:** Mapping code structure to documentation, creating searchable code references

#### `patterns/` - Reusable Solutions
Design motifs, recurring fixes, and architectural patterns.

**Examples:**
- `dogfooding-cli-workflow.md` - Development workflow patterns

**Use Case:** Documenting reusable solutions and best practices

#### `plans/` - Project and Release Plans
Detailed development plans, strategies, and roadmaps.

**Examples:**
- `tidy-first-cleanup-strategy.md` - Code cleanup strategy
- `2025-12-03_12-15-00_cli-output-standardization.md` - Timestamped specific plans

**Use Case:** Tracking development intentions and execution strategies

#### `other/` - Scratch Notes
Temporary notes ignored by CLI validation (original behavior, now shifted to whitelist approach).

**Examples:**
- `ignored-note.md` - Example ignored file

**Use Case:** Drafting content before categorization, temporary thoughts

#### `cheatsheets/` - Quick References
Condensed command references and procedural guides.

**Examples:**
- `cli-command-cheatsheet.md` - CLI usage quick reference
- `tidy-first-quick-reference.md` - Refactoring guidelines

**Use Case:** Rapid lookup of common commands and procedures

#### `memory_anchors/` - Core Concepts
Foundational concepts tracked by UUID for cross-referencing.

**Examples:**
- `dogfooding-anchor.md` - Core dogfooding principle

**Use Case:** Defining and tracking key conceptual pillars of the project

### Root-Level Files
- **`manifest.md`** - Auto-generated summary/index of all KB entries, created by `kb-claude manifest` command

## Document Structure

### YAML Front Matter Schema
Every `.md` file contains structured metadata:

```yaml
---
title: Document Title
link: slugified-document-title
type: metadata | debug_history | qa | code_index | pattern | plan | cheatsheet | memory_anchor
ontological_relations:
  - type: relates_to | depends_on | extends | refines
    target: uuid-or-link
tags: [tag1, tag2, tag3]
created_at: 2025-12-03T12:00:00Z
updated_at: 2025-12-03T12:00:00Z
uuid: 12345678-1234-1234-1234-123456789abc
---

# Document Content

Markdown body content here...
```

### Required Fields
- `title` - Human-readable title
- `type` - Entry type determining directory placement
- `created_at` - ISO 8601 timestamp
- `uuid` - Unique identifier for cross-referencing

### Optional Fields
- `link` - URL-safe slug (defaults to slugified title)
- `ontological_relations` - Links to related documents
- `tags` - Free-form tags for categorization
- `updated_at` - Last modification timestamp

## Naming Conventions

### File Naming
- **Format**: `kebab-case-title.md`
- **Source**: Derived from `title` field via `slugify()` function
- **Alternative**: `link` field in front matter overrides default
- **Timestamping**: Some files use `YYYY-MM-DD_HH-MM-00_description.md` (especially in `plans/`)

### Directory Naming
- All lowercase with underscores
- Semantic names matching entry types
- Consistent with `CLAUDE_DIRECTORIES` constant in `src/fs.rs`

### Front Matter Field Naming
- **Standard**: `snake_case` for all field names
- **Type field**: `type` (renamed to `doc_type` in Rust to avoid keyword conflict)
- **Relations**: `ontological_relations` (plural underscore convention)

## Architectural Significance

### Type-Based Organization
Each directory corresponds to an entry type, enforced by:
- `CLAUDE_DIRECTORIES` constant in `src/fs.rs`
- Validation logic in `src/cli/validate.rs`
- Type selection prompts in `src/cli/new.rs`

### Whitelist Validation
Per `validation-whitelist-architecture.md`:
- Original behavior: Ignore non-standard directories
- Current behavior: Whitelist approach for known types
- Ensures only valid entry types are processed

### Cross-Referencing
Documents can reference each other via:
- `ontological_relations` with UUIDs
- `uuid` field for stable references
- `link` field for human-readable references

## Relationships

### Input/Output Relationships
- **Created by**: `kb-claude new` command
- **Indexed by**: `kb-claude manifest` generates `manifest.md`
- **Validated by**: `kb-claude validate` checks schema integrity
- **Searched via**: `kb-claude search` queries content and metadata

### Directory Relationships
- **Siblings**: `memory-bank/` (parallel system for agent workflows)
- **Parent**: Root directory (`/`)
- **Excluded from**: Published crate (per `Cargo.toml` exclude directive)

## Lifecycle

1. **Creation**: User runs `kb-claude new`, selects type, provides title
2. **Initialization**: CLI generates front matter with UUID, timestamp, slug
3. **Editing**: User's editor opens for content entry
4. **Storage**: File saved to appropriate `.claude/<type>/` directory
5. **Linking** (optional): `kb-claude link` adds relationships
6. **Indexing**: `kb-claude manifest` updates search index
7. **Validation**: `kb-claude validate` ensures integrity

## Usage Patterns

### During Development
- Capture debugging sessions in `debug_history/`
- Document decisions in `metadata/`
- Store reusable solutions in `patterns/`

### For Releases
- Plan releases in `plans/` (e.g., `release-X-Y-Z-prep.md`)
- Track release notes in `metadata/`
- Validate all entries before tagging

### For Onboarding
- New developers review `cheatsheets/` for quick starts
- `qa/` answers common questions
- `memory_anchors/` explains core concepts
