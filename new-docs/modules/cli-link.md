---
title: Link Command
path: src/cli/link.rs
type: file
depth: 2
description: Establish bidirectional relationships between documents
exports: [run]
seams: [M]
---

## Where
`src/cli/link.rs`

## What
Implements the `kb-claude link` command, creating bidirectional ontological relationships between two existing documents.

## How

### Arguments (`LinkArgs`)
- `<SOURCE>`: First document's link slug
- `<TARGET>`: Second document's link slug
- `--force`: Overwrite existing relations

### `run()` Function Logic

1. **Document Resolution**
   - Search for SOURCE document by link slug
   - Search for TARGET document by link slug
   - Error if either not found

2. **Relation Check**
   - Check if SOURCE already relates to TARGET
   - Check if TARGET already relates to SOURCE
   - If relations exist and `--force` not set:
     - Error with message about existing relation
   - If `--force` set:
     - Remove existing relations before adding new ones

3. **Relation Creation**
   - Add `OntologicalRelation { relates_to: TARGET.link }` to SOURCE
   - Add `OntologicalRelation { relates_to: SOURCE.link }` to TARGET
   - Bidirectional linking ensures graph consistency

4. **Timestamp Update**
   - Call `touch_updated()` on both documents
   - Records modification time

5. **Persistence**
   - Serialize both documents to Markdown
   - Write back to original file paths
   - Use `to_markdown()` to preserve formatting

## Why
**Bidirectional Linking**: Creating relations in both directions:
- Enables graph traversal from either node
- Matches undirected graph mental model
- Supports "see also" navigation patterns

**Link Slug Lookup**: Using slugs instead of file paths:
- More stable (files can move)
- More user-friendly (rememberable)
- Abstracts filesystem details

**Force Flag**: Handling existing relations:
- Prevents accidental duplication
- Allows intentional correction
- Explicit user intent required for overwrite

**Timestamp Updates**: Touching `updated_at`:
- Indicates recently modified content
- Supports "what changed" queries
- Enables conflict detection in sync scenarios

**Ontological Relations**: Structured linking:
- More semantic than "see also" comments
- Machine-readable for graph analysis
- Supports dependency tracking
- Enables knowledge graph visualization

**Error Handling**: Clear error messages when:
- Documents not found
- Relations already exist
- File write failures occur

This prevents data corruption and helps users correct their commands.

**Use Cases**:
- Connect related debugging sessions
- Link patterns to examples
- Connect questions to answers
- Build concept maps
- Track dependencies between solutions
