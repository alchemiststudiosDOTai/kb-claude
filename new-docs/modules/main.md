---
title: Binary Entry Point
path: src/main.rs
type: file
depth: 1
description: Application binary entry point
exports: []
seams: [L]
---

## Where
`src/main.rs`

## What
The binary entry point for the `claude_kb_cli` executable. Minimal implementation that delegates to the library layer.

## How

### Implementation
```rust
fn main() -> Result<()> {
    claude_kb_cli::cli::run()
}
```

**Key Details**:
- Returns `anyhow::Result<()>` for error handling
- Single responsibility: invoke CLI runner
- No business logic in the binary

## Why
**Separation of Concerns**: Keeping `main.rs` minimal ensures that the CLI logic can be:
- Tested as a library without binary overhead
- Reused in different contexts (e.g., as a library by other tools)
- Mocked/integrated more easily in tests

**Error Propagation**: Using `anyhow::Result` allows the `anyhow` crate to:
- Display user-friendly error messages
- Provide error context chains
- Handle different error types uniformly

**Binary vs Library Split**: This pattern enables the project to be:
- Used as a CLI tool (`cargo install`)
- Used as a library (`dependencies = ["kb-claude"]`)
- Tested without invoking the binary

**Minimal Surface Area**: By having no logic in `main.rs`, we reduce the attack surface and make the entry point predictable and easy to audit.
