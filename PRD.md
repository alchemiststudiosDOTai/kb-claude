Got it. Here’s the **revised PRD** with the project renamed from **Playbook** to **kb-claude** throughout, keeping the structure identical but aligned with your naming convention.

---

# **Product Requirements Document (PRD)**

### Project: **kb-claude (Rust MVP)**

### Version: v0.2

### Owner: Fabian / Alchemist Studios

### Date: 2025-10-23

---

## **1. Overview**

**kb-claude** is a Rust CLI program that automatically generates, organizes, and validates Markdown-based knowledge base files under a structured `.claude/` hierarchy.

Each document represents a typed, interlinked ontological entry and is stored according to its **type**, which corresponds directly to a `.claude/` directory.
This ensures predictable organization, consistent validation, and effortless searchability.

---

## **2. Goals**

- Automate creation of Markdown entries with enforced YAML metadata.
- Use `.claude/` directory names as **types** (e.g., `qa`, `patterns`, `metadata`).
- Enable tag- and keyword-based search.
- Maintain relational context through `relates_to` fields.
- Generate and update a manifest snapshot automatically.

---

## **3. .claude Layout**

```
.claude/
  metadata/        # component summaries
  debug_history/   # debugging timelines
  qa/              # question & answer entries
  code_index/      # file references
  patterns/        # reusable fixes or snippets
  cheatsheets/     # quick reference sections
  memory_anchors/  # concept anchors w/ UUID tracking
  manifest.md      # last sync snapshot
```

---

## **4. Document Schema**

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
We updated Drizzle for better CLI use, and doing so broke the auth module.
Currently rolled back; issue likely due to outdated syntax in `/src/auth/routes.ts`.
```

---

## **5. Core Features**

### **5.1 CLI Commands**

| Command              | Purpose                                            |
| -------------------- | -------------------------------------------------- |
| `kb-claude init`     | Create `.claude/` structure if missing             |
| `kb-claude new`      | Create a new ontological Markdown file             |
| `kb-claude search`   | Search all `.claude/` files by keywords or tags    |
| `kb-claude link`     | Create bidirectional relations between two entries |
| `kb-claude validate` | Verify schema compliance                           |
| `kb-claude manifest` | Rebuild the manifest snapshot                      |

---

## **6. Behavior**

### **6.1 Type-based Directory Enforcement**

- `type` determines storage location (`type: debug_history` → `.claude/debug_history/filename.md`).
- Validation fails if `type` is not one of the `.claude/` subdirectories.

### **6.2 File Creation Flow**

1. User runs:

   ```bash
   kb-claude new "auth module broken after drizzle kit upgrade"
   ```

2. CLI prompts for:

   - type (suggested from `.claude/` dirs)
   - tags
   - relations

3. File is created with UUID, timestamps, and front-matter validation.

### **6.3 Manifest Management**

```
| Title | Type | Path | Tags | Relations | Updated |
|-------|------|------|------|------------|----------|
| auth module broken... | debug_history | .claude/debug_history/auth-module-broken.md | auth, dependencies | drizzle-docs | 2025-10-23 |
```

### **6.4 Search**

```
kb-claude search auth drizzle
```

Returns ranked results with filename, title, tags, and type.

### **6.5 Validation**

- Required fields: `title`, `link`, `type`, `uuid`
- `type` must match a `.claude/` directory
- Slugified filename = `link`
- Proper YAML syntax enforced

---

## **7. Technical Implementation**

### **7.1 Stack**

- **Language:** Rust
- **Dependencies:**

  - `serde_yaml` – YAML parsing
  - `uuid` – unique ID generation
  - `walkdir` – recursive traversal
  - `chrono` – timestamps
  - `clap` – CLI parsing
  - `glob` – search utilities

### **7.2 Core Modules**

| Module         | Purpose                  |
| -------------- | ------------------------ |
| `cli.rs`       | CLI commands and args    |
| `fs.rs`        | Directory + file I/O     |
| `models.rs`    | YAML front-matter struct |
| `validator.rs` | Schema rules             |
| `search.rs`    | Keyword + tag search     |
| `manifest.rs`  | Manifest generation      |

---

## **8. Example Usage**

```bash
$ kb-claude init
✅ Created .claude/ structure

$ kb-claude new "auth module broken after drizzle kit upgrade"
> Type: debug_history
> Tags: dependencies, auth
✅ Created .claude/debug_history/auth-module-broken.md

$ kb-claude search drizzle
1. .claude/debug_history/auth-module-broken.md — tags: dependencies, auth

$ kb-claude validate
✅ All 12 files valid
```

---

## **9. Future Enhancements**

- LLM-assisted classification and relation creation
- Ontological graph visualization
- Integration with Tantivy or SQLite for semantic search
- Git-aware manifest diffing
- ACE-style Reflector/Curator agents for auto-evolving `.claude/` entries

---

Would you like me to add the **CLI argument and flag specification table** next (for `kb-claude new`, `search`, `link`, etc.)?
