---
title: Tidy-First Quick Reference
link: tidy-first-quick-reference
type: cheatsheets
ontological_relations:
  - relates_to: tidy-first-cleanup-strategy
tags: [refactoring, quick-reference]
created_at: 2025-11-06T00:00:00Z
updated_at: 2025-11-06T00:00:00Z
uuid: 011CUrCb-sJJM-kJZE-zjbM-JCJB00000002
---

# Tidy-First Quick Reference

Quick lookup for identified cleanup tasks. See `tidy-first-cleanup-strategy.md` for full details.

## 🎯 Quick Wins (< 15 minutes each)

| Task | File | Lines | Rule | Time |
|------|------|-------|------|------|
| Delete `new_uuid()` | `src/model.rs` | 141-143 | #2 | 2 min |
| Delete `now_timestamp()` | `src/model.rs` | 145-147 | #2 | 2 min |
| Simplify `insert_relation` | `src/cli/link.rs` | 115-134 | #1 | 10 min |
| Extract nil UUID check | `src/cli/validate.rs` | 163-165 | #8 | 5 min |
| Reorder init.rs functions | `src/cli/init.rs` | 34-107 | #5 | 5 min |
| Extract magic strings | Multiple files | - | #9 | 20 min |

## 🔄 Duplication Elimination (Medium Effort)

| Task | Affected Files | Rule | Time |
|------|----------------|------|------|
| Consolidate `display_relative` | init.rs, new.rs, link.rs | #3, #6 | 30 min |
| Extract "find root" pattern | 5 CLI files | #3, #4 | 20 min |
| Consolidate file walking | search, link, validate, manifest | #6, #8 | 60 min |

## 📋 10 Tidy-First Rules

1. **Guard Clause** - Flatten nested conditionals with early returns
2. **Delete Dead Code** - Remove unused code (VCS has your back)
3. **Normalize Symmetries** - Make identical things look identical
4. **New Interface, Old Implementation** - Design ideal APIs, delegate to legacy
5. **Reading Order** - Order code by narrative, not definition
6. **Cohesion Order** - Group tightly related code together
7. **Move Declaration & Initialization Together** - Keep vars close to use
8. **Explaining Variable** - Extract expressions into named variables
9. **Explaining Constant** - Replace magic values with named constants
10. **Explicit Parameters** - Pass inputs explicitly, avoid hidden state

## 🚀 Parallel Work Streams

### Stream A: Dead Code (Anyone, Low Risk)
- Delete `new_uuid()` and `now_timestamp()`
- Extract magic string constants
- Add missing docstrings

### Stream B: Simple Refactors (Anyone, Low Risk)
- Fix `insert_relation` guard clauses
- Extract explaining variables
- Reorder functions

### Stream C: Duplication (Coordinate, Medium Risk)
- Consolidate `display_relative`
- Extract "find root" pattern
- Create path utilities module

### Stream D: File Walking (Advanced, Medium Risk)
- Design shared file walking abstraction
- Refactor all 4 CLI commands
- Add comprehensive tests

## ✅ PR Checklist

- [ ] Title: `refactor(tidy): [Rule #N] - [Description]`
- [ ] Applied only ONE tidy-first rule
- [ ] Tests pass: `cargo test`
- [ ] Formatted: `cargo fmt`
- [ ] No new warnings: `cargo clippy`
- [ ] Updated this cheatsheet if completed

## 📊 Progress Tracker

| Priority | Completed | Total | % Done |
|----------|-----------|-------|--------|
| P1 (Dead Code) | 0 | 1 | 0% |
| P2 (Duplication) | 0 | 3 | 0% |
| P3 (Clarity) | 0 | 4 | 0% |
| P4 (Structure) | 0 | 1 | 0% |
| **TOTAL** | **0** | **9** | **0%** |

## 🔗 Related Documents

- Full strategy: `.claude/plans/tidy-first-cleanup-strategy.md`
- Repository guidelines: `CLAUDE.md`
- Test suite: `tests/`
