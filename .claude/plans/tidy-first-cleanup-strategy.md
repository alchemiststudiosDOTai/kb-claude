---
title: Tidy-First Cleanup Strategy
link: tidy-first-cleanup-strategy
type: plans
ontological_relations: []
tags: [refactoring, code-quality, tidy-first]
created_at: 2025-11-06T00:00:00Z
updated_at: 2025-11-06T00:00:00Z
uuid: 011CUrCb-sJJM-kJZE-zjbM-JCJB00000001
---

# Tidy-First Cleanup Strategy for kb-claude

This document outlines a comprehensive cleanup strategy using Kent Beck's "Tidy First?" micro-refactoring principles. Each task is designed to be small, focused, and parallelizable.

## 🔥 Priority 1: Delete Dead Code (Rule #2)

### Dead Functions in src/model.rs
**Location**: `src/model.rs:141-147`

Two public functions are exported but never used anywhere in the codebase:
- `pub fn new_uuid()` - line 141
- `pub fn now_timestamp()` - line 145

These are redundant since `Uuid::new_v4()` and `Utc::now()` are called directly throughout the codebase.

**Action**: Delete both functions entirely.

**Tidy-First Rule**: #2 - Delete Dead Code
**Risk**: Low (not referenced anywhere)
**Estimated Effort**: 5 minutes

---

## 🔁 Priority 2: Eliminate Code Duplication

### P2.1: Consolidate `display_relative` Functions (Rules #3, #6)

**Affected Files**:
- `src/cli/init.rs:97-107`
- `src/cli/new.rs:216-231`
- `src/cli/link.rs:143-147`

Three different implementations of the same concept exist with slightly different signatures:
1. `init.rs`: Returns `Cow<'a, str>`, handles workspace display
2. `new.rs`: Returns `String`, includes claude_dir_name parameter
3. `link.rs`: Returns `String`, simple implementation

**Action**:
1. Create a single `display_relative` function in `src/fs.rs`
2. Make it flexible enough to handle all three use cases
3. Replace all three implementations with calls to the shared version

**Tidy-First Rules**:
- #3 - Normalize Symmetries (make identical things look identical)
- #6 - Cohesion Order (group related path utilities together)

**Risk**: Low-medium (requires careful signature design)
**Estimated Effort**: 30 minutes

### P2.2: Extract Common "Find Claude Root" Pattern (Rule #4)

**Affected Files**: Every CLI command file except `init.rs` contains:
```rust
let cwd = std::env::current_dir().context("Unable to determine current directory")?;
let claude_root = find_existing_root(&cwd).unwrap_or_else(|| claude_root_from(&cwd));
```

**Locations**:
- `src/cli/validate.rs:12-15`
- `src/cli/new.rs:12-13`
- `src/cli/search.rs:12-13`
- `src/cli/link.rs:16-17`
- `src/cli/manifest.rs:12-14`

**Action**:
1. Create a helper function `resolve_claude_root_from_cwd()` in `src/fs.rs`
2. Replace all 5 duplicated patterns with a single function call

**Tidy-First Rules**:
- #3 - Normalize Symmetries
- #4 - New Interface, Old Implementation

**Risk**: Low
**Estimated Effort**: 20 minutes

### P2.3: Consolidate File Walking Logic (Rules #6, #8)

**Pattern**: Four CLI commands use nearly identical WalkDir patterns with similar filters:
- Skip non-files
- Skip manifest.md
- Skip ignored paths
- Filter for .md extensions
- Parse documents

**Locations**:
- `src/cli/search.rs:71-109` (`collect_documents`)
- `src/cli/link.rs:67-113` (`load_document`)
- `src/cli/validate.rs:81-140` (`collect_findings`)
- `src/cli/manifest.rs:51-104` (`collect_entries`)

**Action**:
1. Extract a shared `walk_kb_documents()` iterator in `src/fs.rs`
2. Make it return `Result<impl Iterator<Item = (PathBuf, Document)>>`
3. Let each command apply its own filtering/mapping logic

**Tidy-First Rules**:
- #6 - Cohesion Order
- #8 - Explaining Variable

**Risk**: Medium (affects multiple critical paths)
**Estimated Effort**: 60 minutes

---

## 🧠 Priority 3: Improve Code Clarity

### P3.1: Apply Guard Clauses (Rule #1)

**Location**: `src/cli/link.rs:115-134` - `insert_relation` function

The function has redundant logic:
```rust
if exists && !force {
    return false;
}

if !exists || force {
    relations.push(...);
    document.front_matter.touch_updated();
    return true;
}

false  // unreachable
```

**Action**: Simplify with guard clauses:
```rust
fn insert_relation(document: &mut Document, target_link: &str, force: bool) -> bool {
    let relations = &mut document.front_matter.ontological_relations;
    let exists = relations.iter().any(|r| r.relates_to == target_link);

    // Guard: early return if relation exists and not forcing
    if exists && !force {
        return false;
    }

    // Only add if doesn't exist or we're forcing
    if !exists || force {
        relations.push(OntologicalRelation {
            relates_to: target_link.to_string(),
        });
        document.front_matter.touch_updated();
    }

    true
}
```

**Tidy-First Rule**: #1 - Guard Clause
**Risk**: Low
**Estimated Effort**: 10 minutes

### P3.2: Extract Magic String Constants (Rule #9)

**Locations**:
- Error messages repeated across multiple files
- File extension checks: `.extension().is_none_or(|ext| ext != "md")` appears in 4 files
- String literals like `"Unable to determine current directory"` appear 6 times

**Action**: Create constants in `src/fs.rs`:
```rust
pub const MD_EXTENSION: &str = "md";
pub const CURRENT_DIR_ERROR: &str = "Unable to determine current directory";
pub const NO_CLAUDE_DIR_ERROR: &str = "No .claude directory found under {}. Run `kb-claude init` first.";
```

**Tidy-First Rule**: #9 - Explaining Constant
**Risk**: Very low
**Estimated Effort**: 20 minutes

### P3.3: Improve Validation Logic Readability (Rule #8)

**Location**: `src/cli/validate.rs:163-165`

Expression is overly clever:
```rust
if front.uuid.as_bytes().iter().all(|byte| *byte == 0) {
    findings.push(error(path, "`uuid` cannot be nil"));
}
```

**Action**: Extract explaining variable:
```rust
let is_nil_uuid = front.uuid.as_bytes().iter().all(|byte| *byte == 0);
if is_nil_uuid {
    findings.push(error(path, "`uuid` cannot be nil"));
}
```

**Tidy-First Rule**: #8 - Explaining Variable
**Risk**: Very low
**Estimated Effort**: 5 minutes

### P3.4: Improve Reading Order (Rule #5)

**Location**: `src/cli/init.rs`

Helper functions appear in this order:
1. `normalize_workspace`
2. `plan_layout`
3. `report_dry_run`
4. `report_execution`
5. `display_relative`

**Issue**: Functions are ordered by when they're defined, not when they're needed conceptually.

**Action**: Reorder to match the narrative flow:
1. `normalize_workspace` (path handling)
2. `display_relative` (path display helper)
3. `plan_layout` (planning)
4. `report_dry_run` (reporting - uses display_relative)
5. `report_execution` (reporting - uses display_relative)

**Tidy-First Rule**: #5 - Reading Order
**Risk**: Very low (pure reordering)
**Estimated Effort**: 5 minutes

---

## 📦 Priority 4: Structural Improvements

### P4.1: Create Path Utilities Module (Rules #6, #4)

**Action**: Extract all path-related utilities into a new internal module or expand `fs.rs`:
- `display_relative` variations
- `normalize_workspace`
- `resolve_claude_root_from_cwd`
- Path resolution helpers

This improves cohesion and makes the codebase easier to navigate.

**Tidy-First Rules**:
- #6 - Cohesion Order
- #4 - New Interface, Old Implementation

**Risk**: Low-medium (requires module organization)
**Estimated Effort**: 45 minutes

---

## 🎯 Suggested Parallel Work Streams

To maximize contributor productivity, these tasks can be done in parallel:

### Stream A: Dead Code Removal (Quick Wins)
1. P1: Delete unused functions in model.rs
2. P3.2: Extract magic strings to constants
3. P3.5: Add missing docstrings to public APIs

### Stream B: Guard Clauses & Simple Refactors
1. P3.1: Simplify `insert_relation` logic
2. P3.3: Extract explaining variables in validation
3. P3.4: Reorder functions in init.rs

### Stream C: Duplication Removal (Requires Coordination)
1. P2.1: Consolidate `display_relative`
2. P2.2: Extract "find claude root" pattern
3. P4.1: Create path utilities module

### Stream D: File Walking Consolidation (Bigger Refactor)
1. P2.3: Consolidate WalkDir patterns
2. Add comprehensive integration tests for the new shared logic

---

## 🚦 Implementation Guidelines

### For Each PR:
1. **Title Format**: `refactor(tidy): [Rule Name] - [Brief Description]`
   - Example: `refactor(tidy): Delete Dead Code - Remove unused helper functions`
2. **Keep Changes Small**: Each PR should address ONE tidy-first rule or ONE isolated area
3. **Run Tests**: Always run `cargo test` before committing
4. **Update This Document**: Mark tasks as completed with PR links

### Commit Message Template:
```
refactor(tidy): [Rule #N] - [Brief description]

Applies Tidy-First rule #N: [Rule Name]

Changes:
- [Specific change 1]
- [Specific change 2]

Files modified: [list]

Risk: [Low/Medium/High]
Tests: All passing ✓
```

### Testing Strategy:
- All existing integration tests must pass
- For structural changes (P2.3, P4.1), add new tests covering edge cases
- Use `cargo test -- --nocapture` to verify output formatting changes

---

## 📊 Summary Statistics

| Priority | Tasks | Est. Total Time | Risk Level |
|----------|-------|-----------------|------------|
| P1 (Dead Code) | 1 | 5 min | Low |
| P2 (Duplication) | 3 | 110 min | Low-Medium |
| P3 (Clarity) | 4 | 40 min | Very Low |
| P4 (Structure) | 1 | 45 min | Low-Medium |
| **TOTAL** | **9** | **~3.5 hours** | **Low-Medium** |

---

## 🔍 Next Steps

1. **Assign tasks** from the parallel work streams to different contributors
2. **Create GitHub issues** for each task using the template above
3. **Start with Priority 1** (dead code) as a warm-up and confidence builder
4. **Review PRs promptly** to avoid merge conflicts
5. **Update this document** as tasks are completed

---

## 📚 References

- Kent Beck's "Tidy First?" - Micro-refactoring principles
- Project CLAUDE.md - Repository guidelines
- Existing test suite in `tests/` for validation

---

**Document Status**: ✅ Ready for Implementation
**Last Updated**: 2025-11-06
**Maintainer**: Architecture Team
