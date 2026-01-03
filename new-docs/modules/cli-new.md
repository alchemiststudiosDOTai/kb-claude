---
title: New Command
path: src/cli/new.rs
type: file
depth: 2
description: Create new knowledge base documents with metadata
exports: [run]
seams: [M]
---

## Where
`src/cli/new.rs`

## What
Implements the `kb-claude new` command, creating new knowledge base documents with interactive prompts for metadata.

## How

### Arguments (`NewArgs`)
- `<TITLE>`: Required positional argument
- `-t, --type <TYPE>`: Document category
- `-g, --tag <TAG>`: Repeatable tags
- `--relates-to <LINK>`: Repeatable ontological relations
- `-f, --file <PATH>`: Output path override

### `run()` Function Logic

1. **Interactive Prompts** (if not provided via args)
   - Prompt for document type
   - Prompt for tags
   - Prompt for relations (document links)
   - Prompt for body content

2. **Path Resolution**
   - Determine target directory based on `doc_type`
   - Use type-specific subdirectory (e.g., `.claude/qa/`)
   - Respect `--file` override if provided

3. **Slug Generation**
   - Generate URL-friendly slug from title
   - Use `slugify()` function
   - Ensures consistent linking

4. **Front Matter Construction**
   - Create `DocumentFrontMatter`:
     - Auto-generate UUID
     - Set `created_at` and `updated_at` timestamps
     - Store title, link, type, tags, relations

5. **File Writing**
   - Format as Markdown with YAML front matter
   - Write to appropriate directory
   - Use `{link}.md` filename pattern

## Why
**Interactive vs Scriptable**: Supporting both:
- Interactive mode for manual usage (prompts)
- Flag-based mode for scripting/automation

**Type-Based Organization**: Placing documents in type-specific directories:
- Makes navigation intuitive
- Enables batch operations by type
- Mirrors user mental models

**Slug Consistency**: Using slugs for both:
- Filenames: `{link}.md`
- Front matter `link` field

This enables reliable lookups and prevents broken links.

**UUID Generation**: Ensures:
- Global uniqueness across distributed KBs
- Stable identifiers even if titles/links change
- Conflict-free synchronization

**Timestamp Tracking**: `created_at` and `updated_at` enable:
- Sorting by recency
- Detecting stale content
- Conflict resolution during sync

**Ontological Relations**: The `--relates-to` flag:
- Builds knowledge graph structure
- Enables semantic navigation
- Supports dependency tracking

**Tag Support**: Multiple tags allow:
- Cross-cutting categorization
- Flexible filtering in search
- Multiple organizational views
