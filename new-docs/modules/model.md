---
title: Model Module
path: src/model.rs
type: file
depth: 1
description: Core data structures for KB entries with YAML front matter
exports: [Document, DocumentFrontMatter, OntologicalRelation, slugify, iso8601]
seams: [M]
---

## Where
`src/model.rs`

## What
Defines the core data structures and serialization logic for knowledge base documents. Each document is a Markdown file with structured YAML front matter containing metadata.

## How

### Key Data Structures

**`OntologicalRelation`**
- Represents relationships between documents
- Single field: `relates_to: String`
- Enables linking concepts in the knowledge graph

**`DocumentFrontMatter`**
- Complete metadata for a document
- Fields:
  - `title`: Human-readable title
  - `link`: URL-friendly slug for persistent linking
  - `doc_type`: Category (note, article, cheatsheet, etc.)
  - `ontological_relations`: List of related concepts
  - `tags`: Keywords for categorization
  - `created_at`, `updated_at`: ISO8601 timestamps
  - `uuid`: Globally unique identifier
- Methods:
  - `new()`: Constructor with auto-generated slug, UUID, timestamps
  - `touch_updated()`: Update timestamp
  - `slug_from_title()`: Generate consistent slug
  - `is_link_consistent()`: Validate slug consistency

**`Document`**
- Combines `DocumentFrontMatter` with `body: String`
- `parse(raw: &str)`: Deserialize Markdown with YAML front matter
  - Expects `---` delimiters around YAML block
  - Uses `serde_yaml` for front matter parsing
  - Separates metadata from body content
- `to_markdown()`: Serialize back to Markdown format
  - Reconstructs `---` delimited structure
  - Serializes front matter via `serde_yaml`

### Helper Functions

**`slugify(input: &str) -> String`**
- Converts strings to URL-friendly slugs
- Lowercase, alphanumeric, hyphen-separated
- Handles punctuation and spacing

**`iso8601` module**
- Custom `serde` serialization for `chrono::DateTime<Utc>`
- `serialize()`: RFC3339 format with 'Z' suffix
- `deserialize()`: Parse ISO8601 strings
- Ensures consistent datetime representation

## Why
**Structured Metadata**: Separating metadata from content enables querying, sorting, and linking without parsing Markdown bodies.

**Persistent Linking**: The `link` field provides stable, human-readable identifiers that don't change even if titles are modified.

**Global Uniqueness**: UUIDs ensure documents can be synchronized across distributed knowledge bases without collisions.

**Timestamps**: Essential for versioning, sorting by recency, and detecting conflicts during sync.

**Serialization Standard**: Using YAML front matter follows the pattern of static site generators (Jekyll, Hugo) and makes documents readable by both humans and tools.

**Ontological Relations**: Enables building a knowledge graph where concepts are explicitly linked, supporting semantic navigation and discovery.
