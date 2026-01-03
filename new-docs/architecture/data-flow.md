---
title: Data Flow and Execution
path: /Users/tuna/kb-claude/new-docs/architecture/data-flow.md
type: metadata
depth: 1
description: How data flows through the system during command execution
seams: ["architecture/overview", "architecture/module-structure"]
---

# Data Flow and Execution

This document traces how data moves through the kb-claude system during typical operations.

## Command Execution Flow

### Generic Command Pipeline

Every CLI command follows the same execution pipeline:

```
┌─────────────────────────────────────────────────────────────┐
│ 1. Shell Invocation                                         │
│    $ kb-claude <command> [args]                             │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ 2. Entry Point (main.rs)                                    │
│    fn main() -> Result<()> {                                │
│        claude_kb_cli::cli::run()                            │
│    }                                                         │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ 3. CLI Parsing (cli/mod.rs)                                 │
│    let cli = Cli::parse();  // clap derives this            │
│    // Validates arguments, generates help text              │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ 4. Command Dispatch (cli/mod.rs)                            │
│    match cli.command {                                      │
│        Command::New(args) => new::run(args),                │
│        Command::Search(args) => search::run(args),          │
│        // ...                                               │
│    }                                                         │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ 5. Subcommand Execution (cli/*.rs)                          │
│    // Business logic for specific command                   │
│    // Interacts with fs and model layers                   │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ 6. Result Propagation                                       │
│    // Ok(()) printed to stdout                              │
│    // Err(e) formatted by anyhow and printed to stderr      │
└─────────────────────────────────────────────────────────────┘
```

## Document Creation Flow

### Command: `kb-claude new "My Title" -t qa`

```
┌─────────────────────────────────────────────────────────────┐
│ USER INPUT                                                  │
│ - CLI arguments: title, type, tags, relations              │
│ - Interactive prompts: body text                            │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ PATH RESOLUTION (cli/new.rs)                                │
│ resolve_claude_root_from_cwd()                              │
│ → Searches up directory tree for .claude/                   │
│ → Returns (cwd, claude_root)                                │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ AUTO-INIT (if needed)                                       │
│ ClaudePaths::ensure_layout()                                │
│ → Creates .claude/ if missing                               │
│ → Creates subdirectories: qa/, metadata/, etc.              │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ TYPE DETERMINATION (cli/new.rs)                             │
│ determine_type()                                            │
│ → Use --type flag if provided                               │
│ → Else: Interactive prompt with known types                 │
│ → Validates type is in CLAUDE_DIRECTORIES                   │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ METADATA COLLECTION (cli/new.rs)                            │
│ - collect_tags(): Parse --tag flags or prompt               │
│ - collect_relations(): Parse --relates-to flags or prompt   │
│ - collect_body(): Interactive prompt until empty line       │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ MODEL INSTANTIATION (model.rs)                              │
│ DocumentFrontMatter::new(title, doc_type)                   │
│ → slugify(title) → link                                     │
│ → Uuid::new_v4() → uuid                                     │
│ → Utc::now() → created_at, updated_at                       │
│ → Set tags, relations                                       │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ PATH COMPUTATION (cli/new.rs)                               │
│ compute_output_path()                                       │
│ → If --file flag: use that path                             │
│ → Else: layout.type_directory(doc_type) / link + ".md"      │
│ → Example: .claude/qa/my-title.md                           │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ SERIALIZATION (model.rs)                                    │
│ document.to_markdown()                                      │
│ → serde_yaml::to_string(front_matter)                       │
│ → Format: "---\n{yaml}\n---\n{body}\n"                      │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ FILE WRITE (cli/new.rs)                                     │
│ fs::write(output_path, content)                             │
│ → Creates parent directories if needed                      │
│ → Writes complete markdown file                             │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ FEEDBACK                                                    │
│ println!("Created {relative_path}")                         │
└─────────────────────────────────────────────────────────────┘
```

## Document Reading and Parsing Flow

### Used by: search, validate, manifest, link commands

```
┌─────────────────────────────────────────────────────────────┐
│ DISCOVERY (fs.rs)                                           │
│ walk_kb_documents(claude_root)                              │
│ → WalkDir::new(claude_root)                                 │
│ → Iterate all files recursively                             │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ FILTERING (fs.rs)                                           │
│ For each file:                                              │
│ 1. Skip if not a file                                       │
│ 2. Skip if manifest.md                                      │
│ 3. Skip if not in CLAUDE_DIRECTORIES                        │
│ 4. Skip if extension != .md                                 │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ FILE READ (fs.rs)                                           │
│ fs::read_to_string(path)                                    │
│ → Returns raw markdown content                              │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ PARSING (model.rs)                                          │
│ Document::parse(raw_content)                                │
│ → Split on "\n---\n" delimiter                              │
│ → Extract YAML front matter block                           │
│ → serde_yaml::from_str(yaml) → DocumentFrontMatter          │
│ → Remainder is body                                         │
│ → Return Document { front_matter, body }                    │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ DOCUMENT ENTRY (fs.rs)                                      │
│ DocumentEntry {                                             │
│     path,                                                   │
│     document,                                               │
│ }                                                            │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ ITERATOR CONSUMER (cli/*.rs)                                │
│ Collect into Vec or process stream                          │
└─────────────────────────────────────────────────────────────┘
```

## Search Execution Flow

### Command: `kb-claude search "auth" --tag dependency`

```
┌─────────────────────────────────────────────────────────────┐
│ TERM PREPARATION (cli/search.rs)                            │
│ - Lowercase search terms: ["auth"]                          │
│ - Lowercase tag filters: ["dependency"]                     │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ DOCUMENT LOADING (cli/search.rs)                            │
│ collect_documents(claude_root)                              │
│ → walk_kb_documents()                                       │
│ → Parse all documents                                       │
│ → Collect into Vec<DocumentEntry>                           │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ FILTERING AND SCORING (cli/search.rs)                       │
│ For each document:                                          │
│                                                            │
│ 1. BUILD SEARCH BLOB                                        │
│    build_search_blob(front_matter, body)                   │
│    → Concatenate: title + link + type + body               │
│    → Add: tags (space-separated)                            │
│    → Add: relations (space-separated)                       │
│    → Lowercase everything                                   │
│                                                            │
│ 2. TAG FILTERING                                            │
│    → If tag_filters specified:                              │
│      - Check ALL tags present in document.tags             │
│      - Skip document if any tag missing                     │
│                                                            │
│ 3. TERM MATCHING                                            │
│    → For each search term:                                  │
│      - Check term present in search blob                   │
│      - Skip document if any term missing                    │
│      - Count occurrences for scoring                        │
│                                                            │
│ 4. SCORING                                                  │
│    → Sum of all term occurrences                            │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ SORTING (cli/search.rs)                                     │
│ matches.sort_by(|a, b| {                                    │
│     b.score.cmp(&a.score)      // Descending score         │
│         .then_with(|| a.title.cmp(&b.title))  // Ascending  │
│ })                                                           │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ OUTPUT (cli/search.rs)                                      │
│ For each match:                                             │
│   println!(                                                 │
│       "{i}. {path} — {title} (type: {type}, tags: {tags})"  │
│   )                                                          │
└─────────────────────────────────────────────────────────────┘
```

## Validation Flow

### Command: `kb-claude validate --strict`

```
┌─────────────────────────────────────────────────────────────┐
│ PATH RESOLUTION (cli/validate.rs)                           │
│ resolve_claude_root(directory)                              │
│ → Use --directory flag or current directory                │
│ → Find .claude/ parent                                      │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ DOCUMENT COLLECTION (cli/validate.rs)                       │
│ collect_findings(claude_root, layout)                       │
│ → walk_kb_documents()                                       │
│ → For each document: validate_document()                    │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ VALIDATION RULES (cli/validate.rs)                          │
│ validate_document(path, claude_root, layout, document)     │
│                                                            │
│ CHECKS (in order):                                          │
│                                                            │
│ ERROR CONDITIONS:                                           │
│ 1. title is empty                                           │
│ 2. link is empty                                            │
│ 3. doc_type is empty                                        │
│ 4. uuid is nil (all bytes are 0)                           │
│ 5. doc_type not in CLAUDE_DIRECTORIES                      │
│ 6. file directory != doc_type                               │
│                                                            │
│ WARNING CONDITIONS:                                         │
│ 1. link != file stem (filename without .md)                │
│ 2. link != slugify(title)                                   │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ FINDING COLLECTION (cli/validate.rs)                        │
│ Vec<Finding> {                                              │
│     path,                                                   │
│     message,                                                │
│     severity: Error | Warning,                              │
│ }                                                            │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ REPORTING (cli/validate.rs)                                 │
│ print_findings(findings, workspace)                         │
│ → For each finding:                                         │
│   println!("{severity}: {path} — {message}")                │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ EXIT CODE DECISION (cli/validate.rs)                        │
│ if error_count > 0:                                         │
│     bail!("Validation failed with {error_count} errors")    │
│ if strict && warning_count > 0:                             │
│     bail!("Validation failed with {warning_count} warnings")│
│ else:                                                       │
│     println!("Validation completed with {warnings} warnings")│
└─────────────────────────────────────────────────────────────┘
```

## Manifest Generation Flow

### Command: `kb-claude manifest`

```
┌─────────────────────────────────────────────────────────────┐
│ DOCUMENT COLLECTION (cli/manifest.rs)                       │
│ collect_entries(claude_root)                                │
│ → walk_kb_documents()                                       │
│ → For each document: extract metadata                      │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ METADATA EXTRACTION (cli/manifest.rs)                       │
│ ManifestEntry {                                             │
│     title: front_matter.title.clone(),                      │
│     doc_type: front_matter.doc_type.clone(),                │
│     relative_path: path.strip_prefix(workspace),            │
│     tags: front_matter.tags.clone(),                        │
│     relations: front_matter.ontological_relations          │
│         .iter().map(|r| r.relates_to.clone()).collect(),    │
│     updated_at: front_matter.updated_at.date_naive(),       │
│ }                                                            │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ SORTING (cli/manifest.rs)                                   │
│ entries.sort_by(|a, b| {                                    │
│     a.title.to_lowercase().cmp(&b.title.to_lowercase())     │
│ })                                                           │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ TABLE RENDERING (cli/manifest.rs)                           │
│ render_manifest(claude_root, entries)                       │
│ → Create markdown table header                              │
│ → For each entry: create table row                          │
│   | Title | Type | Path | Tags | Relations | Updated |     │
│ → If empty: | *(empty)* | - | - | - | - | - |              │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ FILE WRITE (cli/manifest.rs)                                │
│ resolve_output_path(base_dir, layout, override)            │
│ → Use --output flag if provided                             │
│ → Else: layout.manifest_path()                              │
│                                                            │
│ fs::write(output_path, manifest_content)                    │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ FEEDBACK                                                    │
│ println!("Wrote manifest to {relative_path}")               │
└─────────────────────────────────────────────────────────────┘
```

## Link Creation Flow

### Command: `kb-claude link source-doc target-doc`

```
┌─────────────────────────────────────────────────────────────┐
│ VALIDATION (cli/link.rs)                                    │
│ if source == target:                                        │
│     bail!("Source and target must be different")            │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ DOCUMENT LOADING (cli/link.rs)                              │
│ load_document(claude_root, "source-doc")                    │
│ → walk_kb_documents()                                       │
│ → Find document where link == "source-doc"                  │
│ → Return DocumentEntry                                      │
│                                                            │
│ load_document(claude_root, "target-doc")                    │
│ → Same process for target                                   │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ RELATION INSERTION (cli/link.rs)                            │
│ insert_relation(&mut source, target_link, force)            │
│ → Check if relation already exists                          │
│ → If exists and !force: return false                        │
│ → Else: push new OntologicalRelation                        │
│   → source.front_matter.ontological_relations.push(OntologicalRelation { │
│         relates_to: target_link.to_string(),                │
│     })                                                       │
│   → source.front_matter.touch_updated()                     │
│   → return true                                             │
│                                                            │
│ (Repeat for target → source)                                │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ SERIALIZATION (model.rs)                                    │
│ source.document.to_markdown()                               │
│ target.document.to_markdown()                               │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ FILE WRITE (cli/link.rs)                                    │
│ write_document(&source)                                     │
│ → fs::write(source.path, content)                           │
│                                                            │
│ write_document(&target)                                     │
│ → fs::write(target.path, content)                           │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ FEEDBACK                                                    │
│ println!("Linked {source} <-> {target}")                    │
└─────────────────────────────────────────────────────────────┘
```

## Error Propagation Flow

### How errors flow through the system

```
┌─────────────────────────────────────────────────────────────┐
│ ERROR GENERATION                                            │
│ anyhow::bail!("message")                                    │
│ Err(anyhow!("message"))                                     │
│ .with_context(|| "context")                                 │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ PROPAGATION (via ? operator)                                │
│ fn inner() -> Result<()> {                                  │
│     let value = operation()?;  // Propagates if Err        │
│     Ok(())                                                  │
│ }                                                            │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ CONTEXT ADDITION                                             │
│ fs::write(path, content)                                    │
│     .with_context(|| format!("Unable to write {}", path))?; │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ BUBBLE TO MAIN                                              │
│ All errors propagate up call stack to main() via ?          │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ↓
┌─────────────────────────────────────────────────────────────┐
│ FINAL HANDLING                                              │
│ fn main() -> Result<()> {                                   │
│     cli::run()  // Returns Result                           │
│ }                                                            │
│ → anyhow automatically prints error to stderr               │
│ → Returns non-zero exit code                                │
└─────────────────────────────────────────────────────────────┘
```

## Data Transformation Examples

### Title to Link Transformation

```
INPUT: "Fix: auth module broke after upgrade!"
  ↓
slugify() in model.rs
  ↓
TRANSFORMATIONS:
  1. Trim: "Fix: auth module broke after upgrade!"
  2. Lowercase alphanumeric: "fix", "auth", "module", "broke", "after", "upgrade"
  3. Replace spaces/separators with '-': "fix-auth-module-broke-after-upgrade"
  4. Remove trailing '-': "fix-auth-module-broke-after-upgrade"
  5. Fallback for empty: "untitled"
  ↓
OUTPUT: "fix-auth-module-broke-after-upgrade"
```

### Document Serialization

```
INPUT: Document {
    front_matter: DocumentFrontMatter {
        title: "My Document",
        link: "my-document",
        doc_type: "qa",
        tags: ["rust", "cli"],
        created_at: "2025-01-03T12:00:00Z",
        uuid: "123e4567-e89b-12d3-a456-426614174000",
        ...
    },
    body: "# Content\n\nSome text",
}
  ↓
to_markdown() in model.rs
  ↓
SERIALIZATION:
  1. serde_yaml::to_string(front_matter)
     → "---\ntitle: My Document\nlink: my-document\n..."
  2. Remove leading "---\n"
  3. Format with delimiters
  ↓
OUTPUT:
---
title: My Document
link: my-document
type: qa
tags:
  - rust
  - cli
created_at: 2025-01-03T12:00:00Z
uuid: 123e4567-e89b-12d3-a456-426614174000
---
# Content

Some text
```

## Performance Characteristics

### Time Complexity

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| `kb-claude new` | O(1) | Single file write |
| `kb-claude init` | O(n) | n = number of directories (8) |
| `kb-claude search` | O(d × f) | d = documents, f = avg file size |
| `kb-claude validate` | O(d) | d = documents |
| `kb-claude manifest` | O(d log d) | d = documents (sorting dominates) |
| `kb-claude link` | O(d) | d = documents (scan for source/target) |

### Space Complexity

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| `kb-claude new` | O(1) | Single document in memory |
| `kb-claude search` | O(d × s) | d = documents, s = avg size |
| `kb-claude validate` | O(d × s) | d = documents, s = avg size |
| `kb-claude manifest` | O(d) | Metadata only, not full content |

### Optimization Opportunities

If performance becomes an issue:

1. **Search acceleration**: Add inverted index
   - Map terms → document IDs
   - O(1) lookup per term
   - Trade-off: Index maintenance on writes

2. **Incremental validation**: Track which files changed
   - Use filesystem timestamps or git status
   - Only validate modified documents
   - Trade-off: State management complexity

3. **Lazy loading**: Stream documents instead of loading all
   - Use iterator pattern more aggressively
   - Process documents as discovered
   - Trade-off: Multiple passes for some operations

4. **Caching**: Cache parsed documents
   - In-memory cache with TTL
   - Avoid re-parsing on repeated operations
   - Trade-off: Memory usage, cache invalidation
