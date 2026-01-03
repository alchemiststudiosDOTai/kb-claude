---
title: Manifest Command
path: src/cli/manifest.rs
type: file
depth: 2
description: Generate manifest of all knowledge base documents
exports: [run]
seams: [M]
---

## Where
`src/cli/manifest.rs`

## What
Implements the `kb-claude manifest` command, generating a Markdown table listing all documents and their metadata.

## How

### Arguments (`ManifestArgs`)
- `--output <PATH>`: Custom output file location
- `-d, --directory <PATH>`: KB location

### `run()` Function Logic

1. **Document Collection**
   - Use `walk_kb_documents()` to iterate all KB files
   - Load parsed content for each

2. **Metadata Extraction**
   For each document, extract:
   - Title
   - Type
   - Tags (comma-separated)
   - Relations (count of `ontological_relations`)
   - Updated timestamp
   - Relative path from `.claude` root

3. **Sorting**
   - Sort documents by:
     - Type (alphabetical)
     - Then by updated date (newest first)

4. **Table Rendering**
   - Generate Markdown table with columns:
     - Title
     - Type
     - Tags
     - Relations
     - Updated
     - Path
   - Use standard Markdown table syntax

5. **Output**
   - Default: `.claude/manifest.md`
   - Respect `--output` override
   - Write table to file

## Why
**Knowledge Base Overview**: Manifest provides:
- Single-file view of all content
- Quick navigation aid
- Searchable index

**Markdown Format**: Using Markdown tables:
- Readable in text editors
- Rendered nicely in GitHub/GitLab
- Searchable with grep
- Version-controlled

**Sorting Strategy**:
- By type: Groups similar content
- By date: Shows recent activity
- Helps users find relevant content quickly

**Metadata Display**:
- **Title**: Identifies content
- **Type**: Enables filtering/grouping
- **Tags**: Cross-cutting categories
- **Relations**: Shows connected content
- **Updated**: Highlights recent changes
- **Path**: Enables direct file access

**Use Cases**:
- Onboarding: See what's in the KB
- Review: Check for stale content
- Navigation: Find specific documents
- Audit: Verify completeness
- Documentation: Auto-generated index

**Custom Output Path**: `--output` flag enables:
- Generate manifest in repo root
- Create multiple specialized manifests
- Integrate with docs pipelines

**Relation Counts**: Showing number of relations:
- Indicates well-connected content
- Identifies orphaned nodes (0 relations)
- Helps assess knowledge graph health

**Timestamp Display**: Showing `updated_at`:
- Identifies stale content
- Helps prioritize review
- Shows recent activity
- Supports "what's new" queries

**Tag Aggregation**: Comma-separated tags:
- Shows topical coverage
- Enables tag discovery
- Supports tag consistency checks

**Performance Considerations**:
- Full KB scan on each run
- Acceptable for manual invocation
- Could be cached if needed
- Simple implementation prioritized over speed
