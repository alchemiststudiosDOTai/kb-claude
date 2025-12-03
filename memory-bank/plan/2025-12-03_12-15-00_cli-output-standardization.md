# Plan – CLI Output Standardization

---
title: "CLI Output Standardization – Plan"
phase: Plan
date: "2025-12-03T12:15:00Z"
owner: "Claude Agent"
parent_research: "memory-bank/research/2025-12-03_12-09-03_cli-output-standardization.md"
git_commit_at_plan: "b584c8c"
tags: [plan, cli, output, standardization]
---

## Goal

**Singular Focus:** Standardize path display across all CLI commands by universally adopting the existing `display_relative()` helper, eliminating duplicate implementations.

**Non-Goals:**
- Adding color/ANSI support
- Changing output message wording
- Adding new Unicode symbols (✓, ✗)
- Creating an elaborate output abstraction layer

## Scope & Assumptions

**In Scope:**
- Replace custom relative-path logic in `search.rs` and `validate.rs` with `display_relative()`
- Change `manifest.rs` to use relative path instead of absolute
- Ensure all success/error messages use consistent `./{path}` format

**Out of Scope:**
- New dependencies (colored, console, etc.)
- Refactoring message content or prefixes
- Progress indicators
- stderr vs stdout changes

**Assumptions:**
- `display_relative()` in `src/fs.rs:115-121` is the canonical implementation
- Integration tests in `tests/smoke.rs` may need path format updates
- No breaking changes to CLI behavior, only cosmetic path formatting

## Deliverables (DoD)

| Artifact | Acceptance Criteria |
|----------|---------------------|
| Updated `search.rs` | Uses `display_relative()` for result paths |
| Updated `validate.rs` | Uses `display_relative()` for finding paths |
| Updated `manifest.rs` | Outputs relative path instead of absolute |
| Updated `new.rs` | Auto-init message uses relative path |
| All tests pass | `cargo test` green |
| No new warnings | `cargo clippy -- -D warnings` clean |

## Readiness (DoR)

- [x] Research document complete with file locations
- [x] `display_relative()` helper exists at `src/fs.rs:115-121`
- [x] Test suite exists at `tests/smoke.rs`
- [x] No external dependencies required

## Milestones

| ID | Milestone | Description |
|----|-----------|-------------|
| M1 | Core Standardization | Update search.rs, validate.rs, manifest.rs to use `display_relative()` |
| M2 | Edge Case Fix | Update new.rs auto-init message for consistency |
| M3 | Validation | Run tests, clippy, and manual verification |

## Work Breakdown (Tasks)

### Task 1: Update `search.rs` path display
- **Owner:** Developer
- **Milestone:** M1
- **Dependencies:** None
- **Files:** `src/cli/search.rs:34-47`

**Work:**
- Import `display_relative` from `crate::fs`
- Replace custom path formatting at lines 93-102 with `display_relative(workspace, &item.path)`
- Remove redundant path logic

**Acceptance Tests:**
- `cargo run -- search <term>` outputs paths as `./{relative}` format
- No regression in search functionality

---

### Task 2: Update `validate.rs` path display
- **Owner:** Developer
- **Milestone:** M1
- **Dependencies:** None
- **Files:** `src/cli/validate.rs:206-218`

**Work:**
- Import `display_relative` from `crate::fs`
- Replace custom path display logic at lines 212-216 with `display_relative(workspace, &finding.path)`

**Acceptance Tests:**
- `cargo run -- validate` outputs finding paths as `./{relative}` format
- Error/warning formatting unchanged

---

### Task 3: Update `manifest.rs` to use relative path
- **Owner:** Developer
- **Milestone:** M1
- **Dependencies:** None
- **Files:** `src/cli/manifest.rs:33`

**Work:**
- Import `display_relative` from `crate::fs`
- Change success message from absolute to relative path

**Acceptance Tests:**
- `cargo run -- manifest` outputs `Wrote manifest to ./.claude/manifest.md` (relative)

---

### Task 4: Update `new.rs` auto-init message
- **Owner:** Developer
- **Milestone:** M2
- **Dependencies:** Task 1-3 complete
- **Files:** `src/cli/new.rs:57`

**Work:**
- Change auto-init message to use relative path format for consistency

**Acceptance Tests:**
- Auto-init message uses `./{path}` format

---

### Task 5: Validation & Testing
- **Owner:** Developer
- **Milestone:** M3
- **Dependencies:** Tasks 1-4

**Work:**
- Run `cargo test` - update any hardcoded path assertions in smoke tests if needed
- Run `cargo clippy -- -D warnings`
- Run `cargo fmt`
- Manual smoke test all commands

**Acceptance Tests:**
- All tests pass
- No clippy warnings
- Code formatted

## Risks & Mitigations

| Risk | Impact | Likelihood | Mitigation | Trigger |
|------|--------|------------|------------|---------|
| Test assertions break | Medium | High | Update test expectations to use relative paths | Tests fail after changes |
| Windows path separators | Low | Low | `display_relative()` uses `display()` which handles OS paths | CI failure on Windows |

## Test Strategy

**Single Integration Test Focus:**
- Verify `cargo run -- validate` outputs relative paths in findings
- This exercises the most complex path-display change and validates the helper integration

## References

- Research: `memory-bank/research/2025-12-03_12-09-03_cli-output-standardization.md`
- Helper: `src/fs.rs:115-121` (`display_relative()`)
- Test file: `tests/smoke.rs`

---

## Alternative Option (Documented for Context)

**Option 2 from Research:** Create `src/cli/output.rs` with helper functions. This was rejected because:
1. Over-engineers a simple path consistency fix
2. Adds abstraction without immediate benefit
3. Project guidelines emphasize avoiding over-engineering

---

## Summary

| Attribute | Value |
|-----------|-------|
| Plan Path | `memory-bank/plan/2025-12-03_12-15-00_cli-output-standardization.md` |
| Milestones | 3 |
| Tasks | 5 |
| Gate | All tests pass, clippy clean |
| Next Command | `/context-engineer:execute "memory-bank/plan/2025-12-03_12-15-00_cli-output-standardization.md"` |
