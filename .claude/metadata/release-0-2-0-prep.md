---
title: Release 0.2.0 Prep
link: release-0-2-0-prep
type: metadata
ontological_relations:
- relates_to: dogfooding-knowledge-base
tags:
- release
- roadmap
created_at: 2025-10-23T20:49:00Z
updated_at: 2025-10-23T20:49:00Z
uuid: 1f2345a0-3af5-4f2a-b3bb-19d4a36fcf81
---
Target version bumped to 0.2.0 to skirt crates.io conflict with 0.1.x lineage.
Validations: cargo fmt, cargo clippy -- -D warnings, cargo test.
Queued for `cargo publish --dry-run` followed by live publish and git tag.
