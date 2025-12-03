# Research – CLI Output Standardization
**Date:** 2025-12-03
**Owner:** Claude Agent
**Phase:** Research
**Git Commit:** b584c8c

## Goal
Analyze the current CLI output formatting patterns across kb-claude commands to identify inconsistencies, document existing conventions, and evaluate standardization options.

## Findings

### Current Output Pattern Inventory

#### Files with Output Statements

| File | Output Calls | Primary Pattern |
|------|--------------|-----------------|
| `src/cli/new.rs` | 7 | Interactive prompts + success |
| `src/cli/init.rs` | 3 | Header + prefixed list items |
| `src/cli/validate.rs` | 3 | Labeled findings + summary |
| `src/cli/search.rs` | 2 | Numbered results |
| `src/cli/link.rs` | 2 | Bidirectional arrow notation |
| `src/cli/manifest.rs` | 1 | Simple confirmation |

### Existing Helpers & Constants

**`src/fs.rs:115-121`** - `display_relative()`:
```rust
pub fn display_relative(workspace: &Path, path: &Path) -> String {
    match path.strip_prefix(workspace) {
        Ok(relative) if relative.as_os_str().is_empty() => ".".to_string(),
        Ok(relative) => format!("./{}", relative.display()),
        Err(_) => path.display().to_string(),
    }
}
```
- Used by: `init.rs`, `new.rs`, `link.rs`
- NOT used by: `manifest.rs` (absolute), `validate.rs` (custom reimplementation), `search.rs` (custom reimplementation)

**`src/fs.rs:20-26`** - Error constants:
```rust
pub const CURRENT_DIR_ERROR: &str = "Unable to determine current directory";
pub const NO_CLAUDE_DIR_ERROR: &str = "No .claude directory found under {}. Run `kb-claude init` first.";
```

### Detailed Pattern Analysis by Command

#### 1. init.rs - Dual-Mode Prefixes
**Location:** `src/cli/init.rs:67-89`

| Mode | Empty State | Header | Item Prefix |
|------|-------------|--------|-------------|
| DryRun | "Dry run: .claude hierarchy already exists at" | "Dry run: would initialize .claude hierarchy under" | `"  + "` |
| Execution | "No changes needed; .claude hierarchy already present at" | "Initialized .claude hierarchy under" | `"  created "` |

**Output Example:**
```
Initialized .claude hierarchy under /path/.claude
  created ./.claude/metadata
  created ./.claude/debug_history
```

#### 2. new.rs - Interactive + Success
**Location:** `src/cli/new.rs:57,70-92`

- Interactive prompts: `"Select type:"`, `"  - {item}"`, `"Type [{default}]: "`
- Success: `"Created {relative_path}"`
- Auto-init: `"No existing knowledge base detected; created layout at {absolute_path}"`

**Inconsistency:** Uses absolute path for auto-init but relative for created file.

#### 3. link.rs - Bidirectional Notation
**Location:** `src/cli/link.rs:39-55`

- Success: `"Linked {source} <-> {target}"`
- No-change: `"Relations already existed between \`{source}\` and \`{target}\`; no changes made."`

**Unique Element:** `<->` arrow for bidirectional relationships.

#### 4. manifest.rs - Simple Confirmation
**Location:** `src/cli/manifest.rs:33`

- Success: `"Wrote manifest to {absolute_path}"`

**Issue:** Only command using absolute path for success message.

#### 5. validate.rs - Labeled Output
**Location:** `src/cli/validate.rs:206-218`

```rust
println!("{label}: {display} — {}", finding.message);
// label = "error" | "warning"
```

**Pattern:** `<severity>: <path> — <message>`

**Example:**
```
error: ./.claude/metadata/test.md — Missing `title`
warning: ./.claude/qa/file.md — `link` should match file name
```

#### 6. search.rs - Numbered Results
**Location:** `src/cli/search.rs:34-47`

```rust
println!(
    "{}. {} — {} (type: {}, tags: {})",
    index + 1, item.path.display(), item.title, item.doc_type, tags
);
```

**Empty value:** Uses em-dash `"—"` for empty tags.

### Inconsistency Summary

| Aspect | Inconsistency | Affected Commands |
|--------|---------------|-------------------|
| **Path Display** | Mix of absolute and relative | manifest (abs), validate header (abs), new auto-init (abs) |
| **Helper Usage** | Duplicate relative path logic | search.rs:93-102, validate.rs:212-216 |
| **Success Prefix** | No prefix vs `"  created "` vs `"  + "` | All commands |
| **Message Style** | Complete sentence vs verb+object | validate (sentence) vs new/link (verb) |
| **Error Paths** | No standard for bail! path format | All commands |

### Symbol Inventory

| Symbol | Usage | Location |
|--------|-------|----------|
| `"  + "` | Dry-run will create | init.rs:72 |
| `"  created "` | Actually created | init.rs:77 |
| `"  - "` | List bullet | new.rs:72 |
| `"<->"` | Bidirectional link | link.rs:52 |
| `"{n}. "` | Numbered item | search.rs:36 |
| `"error:"` | Error severity | validate.rs:209 |
| `"warning:"` | Warning severity | validate.rs:210 |
| `"—"` | Empty value / separator | search.rs:42, validate.rs:217 |

### Missing Symbols (Not Used)
- `✓` (checkmark) - Success indicator
- `✗` (cross) - Failure indicator
- `→` (arrow) - Action indicator
- Color/ANSI codes - None used

### Dependencies Analysis

**Cargo.toml shows NO formatting libraries:**
- No `colored`, `console`, `indicatif`, `termcolor`
- Only `anyhow` for error handling, `clap` for parsing

## Key Patterns / Solutions Found

### Pattern 1: Two-Space Indentation
- Used consistently for sub-items in init.rs and new.rs
- Format: `"  {prefix}{content}"`

### Pattern 2: Em-Dash Separator
- Used in validate.rs and search.rs for separating components
- Format: `" — "` (with spaces)

### Pattern 3: Backtick Quoting
- Used in error messages for user-provided values
- Format: `` `{value}` ``

### Pattern 4: Relative Path Display
- Standard format: `./{relative_path}`
- Helper exists but not universally used

## Knowledge Gaps

1. **No stderr usage**: All output goes to stdout, including errors (via anyhow)
2. **No color support**: Unclear if this is intentional design choice or oversight
3. **No progress indicators**: Long operations don't show progress
4. **Testing patterns**: Integration tests in `tests/smoke.rs` verify specific strings - any changes require test updates

## Proposed Standardization Options

### Option 1: Constants Module (User's Proposal)
```rust
// src/cli/output.rs
pub mod constants {
    pub const DRY_RUN_PREFIX: &str = "  + ";
    pub const CREATED_PREFIX: &str = "  created ";
    pub const SUCCESS_PREFIX: &str = "✓ ";
    pub const ERROR_PREFIX: &str = "✗ ";
    pub const ACTION_PREFIX: &str = "→ ";
}
```

**Pros:**
- Simple, low-overhead
- Easy to audit and maintain
- No behavioral changes

**Cons:**
- Doesn't address path consistency
- Unicode symbols may have terminal compatibility issues
- Still requires manual println! calls

### Option 2: Output Helper Module
```rust
// src/cli/output.rs
pub fn success(msg: &str) { println!("✓ {}", msg); }
pub fn created(path: &Path, workspace: &Path) {
    println!("  created {}", display_relative(workspace, path));
}
pub fn error_finding(path: &Path, msg: &str, workspace: &Path) {
    println!("✗ {} — {}", display_relative(workspace, path), msg);
}
```

**Pros:**
- Enforces consistent path formatting
- Centralizes output logic
- Easier to add color later

**Cons:**
- More invasive changes
- May over-engineer simple cases

### Option 3: Minimal Fixes Only
- Standardize `display_relative()` usage across all commands
- Add constants for commonly repeated strings
- Keep current output style unchanged

**Pros:**
- Smallest change footprint
- Preserves existing test compatibility

**Cons:**
- Doesn't improve visual consistency

## References

### Primary Files (Output Implementation)
- `src/cli/init.rs:67-89` - Report mode switching
- `src/cli/new.rs:57,70-92` - Interactive and success output
- `src/cli/link.rs:39-55` - Bidirectional messaging
- `src/cli/manifest.rs:33` - Simple confirmation
- `src/cli/validate.rs:206-218` - Labeled findings
- `src/cli/search.rs:29-47` - Numbered results

### Helper Infrastructure
- `src/fs.rs:115-121` - `display_relative()` helper
- `src/fs.rs:20-26` - Error message constants

### Testing
- `tests/smoke.rs` - Integration tests verify specific output strings

### Dependencies
- `Cargo.toml:15-23` - No formatting crate dependencies
