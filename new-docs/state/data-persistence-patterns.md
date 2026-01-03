---
title: Data Persistence and File Storage Patterns
path: /Users/tuna/kb-claude/new-docs/state/data-persistence-patterns.md
type: state-analysis
depth: 1
description: Analysis of serialization, file I/O, and data lifecycle in kb-claude
seams: []
---

# Data Persistence and File Storage Patterns

## Overview

kb-claude uses **the filesystem as the database**. All persistent state is stored as Markdown files with YAML front matter, organized in a hierarchical directory structure under `.claude/`.

## Data Model

### Document Structure

Every knowledge base entry is a Markdown file:

```markdown
---
title: "Example Document"
link: "example-document"
type: "metadata"
tags:
  - tag1
  - tag2
ontological_relations:
  - relates_to: "other-document"
created_at: "2025-01-03T00:00:00Z"
updated_at: "2025-01-03T12:30:00Z"
uuid: "550e8400-e29b-41d4-a716-446655440000"
---

# Document Body

This is the Markdown content of the document.
```

### Rust Representation

```rust
pub struct Document {
    pub front_matter: DocumentFrontMatter,
    pub body: String,
}

pub struct DocumentFrontMatter {
    pub title: String,
    pub link: String,
    pub doc_type: String,
    pub ontological_relations: Vec<OntologicalRelation>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub uuid: Uuid,
}
```

## File Organization

### Directory Structure

```
.claude/
├── metadata/           # Component documentation
├── debug_history/      # Debugging session records
├── qa/                # Q&A and learning notes
├── code_index/        # File/module references
├── patterns/          # Reusable solutions
├── plans/             # Project planning documents
├── cheatsheets/       # Quick reference guides
├── memory_anchors/    # Core concepts with UUIDs
├── other/             # Scratch notes (ignored)
└── manifest.md        # Generated index
```

### File Naming Convention

Files are named using URL-friendly slugs derived from titles:

```
"My Awesome Document" → "my-awesome-document.md"
```

**Implementation** (`src/model.rs`):

```rust
pub fn slugify(input: &str) -> String {
    let mut slug = String::new();
    let mut pending_dash = false;

    for ch in input.trim().chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            pending_dash = false;
        } else if matches!(ch, ' ' | '-' | '_' | '.') && !slug.is_empty() && !pending_dash {
            slug.push('-');
            pending_dash = true;
        }
    }

    if slug.ends_with('-') {
        slug.pop();
    }

    if slug.is_empty() {
        "untitled".to_string()
    } else {
        slug
    }
}
```

### Path Resolution

**Type-based organization**:

```rust
// Document type determines subdirectory
pub fn type_directory(&self, doc_type: &str) -> PathBuf {
    self.root.join(doc_type)
}

// Full path computation
.claude/<doc_type>/<link>.md
// Example: .claude/metadata/my-awesome-document.md
```

## Serialization/Deserialization

### Serialization: Struct → Markdown

**Location**: `src/model.rs`

```rust
impl Document {
    pub fn to_markdown(&self) -> Result<String> {
        // 1. Serialize front matter to YAML
        let yaml = serde_yaml::to_string(&self.front_matter)
            .with_context(|| "Unable to serialize document front matter")?;

        // 2. Remove leading delimiter added by serde_yaml
        let yaml_trimmed = yaml.trim_start_matches(&format!("{FRONT_MATTER_DELIMITER}\n"));

        // 3. Combine: delimiters + YAML + body
        Ok(format!(
            "{delim}\n{front}{delim}\n{body}\n",
            delim = FRONT_MATTER_DELIMITER,
            front = yaml_trimmed,
            body = self.body.trim_end()
        ))
    }
}
```

**Custom Date Serialization**:

```rust
pub mod iso8601 {
    use chrono::{DateTime, SecondsFormat, Utc};
    use serde::{Serializer, Deserializer};

    pub fn serialize<S>(value: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Format: "2025-01-03T12:30:00Z"
        let formatted = value.to_rfc3339_opts(SecondsFormat::Secs, true);
        serializer.serialize_str(&formatted)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        DateTime::from_str(&raw).map_err(serde::de::Error::custom)
    }
}
```

### Deserialization: Markdown → Struct

```rust
impl Document {
    pub fn parse(raw: &str) -> Result<Self> {
        let trimmed = raw.trim_start();

        // 1. Extract YAML block between delimiters
        let rest = trimmed
            .strip_prefix(FRONT_MATTER_DELIMITER)
            .ok_or_else(|| anyhow!("Document missing starting front matter delimiter"))?;

        let rest = rest
            .strip_prefix('\n')
            .ok_or_else(|| anyhow!("Front matter must start on a new line"))?;

        let (yaml_block, body) = rest
            .split_once(FRONT_MATTER_BLOCK_BREAK)
            .ok_or_else(|| anyhow!("Document missing closing front matter delimiter"))?;

        // 2. Parse YAML front matter
        let front_matter: DocumentFrontMatter = serde_yaml::from_str(yaml_block)
            .with_context(|| "Unable to parse document front matter as YAML")?;

        // 3. Return Document with parsed components
        Ok(Self {
            front_matter,
            body: body.to_string(),
        })
    }
}
```

## File I/O Operations

### Reading Documents

**Location**: `src/fs.rs`

```rust
pub fn walk_kb_documents(claude_root: &Path)
    -> impl Iterator<Item = Result<DocumentEntry>> + '_
{
    WalkDir::new(claude_root)
        .into_iter()
        .filter_map(move |entry| {
            let entry = entry.ok()?;
            let path = entry.path();

            // Skip non-files
            if !entry.file_type().is_file() {
                return None;
            }

            // Skip manifest file
            if path.file_name().is_some_and(|name| name == MANIFEST_FILE) {
                return None;
            }

            // Skip ignored directories
            if is_ignored_path(path, claude_root) {
                return None;
            }

            // Skip non-markdown files
            if path.extension().is_none_or(|ext| ext != MD_EXTENSION) {
                return None;
            }

            // Read file content
            let content = match fs::read_to_string(path) {
                Ok(c) => c,
                Err(e) => {
                    return Some(Err(anyhow::anyhow!(
                        "Unable to read {}: {}",
                        path.display(),
                        e
                    )));
                }
            };

            // Parse document
            let document = match Document::parse(&content) {
                Ok(doc) => doc,
                Err(e) => {
                    return Some(Err(anyhow::anyhow!(
                        "Unable to parse {}: {}",
                        path.display(),
                        e
                    )));
                }
            };

            Some(Ok(DocumentEntry {
                path: path.to_path_buf(),
                document,
            }))
        })
}
```

**Key Features**:
- Lazy iteration (on-demand reading)
- Comprehensive filtering
- Error propagation with context
- Type-safe result handling

### Writing Documents

**Location**: `src/cli/new.rs`

```rust
pub fn run(args: NewArgs) -> Result<()> {
    // 1. Resolve paths
    let (cwd, claude_root) = resolve_claude_root_from_cwd()?;
    let layout = ClaudePaths::new(claude_root.clone());

    // 2. Ensure directory structure exists
    if !claude_root.exists() {
        layout.ensure_layout()?;
    }

    // 3. Create document state
    let mut front_matter = DocumentFrontMatter::new(&args.title, doc_type);
    front_matter.tags = tags;
    front_matter.ontological_relations = relations;
    front_matter.ensure_link_matches_slug();

    // 4. Compute output path
    let output_path = compute_output_path(&cwd, &layout, &mut front_matter, args.file_override.as_ref())?;

    // 5. Ensure parent directories exist
    ensure_parent_dirs(&output_path)?;

    // 6. Check for existing file
    if output_path.exists() {
        bail!(
            "A document already exists at {}; choose a different title or override path",
            output_path.display()
        );
    }

    // 7. Serialize document
    let document = Document::new(front_matter, body);
    let content = document.to_markdown()?;

    // 8. Write to disk
    fs::write(&output_path, content)
        .with_context(|| format!("Unable to write {}", output_path.display()))?;

    println!("Created {}", display_relative(&workspace, &output_path));

    Ok(())
}
```

**Safety Checks**:
- Directory creation before write
- Collision detection (existing file check)
- Contextual error messages
- Relative path display for user feedback

## Data Lifecycle

### Creation Flow

```
User Input (stdin/CLI args)
    ↓
DocumentFrontMatter::new()
    ↓
Document::new()
    ↓
Document::to_markdown()
    ↓
fs::write()
    ↓
.md file on disk
```

**Code Trace**:

```rust
// 1. Create front matter
let mut front_matter = DocumentFrontMatter::new(&args.title, doc_type);

// 2. Set metadata
front_matter.tags = tags;
front_matter.ontological_relations = relations;
front_matter.touch_updated();

// 3. Compose document
let document = Document::new(front_matter, body);

// 4. Serialize
let content = document.to_markdown()?;

// 5. Persist
fs::write(&output_path, content)?;
```

### Reading Flow

```
Filesystem (.md file)
    ↓
fs::read_to_string()
    ↓
Document::parse()
    ↓
Document struct in memory
    ↓
Processing (search/validate/etc.)
```

**Code Trace**:

```rust
// 1. Traverse filesystem
for entry in walk_kb_documents(&claude_root) {
    let entry = entry?;

    // 2. Access parsed document
    let doc = &entry.document;
    let front = &doc.front_matter;

    // 3. Process
    println!("{}: {}", front.title, front.doc_type);
}
```

### Modification Flow

```
Existing .md file
    ↓
fs::read_to_string()
    ↓
Document::parse()
    ↓
Mutate DocumentFrontMatter
    ↓
Document::to_markdown()
    ↓
fs::write() (overwrite)
```

**Example: Link Command** (`src/cli/link.rs`)

```rust
// 1. Read both documents
let source_doc = load_document(&args.source, &claude_root)?;
let target_doc = load_document(&args.target, &claude_root)?;

// 2. Create relation
let source_rel = OntologicalRelation {
    relates_to: target_doc.front_matter.link.clone(),
};
let target_rel = OntologicalRelation {
    relates_to: source_doc.front_matter.link.clone(),
};

// 3. Modify state
source_doc.front_matter.ontological_relations.push(source_rel);
target_doc.front_matter.ontological_relations.push(target_rel);
source_doc.front_matter.touch_updated();
target_doc.front_matter.touch_updated();

// 4. Serialize and write
let source_content = source_doc.to_markdown()?;
let target_content = target_doc.to_markdown()?;
fs::write(&source_path, source_content)?;
fs::write(&target_path, target_content)?;
```

## Error Handling

### Contextual Error Messages

**Pattern**: Use `anyhow::Context` to enrich errors

```rust
fs::write(&output_path, content)
    .with_context(|| format!("Unable to write {}", output_path.display()))?;
```

**Output**:

```
Error: Unable to write .claude/metadata/my-doc.md

Caused by:
    Permission denied (os error 13)
```

### Validation Errors

**Example**: Document structure validation

```rust
pub fn parse(raw: &str) -> Result<Self> {
    let trimmed = raw.trim_start();

    // Specific validation with helpful messages
    let rest = trimmed
        .strip_prefix(FRONT_MATTER_DELIMITER)
        .ok_or_else(|| anyhow!(
            "Document missing starting front matter delimiter.\n\
             Documents must start with --- on the first line."
        ))?;

    // ... more validation
}
```

### Bail Pattern

**Usage**: Early exit on logical errors

```rust
if output_path.exists() {
    bail!(
        "A document already exists at {}; choose a different title or override path",
        output_path.display()
    );
}
```

## Data Consistency

### Single-File Atomicity

**Guarantee**: `fs::write()` is atomic at the filesystem level

```rust
// Either fully succeeds or leaves file unchanged
fs::write(&path, content)?;
```

**Caveat**: Depends on underlying filesystem (most modern filesystems guarantee atomic writes).

### Multi-File Operations

**Risk**: No transactional guarantees across files

```rust
// If this fails, inconsistent state:
fs::write(&source_path, source_content)?;  // Succeeds
fs::write(&target_path, target_content)?;  // FAILS

// Result: source updated, target not updated
```

**Current Mitigation**: None (documented limitation)

**Potential Solution**: Transaction log or temporary files with atomic rename

### Manifest Consistency

**Approach**: Manifest is derived data, can be regenerated

```rust
// Always rebuild from source documents
fn render_manifest(claude_root: &Path, entries: &[ManifestEntry]) -> Result<String> {
    // Generate table from current document state
}
```

**Benefit**: Manifest corruption doesn't affect source documents

## Concurrency Considerations

### No Locking Mechanism

**Current**: No file locking or mutex protection

**Risk**: Race conditions if multiple processes write simultaneously

```bash
# Two processes running concurrently:
kb-claude link doc-a doc-b &  # Process 1
kb-claude link doc-a doc-c &  # Process 2

# Possible result: One relation lost due to race
```

**Mitigation**: Users must avoid concurrent writes

### Read-Multiple-Write-Any Pattern

**Safe**: Multiple concurrent readers
**Unsafe**: Multiple concurrent writers

**Example**: Safe usage pattern

```bash
# Multiple reads: OK
kb-claude search "foo" &
kb-claude validate &

# Single write: OK
kb-claude new "My Doc"

# Multiple writes: UNSUPPORTED
kb-claude link doc-a doc-b &
kb-claude link doc-c doc-d &  # May conflict
```

## Performance Characteristics

### I/O Complexity

| Operation | Time Complexity | Space Complexity |
|-----------|----------------|------------------|
| Create document | O(1) | O(1) |
| Read document | O(1) | O(n) where n = doc size |
| Scan all documents | O(n) where n = #docs | O(1) with iterator |
| Search | O(n * m) where n=#docs, m=search terms | O(k) where k=matches |
| Manifest | O(n log n) for sorting | O(n) for collection |

### Optimization Strategies

**Lazy Iteration**:

```rust
// Streaming: O(1) memory
for entry in walk_kb_documents(&claude_root) {
    process(entry)?;
}
```

**In-Memory Collection** (when sorting needed):

```rust
// Collect: O(n) memory for sorting
let mut entries: Vec<_> = walk_kb_documents(&claude_root).collect()?;
entries.sort_by_key(|e| &e.document.front_matter.title);
```

## Design Philosophy

### Filesystem as Database

**Advantages**:
- Human-readable (Markdown + YAML)
- Version control friendly (text-based)
- Tool-agnostic (any text editor works)
- No database server required
- Simple backup (just copy `.claude/`)

**Disadvantages**:
- No transactions
- No querying language
- Manual schema validation
- Performance limits at scale

### Serialization Choice: YAML + Markdown

**Why YAML?**
- Human-readable and editable
- Supports nested structures (tags, relations)
- Well-supported in Rust (`serde_yaml`)
- Industry standard for config files

**Why Markdown?**
- Universal rendering support
- Rich ecosystem (editors, preview tools)
- Suitable for documentation
- Easy to convert to other formats

### Error Handling Philosophy

**Principle**: Fail with detailed context

```rust
fs::write(&path, content)
    .with_context(|| format!("Unable to write {}", path.display()))?;
```

**Benefits**:
- Clear error messages for users
- Sufficient context for debugging
- Stack traces preserved via `anyhow`
- Actionable error information

## Future Enhancements

### Potential Improvements

1. **Atomic Multi-File Operations**: Use temporary files + atomic rename
2. **File Locking**: Prevent concurrent write conflicts
3. **Indexing**: Accelerate searches on large KBs
4. **Compression**: Reduce storage for large document bodies
5. **Encryption**: Protect sensitive knowledge entries

### Implementation Considerations

**Before Adding Complexity**:
- Measure actual performance on real KBs
- Identify specific bottlenecks
- Evaluate if complexity is justified
- Consider migration strategy for existing KBs

## Conclusion

The kb-claude persistence layer embraces simplicity:
- **Filesystem as database**: No separate database server
- **Text-based storage**: Human-readable Markdown + YAML
- **Explicit serialization**: Clear conversion between structs and text
- **Contextual errors**: Detailed error messages with `anyhow`
- **Lazy processing**: Iterator-based I/O for efficiency
- **Trade-offs**: Accepts limited transactional guarantees for simplicity

This design prioritizes correctness, usability, and maintainability over performance optimizations that premature complexity would introduce.
