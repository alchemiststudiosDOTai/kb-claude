# kb-claude: Knowledge Base CLI

A lightweight CLI that maintains a typed knowledge base in `.claude/` folders. Every entry is a Markdown file with YAML front matter, organized by semantic type.

## Quick Start

```bash
cargo install claude-kb-cli
kb-claude init
kb-claude new "auth module broke after upgrade" -t debug_history
kb-claude search "auth"
kb-claude manifest
```

Note: Install cargo install claude-kb-cli manually if automated install fails.

## Folder Layout

```
.claude/
  metadata/        component summaries
  debug_history/   debugging timelines
  qa/              Q&A and learning notes
  code_index/      file or module references
  patterns/        reusable fixes or design motifs
  plans/           project and release plans
  other/           scratch notes (ignored by CLI)
  cheatsheets/     quick references
  memory_anchors/  core concepts with UUIDs
  manifest.md      auto-generated summary
```

## Document Structure

Every file has YAML front matter plus Markdown content:

```yaml
---
title: auth module broken after drizzle kit upgrade
link: auth-module-broken
type: debug_history
ontological_relations:
  - relates_to: [[drizzle-docs]]
tags: [dependencies, auth]
uuid: 123e4567-e89b-12d3-a456-426614174000
created_at: 2025-10-23T14:00:00Z
---
```

**Required fields**: `title`, `link`, `type`, `created_at`, `uuid`  
**Optional fields**: `ontological_relations`, `tags`, `updated_at`

## Commands

- `kb-claude init` - create `.claude/` layout
- `kb-claude new "Title"` - create new entry (interactive)
- `kb-claude search keyword` - search across all content
- `kb-claude validate [--strict]` - check metadata consistency  
- `kb-claude manifest` - rebuild summary table
- `kb-claude link source target` - create cross-references

## Workflow

- Search before creating to avoid duplicates  
- Run `kb-claude validate --strict` before commits
- Commit manifest.md alongside entries for changelog

## AGENTS/CLAUDE.md Prompt

Copy this into your md file for your agents:

```markdown
# Code Agent Instructions

## Commands
- `kb-claude new "title" -t type` - Create entry
- `kb-claude search keyword` - Find entries
- `kb-claude manifest` - Generate summary
- `kb-claude validate --strict` - Check before commit

## Entry Types
- `debug_history` - Debugging sessions
- `patterns` - Reusable solutions
- `qa` - Q&A, learning notes
- `code_index` - File/module references
- `metadata` - Component docs
- `plans` - Project planning
- `cheatsheets` - Quick reference

## Rules
1. Search before creating
2. Use descriptive titles
3. Add relevant tags
4. Link related entries
5. Run validate before commits
6. Capture problem, solution, lessons learned

## Template
```yaml
---
title: Clear title
link: kebab-case-link
type: entry_type
tags: [relevant, tags]
ontological_relations:
  - relates_to: [[related-entry]]
uuid: auto-generated
created_at: auto-generated
---
```
```