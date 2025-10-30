---
title: Validation Whitelist Architecture
link: validation-whitelist-architecture
type: metadata
ontological_relations: []
tags:
  - validation
  - architecture
  - v0.3.4
created_at: 2025-10-30T17:15:22Z
updated_at: 2025-10-30T17:15:22Z
uuid: a7297745-34db-4880-bb01-ca6d25c6844b
---

## Change Summary

Switched validation from blacklist to whitelist approach in v0.3.4.

**Before:** Explicitly ignored `other/` directory only
**After:** Only processes 8 known KB directories, ignores everything else in `.claude/`

## Implementation

Modified `src/fs.rs`:
- Removed `IGNORED_DIRECTORIES` constant
- Changed `is_ignored_path()` logic: `!CLAUDE_DIRECTORIES.contains(&name)`

## Impact

All commands now ignore unknown directories (`.claude/.max-claude/`, `.claude/scratch/`, etc.):
- `validate` - skips unknown dirs
- `manifest` - excludes unknown dirs
- `search` - won't search unknown dirs
- `link` - won't scan unknown dirs

## Release

Published as v0.3.4 on 2025-10-30 to crates.io.
