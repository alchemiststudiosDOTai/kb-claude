# kb-claude: Ontological Knowledge Base CLI

## 1. Philosophy

`kb-claude` is not a notes app or a static site generator. It is a _living knowledge base manager_ for engineering projects—equal parts changelog, research archive, and design journal.

Every entry is an **ontological Markdown file**: a typed piece of knowledge that belongs inside a shared mental model. Instead of scattered notes, the tool enforces a single rule:

> If it matters, it belongs in `.claude/`, and it must declare a `type`.

Each `.claude/` subfolder represents a semantic category (debug history, patterns, QA, code index, and so on). By naming these folders explicitly, we get predictable, searchable locations for every insight a team creates.

## 2. What It Does

`kb-claude` automates three responsibilities:

1. **Creation** – Generates Markdown files with validated YAML front matter describing what happened, why, and how it connects to other knowledge.
2. **Organization** – Stores every file under the correct `.claude/` folder based on its `type`.
3. **Maintenance** – Provides CLI commands that validate, search, and summarize the knowledge base.

The result is a transparent, text-native system that thrives in version control.

## 3. Folder Layout

```
.claude/
  metadata/        component summaries
  debug_history/   debugging timelines
  qa/              Q&A and learning notes
  code_index/      file or module references
  patterns/        reusable fixes or design motifs
  cheatsheets/     quick references or how-tos
  memory_anchors/  core concepts tracked by UUID
  manifest.md      automatically generated summary
```

Each subdirectory is a distinct knowledge type. When creating entries, `type:` must match one of these folder names.

## 4. Document Structure

Every `.md` file starts with YAML front matter followed by narrative Markdown content.

```yaml
---
title: auth module broken after drizzle kit upgrade
link: auth-module-broken
type: debug_history
ontological_relations:
  - relates_to: [[drizzle-docs]]
  - relates_to: [[dependency-docs]]
tags:
  - dependencies
  - auth
created_at: 2025-10-23T14:00:00Z
updated_at: 2025-10-23T14:00:00Z
uuid: 123e4567-e89b-12d3-a456-426614174000
---
```

- **title** – human-readable summary.
- **link** – slug that doubles as filename.
- **type** – must align with a `.claude/` subfolder.
- **ontological_relations** – wiki-style cross-links (`[[slug]]`).
- **tags** – keywords for search.
- **uuid** – auto-generated for traceability.

The body is free-form Markdown: logs, analysis, diagrams, etc.

## 5. Core Commands

- `kb-claude init` – create the `.claude/` layout in a repo.
- `kb-claude new "Title"` – guided prompt for new entries; handles tags, relations, timestamps, UUIDs, and file placement.
- `kb-claude search keyword` – case-insensitive search across titles, tags, relations, and body text.
- `kb-claude validate [--strict]` – parse every entry, confirm required metadata, and flag inconsistencies (e.g., slug mismatch).
- `kb-claude manifest` – rebuild `.claude/manifest.md`, a table summarizing every document.
- `kb-claude link source target` – insert reciprocal relations between two slugs.

## 6. Daily Workflow Tips

- Treat `.claude/` as the project’s institutional memory. Capture debugging sessions, architecture decisions, and recurring Q&A.
- Before adding content, run `kb-claude search` to avoid duplication.
- Run `kb-claude validate --strict` before committing to keep the KB clean.
- The manifest acts like a changelog of insights—commit it alongside entries.

## 7. Design Principles

1. **Text over tools** – Markdown and YAML are the only storage formats.
2. **Structure without rigidity** – required metadata, free body text.
3. **Traceable knowledge** – everything has UUIDs and timestamps.
4. **Search-first** – think of `.claude/` as a local wiki you can grep.
5. **Small knowledge commits** – frequent, incremental captures beat perfect essays.
6. **Longevity** – the folder should remain useful in plain text for years.

## 8. Extending the Tool

Future directions: semantic search (Tantivy/SQLite), ontology graphs, automated summarizers, git-aware manifest diffs, or HTML/Notion exporters. The current MVP focuses on correctness, structure, and speed.

## 9. Getting Started

Install from [crates.io](https://crates.io/crates/claude-kb-cli):

```bash
cargo install claude-kb-cli
```

Then run:

```bash
kb-claude init
kb-claude new "First Entry" -t metadata
kb-claude manifest
```

That’s enough to see the typed knowledge base take shape. Everything else—searching, validating, linking—builds on that foundation.
