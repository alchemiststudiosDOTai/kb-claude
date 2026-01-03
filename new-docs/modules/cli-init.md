---
title: Init Command
path: src/cli/init.rs
type: file
depth: 2
description: Initialize new .claude knowledge base directory structure
exports: [run]
seams: [M]
---

## Where
`src/cli/init.rs`

## What
Implements the `kb-claude init` command, which creates the `.claude` directory structure with all required subdirectories.

## How

### Arguments (`InitArgs`)
- `--directory <PATH>`: Optional custom location
- `--dry-run`: Preview changes without executing

### `run()` Function Logic

1. **Path Normalization**
   - Accept custom directory or default to `.`
   - Convert to absolute path

2. **Planning Phase**
   - List directories to be created:
     - `.claude/` root
     - All `CLAUDE_DIRECTORIES` (metadata, debug_history, qa, etc.)

3. **Dry Run Mode**
   - If `--dry-run` flag present:
     - Display planned operations
     - Exit without filesystem changes

4. **Execution**
   - Create `.claude` root using `fs::create_dir_all`
   - Invoke `ClaudePaths::ensure_layout()` to create subdirectories
   - Report created paths to user

## Why
**Onboarding Experience**: The `init` command is typically the first interaction users have, so it needs to be:
- Simple and predictable
- Safe (dry-run support)
- Clear about what it's doing

**Structure Enforcement**: By creating all directories upfront, we ensure:
- Consistent structure across projects
- No missing directory errors during document creation
- Clear expectation of where different document types live

**Dry Run Support**: Critical for:
- User confidence before making filesystem changes
- Debugging path resolution issues
- Integration testing without side effects

**Path Normalization**: Ensures the CLI works correctly whether invoked from:
- Project root
- Subdirectory
- With relative or absolute paths

**Error Handling**: Uses `anyhow::Context` to provide clear error messages when:
- Directory creation fails
- Permission issues occur
- Path resolution fails
