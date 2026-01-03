---
title: Data Model API
path: src/model.rs
type: file
depth: 1
description: Knowledge base document data structures and serialization
exports: [FRONT_MATTER_DELIMITER, OntologicalRelation, DocumentFrontMatter, Document, slugify, iso8601]
seams: [D]
---

# Data Model API

## File: `src/model.rs`

### Purpose
Defines the core data structures for knowledge base documents, including front matter metadata, full document representation, and serialization/deserialization logic.

## Public Constants

```rust
pub const FRONT_MATTER_DELIMITER: &str = "---";
```

## Public Structs

### `OntologicalRelation`
Represents a cross-reference or relationship to another document.

**Fields** (all `pub`):
- `slug: String` - URL-friendly document identifier
- `uuid: String` - Unique document identifier (UUID)
- `doc_type: String` - Document type (metadata, qa, debug_history, etc.)

**Derived Traits**:
- `Clone`
- `Debug`
- `PartialEq`
- `Serialize` (serde)
- `Deserialize` (serde)

**Use Case**: Tracking related documents in front matter

---

### `DocumentFrontMatter`
Represents the YAML front matter metadata of a knowledge entry.

**Required Fields**:
- `title: String` - Human-readable document title
- `doc_type: String` - Type category (metadata, qa, debug_history, etc.)
- `slug: String` - URL-friendly identifier (auto-generated from title)
- `uuid: String` - Unique identifier (auto-generated UUID4)
- `created_at: DateTime<Utc>` - Creation timestamp
- `updated_at: DateTime<Utc>` - Last modification timestamp

**Optional Fields**:
- `tags: Vec<String>` - Category tags
- `relates_to: Vec<OntologicalRelation>` - Cross-references to other documents
- `depth: Option<u8>` - Depth/level indicator
- `path: Option<String>` - Related file path
- `description: Option<String>` - Brief description

**Methods**:
- `pub fn new(title: impl Into<String>, doc_type: impl Into<String>) -> Self`
  - Constructor that auto-generates slug, uuid, and timestamps

- `pub fn touch_updated(&mut self)`
  - Updates `updated_at` to current time

- `pub fn ensure_link_matches_slug(&mut self)`
  - Validates slug consistency

- `pub fn slug_from_title() -> String`
  - Generates slug from title field

- `pub fn is_link_consistent() -> bool`
  - Checks if slug matches expected value

**Derived Traits**:
- `Clone`
- `Debug`
- `Serialize` (serde)
- `Deserialize` (serde)

---

### `Document`
Represents a complete knowledge base document (front matter + body content).

**Fields** (all `pub`):
- `front_matter: DocumentFrontMatter` - YAML metadata
- `body: String` - Markdown content body

**Methods**:
- `pub fn new(front_matter: DocumentFrontMatter, body: impl Into<String>) -> Self`
  - Constructor from components

- `pub fn parse(raw: &str) -> Result<Self>`
  - Parses Markdown string with YAML front matter into Document
  - **Format**: `---\nYAML\n---\nmarkdown body`
  - **Error**: Fails if front matter missing or invalid YAML

- `pub fn to_markdown(&self) -> Result<String>`
  - Serializes Document back to Markdown format
  - **Format**: `---\nYAML\n---\nbody content`

**Derived Traits**:
- `Clone`
- `Debug`

## Public Functions

### `slugify`
```rust
pub fn slugify(input: &str) -> String
```
Converts a string into a URL-friendly slug.

**Transformations**:
- Lowercase conversion
- Replace spaces with hyphens
- Remove special characters
- Collapse multiple hyphens

**Example**:
```
"My Document Title!" → "my-document-title"
```

## Public Modules

### `iso8601`
Custom serialization/deserialization for `DateTime<Utc>` using ISO8601 format.

**Functions**:
- `pub fn serialize<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>`
  - Serializes DateTime to ISO8601 string

- `pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>`
  - Deserializes ISO8601 string to DateTime

**Format**: `YYYY-MM-DDTHH:MM:SSZ` (e.g., `2025-01-03T12:00:00Z`)

## Serialization Format

### Document Front Matter (YAML)
```yaml
---
title: "My Document"
doc_type: "metadata"
slug: "my-document"
uuid: "550e8400-e29b-41d4-a716-446655440000"
created_at: "2025-01-03T12:00:00Z"
updated_at: "2025-01-03T12:00:00Z"
tags: ["rust", "cli"]
depth: 1
description: "A sample document"
relates_to:
  - slug: "related-doc"
    uuid: "660e8400-e29b-41d4-a716-446655440000"
    doc_type: "qa"
---
```

### Complete Document (Markdown)
```markdown
---
title: "My Document"
doc_type: "metadata"
slug: "my-document"
uuid: "550e8400-e29b-41d4-a716-446655440000"
created_at: "2025-01-03T12:00:00Z"
updated_at: "2025-01-03T12:00:00Z"
tags: ["rust", "cli"]
---

# Document Body

This is the markdown content of the document.
```

## Design Patterns

1. **Builder Pattern**: `DocumentFrontMatter::new()` with sensible defaults
2. **Self-Validating**: Methods to ensure consistency (slug matching, timestamps)
3. **Serialization First**: All structs support serde for easy persistence
4. **Immutable Metadata**: Front matter is treated as record of truth

## Use Cases

- **Document Creation**: Generate new knowledge entries with auto-generated metadata
- **Document Parsing**: Read existing `.md` files into structured data
- **Document Serialization**: Write structured data back to markdown
- **Validation**: Ensure front matter consistency
- **Cross-Referencing**: Track relationships between documents

## Error Handling

All operations return `anyhow::Result`:
- Parse errors for invalid YAML
- Validation errors for missing fields
- Serialization errors for invalid data

## Dependencies

- `serde` - Serialization framework
- `serde_yaml` - YAML format support
- `chrono` - DateTime handling
- `uuid` - UUID generation
- `anyhow` - Error handling
