---
title: Root Directory
path: /
type: directory
depth: 0
description: Project root containing Rust CLI configuration, documentation, and knowledge base management
seams: [S]
---

# Root Directory

## Purpose
The root directory serves as the central hub for the `kb-claude` Rust CLI project, which manages a markdown-based knowledge base system. This is a self-documenting project that uses its own tooling to manage institutional memory.

## Organization

### Configuration Files
- **`Cargo.toml`** - Rust project manifest defining package metadata, dependencies, and build configuration. Explicitly excludes `.claude/` artifacts from published crate: `exclude = [".claude/*", ".claude/**/*"]`
- **`Cargo.lock`** - Dependency lock file for reproducible builds
- **`.gitignore`** - Version control exclusions including `/target`, `.env`, and `.claude/` dynamic content

### Documentation Files
- **`README.md`** - Quick start guide, folder layout explanation, document structure reference, and command overview
- **`CLAUDE.md`** - Comprehensive project guidelines covering development workflow, coding standards, testing practices, and commit conventions
- **`AGENTS.md`** - Agent-specific instructions and workflow documentation

### Top-Level Directories
- **`src/`** - Core Rust source code modules (main.rs, lib.rs, cli/, fs.rs, model.rs)
- **`tests/`** - Integration test suite using assert_cmd and assert_fs
- **`.claude/`** - Knowledge base root containing categorized markdown entries
- **`memory-bank/`** - Agent-managed system tracking research, plans, and execution logs
- **`new-docs/`** - Placeholder for future hierarchical documentation organization
- **`target/`** - Rust build output directory (excluded from git)

## Architectural Significance

The root directory embodies the project's "dogfooding" philosophy - it both contains the CLI tooling AND serves as a knowledge base managed by that tooling. The `.claude/` directory at root level demonstrates the tool eating its own dog food.

## Naming Conventions
- Configuration files use standard Rust conventions (`Cargo.toml`, `Cargo.lock`)
- Documentation uses `UPPERCASE.md` for project guidelines (`CLAUDE.md`, `AGENTS.md`)
- Knowledge base directory is dot-prefixed (`.claude/`) to indicate it's a system folder
- Build artifacts in `target/` follow Rust compiler conventions

## Relationships
- Parent to all project subdirectories
- Contains project-level metadata and configuration
- Entry point for both the binary (`src/main.rs`) and library (`src/lib.rs`)
- Root of the knowledge base tree (`.claude/`)
