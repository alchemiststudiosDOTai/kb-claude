# Code Smells to Fix


### 2. Duplicated Entry Type to Directory Mapping Logic
**Location:** `io/file_ops.rs` lines 62-77 and 84-91

**Problem:** The same match statement mapping entry types to directory names appears twice in the same file:
- Once in `get_entry_path()` (lines 64-73)
- Once in `list_entries()` (lines 84-91)

**Impact:** Medium-High - DRY violation, maintenance burden

**Fix:** Extract to a single function:
```rust
fn get_entry_subdir(entry_type: &str) -> Option<&'static str> {
    match entry_type {
        "metadata" => Some("metadata"),
        "debug" => Some("debug_history"),
        "qa" => Some("qa"),
        "delta" => Some("delta"),
        "code_index" => Some("code_index"),
        "pattern" => Some("patterns"),
        "cheatsheet" => Some("cheatsheets"),
        _ => None,
    }
}
```

---

### 3. Confusing Parameter Names for Different Entry Types
**Location:** `commands/add.rs` and `commands/update.rs`

**Problem:** Parameters named `--error` and `--solution` are reused to mean different things:
- For debug: actually error and solution
- For qa: question and answer
- For pattern: pattern name and description
- For cheatsheet: heading and content
- For code_index: file path and note

**Impact:** High - confusing API, poor user experience

**Examples:**
- `add.rs` line 94: `error` and `solution` used for heading/content in cheatsheet
- `update.rs` lines 48-51: `error` used as "question" for QA
- `update.rs` lines 83-86: `error` used as "heading" for cheatsheet

**Fix:** Either:
1. Use type-specific flags (e.g., `--question`, `--answer` for QA)
2. Use generic flags (e.g., `--field1`, `--field2`) with better documentation
3. Use subcommands for each entry type

---

## Priority 2: Code Quality Issues

### 4. Dead Code in Agent Protocol
**Location:** `agent/protocol.rs`

**Problem:** Multiple dead code suppressions:
- Line 4: `#[allow(dead_code)]` on `AgentRequest` struct
- Line 34: `#[allow(dead_code)]` on `error()` method
- Line 55: `#[allow(dead_code)]` on `with_data()` method

**Impact:** Low-Medium - unused code clutters codebase

**Fix:** Either implement usage of this code or remove it

---

### 5. Large Match Statements with Repetitive Logic
**Location:** `commands/add.rs` lines 24-104, `commands/update.rs` lines 29-97

**Problem:** Huge match statements with repetitive patterns for each entry type

**Impact:** Medium - hard to maintain and extend

**Fix:** Implement trait-based approach:
```rust
trait KbEntry {
    fn from_args(component: String, args: &Args) -> Result<Self>;
    fn update_from_args(&mut self, args: &Args) -> Result<()>;
}
```

---

### 6. Inconsistent Use of Option for last_updated
**Location:** Model structs

**Problem:**
- `Metadata` (line 14): `last_updated: DateTime<Utc>` (required)
- `Pattern` (line 11): `last_updated: Option<DateTime<Utc>>` (optional)
- `DebugHistory`: no last_updated field at all

**Impact:** Low-Medium - inconsistent data model

**Fix:** Standardize across all entry types (prefer required field with automatic updates)

---

### 7. Weak Validation Logic
**Location:** `schema/validator.rs`

**Problem:** Some validators are too permissive:
- `validate_pattern()` (lines 152-158): only checks for `component` field
- `validate_cheatsheet()` (lines 160-166): only checks for `component` field
- `validate_code_index()` (lines 144-150): only checks for `component` field

**Impact:** Medium - allows invalid data to pass validation

**Fix:** Add proper field validation for each type:
- Pattern should validate `patterns` array exists and has valid entries
- Cheatsheet should validate required fields
- CodeIndex should validate `files` array

---

### 8. Magic Number in Hash Display
**Location:** `commands/add.rs` line 108, `commands/update.rs` line 101, others

**Problem:** Hardcoded `[..8]` for hash truncation

**Impact:** Low - but should be a named constant

**Fix:**
```rust
const SHORT_HASH_LENGTH: usize = 8;
let short_hash = &hash[..SHORT_HASH_LENGTH];
```

---

## Priority 3: Documentation & Polish

### 9. Missing Documentation
**Location:** Throughout codebase

**Problem:** No rustdoc comments on:
- Public functions
- Public structs
- Modules

**Impact:** Low-Medium - makes code harder to understand and maintain

**Fix:** Add comprehensive rustdoc comments with examples

---

### 10. Inconsistent JSON vs Plain Output Handling
**Location:** All command handlers

**Problem:** Each command handler manually implements JSON vs plain output. Code is duplicated across files.

**Impact:** Low - but violates DRY

**Fix:** Create output formatting trait or helper functions

---

### 11. CLI Argument Structure Could Be Cleaner
**Location:** `main.rs` lines 22-39

**Problem:** Single `Add` command with many optional parameters makes the CLI confusing. Different entry types need different parameters but they're all optional at the type level.

**Impact:** Medium - confusing CLI UX, runtime errors instead of compile-time guarantees

**Fix:** Use subcommands:
```
claude-kb add metadata --component X --summary Y
claude-kb add debug --component X --error Y --solution Z
claude-kb add qa --component X --question Y --answer Z
```

---

### 12. Unused Cargo Features
**Location:** `Cargo.toml` line 7

**Problem:** `clap` includes `cargo` feature which may not be needed

**Impact:** Negligible - minimal bloat

**Fix:** Review and remove unused features

---

## Notes

- **Quick wins:** Items 2, 4, 8 can be fixed quickly
- **High impact:** Items 1, 3, 11 will significantly improve code quality and UX
- **Refactoring needed:** Items 5, 10 require more substantial refactoring

