---
title: src/prompt directory for Claude Code prompts
link: prompt-directory
type: metadata
ontological_relations:
- relates_to: dogfooding-knowledge-base
tags:
  - claude-code
  - prompts
  - tooling
created_at: 2025-10-24T13:25:00Z
updated_at: 2025-10-24T13:25:00Z
uuid: a1b2c3d4-e5f6-7890-abcd-ef1234567890
---

The `src/prompt/` directory stores Claude Code custom prompts that enhance the kb-claude development workflow.

**drive.md**: Core prompt that instructs Claude Code to use the kb-claude CLI for managing project knowledge. Defines the `.claude/` structure and standard commands (init, new, search, validate, manifest).

This enables Claude to automatically leverage the knowledge base system during development sessions.
