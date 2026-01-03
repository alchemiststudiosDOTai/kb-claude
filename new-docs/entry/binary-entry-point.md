---
title: Binary Entry Point
path: src/main.rs
type: file
depth: 1
description: Binary entry point for kb-claude CLI
exports: [main]
seams: [E]
---

# Binary Entry Point

## File: `src/main.rs`

### Purpose
This is the binary entry point for the `kb-claude` executable. It serves as the bootstrap that launches the CLI application.

### Main Function
The `main()` function delegates all logic to the library layer:

```rust
fn main() {
    claude_kb_cli::cli::run();
}
```

### Architecture
- **Binary Name**: `kb-claude`
- **Entry Point**: Calls `claude_kb_cli::cli::run()`
- **Responsibility**: Minimal - simply forwards to the library's CLI module

### Design Pattern
This follows the "binary-as-thin-wrapper" pattern, where the actual CLI logic lives in the library crate (`src/lib.rs` → `src/cli/mod.rs`). This allows:
1. Easier integration testing of CLI logic
2. Potential reuse as a library
3. Clear separation between binary and library concerns

### Dependencies
- `claude_kb_cli` library crate
- No direct dependencies on CLI parsing frameworks
