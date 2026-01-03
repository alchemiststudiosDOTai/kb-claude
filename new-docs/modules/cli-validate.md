---
title: Validate Command
path: src/cli/validate.rs
type: file
depth: 2
description: Validate knowledge base integrity and consistency
exports: [run]
seams: [M]
---

## Where
`src/cli/validate.rs`

## What
Implements the `kb-claude validate` command, checking knowledge base documents for structural consistency and metadata integrity.

## How

### Arguments (`ValidateArgs`)
- `--directory <PATH>`: Custom KB location
- `--strict`: Treat warnings as errors (exit non-zero)

### `run()` Function Logic

1. **Document Collection**
   - Use `walk_kb_documents()` to iterate all KB files
   - Load parsed content for each

2. **Validation Checks**

   **Error-Level Checks**:
   - Missing or empty `title` field
   - Missing or empty `link` field
   - Missing or empty `type` field
   - Nil UUID (not generated correctly)

   **Warning-Level Checks**:
   - Unsupported document type (not in `CLAUDE_DIRECTORIES`)
   - Link inconsistent with filename slug
   - Link inconsistent with title slug
   - Document in wrong directory (type mismatch)

3. **Slug Consistency**
   - Compare `link` field to `slugify(title)`
   - Compare `link` field to filename (sans `.md`)
   - Report mismatches

4. **Directory Consistency**
   - Check file path against declared `type`
   - Example: `type: qa` should be in `.claude/qa/`
   - Report misplaced documents

5. **Reporting**
   - Collect all errors and warnings
   - Display count of each
   - List specific issues with file paths
   - In `--strict` mode: warnings count as errors

6. **Exit Codes**
   - 0: No errors
   - 1: Errors found (or warnings in strict mode)

## Why
**Data Integrity**: Validation prevents:
- Broken links from inconsistent slugs
- Lost documents from missing titles
- Sync conflicts from bad UUIDs

**Consistency Enforcement**: Checking slug/directory matches:
- Ensures findability
- Matches user expectations
- Prevents "file rot" through drift

**Error vs Warning Severity**:
- **Errors**: Critical issues that break functionality
- **Warnings**: Style issues or potential problems

This distinction allows users to:
- Fix critical problems first
- Decide whether to enforce strict standards

**Strict Mode**: The `--strict` flag enables:
- CI/CD integration (fail build on warnings)
- Enforcing team standards
- Preventing technical debt accumulation

**Type Validation**: Checking document types:
- Catches typos in `type` field
- Ensures only supported types exist
- Maintains organizational structure

**Slug Validation**: Comparing links to slugs:
- Catches manual title changes that didn't update links
- Prevents broken references
- Ensures URL-safe identifiers

**Directory Validation**: Checking file locations:
- Catches files moved manually
- Ensures type-based organization works
- Supports type-specific operations

**Error Messages**: Detailed reporting helps:
- Fix issues quickly
- Understand what's wrong
- Learn proper document structure

**CI/CD Integration**: Non-zero exit on errors enables:
- Pre-commit hooks
- Pipeline validation
- Automated quality gates
