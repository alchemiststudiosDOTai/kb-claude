---
title: Search Command
path: src/cli/search.rs
type: file
depth: 2
description: Search knowledge base by terms and tags
exports: [run]
seams: [M]
---

## Where
`src/cli/search.rs`

## What
Implements the `kb-claude search` command, searching documents by content terms and tags with relevance scoring.

## How

### Arguments (`SearchArgs`)
- `<TERM>...`: Required, one or more search terms
- `-t, --tag <TAG>`: Repeatable tag filters

### `run()` Function Logic

1. **Document Collection**
   - Use `walk_kb_documents()` to iterate all KB files
   - Load each document's parsed content

2. **Searchable Blob Construction**
   - Concatenate front matter fields:
     - Title
     - Type
     - Tags
     - Relations
   - Append body content
   - Create unified search text

3. **Filtering**
   - **Tag Filter**: Include only docs matching ALL specified tags
   - **Term Filter**: Case-insensitive substring matching
     - Search term must appear in blob
     - Multiple terms = AND logic (all must match)

4. **Scoring**
   - Count occurrences of each search term
   - Sum across all terms
   - Higher score = more relevant

5. **Display**
   - Sort results by score (descending)
   - For each match, display:
     - Path (relative to `.claude`)
     - Title
     - Type
     - Tags
     - Match score

## Why
**Unified Search Context**: By combining front matter and body into a blob:
- Single search covers all content
- Metadata fields are discoverable
- No separate "search metadata" vs "search content" needed

**Tag Filtering**: Tag-only searches enable:
- Browse by category
- List all docs of a type
- Cross-reference related concepts

**Relevance Scoring**: Simple occurrence counting:
- Easy to understand
- Fast to compute
- Good enough for personal KBs
- Could be enhanced with TF-IDF or ranking later

**Case Insensitivity**: Makes search:
- More user-friendly
- Match user expectations
- Find content regardless of capitalization

**AND Logic for Multiple Terms**: Requiring all terms:
- Narrows results as user adds terms
- Progressive refinement workflow
- Matches typical search mental models

**Path Display**: Showing relative paths:
- Easier to read than absolute paths
- Indicates location within KB structure
- Helps users navigate to files

**Performance Considerations**:
- Linear scan is acceptable for personal KBs (< 10K docs)
- Could be optimized with indexing if needed
- Current approach prioritizes simplicity over speed
