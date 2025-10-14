# Claude KB CLI

![Claude KB CLI](https://maas-log-prod.cn-wlcb.ufileos.com/anthropic/230cb711-68a5-4e5d-b65f-24d823b2aebf/86b1819b06ceced1b670652b9e47cd09.png?UCloudPublicKey=TOKEN_e15ba47a-d098-4fbd-9afc-a0dcf0e4e621&Expires=1760470389&Signature=MOHIr0noh+lOYBVcXQtbwl3Vdso=)

Lightweight command-line helper for managing structured `.claude` knowledge base entries (metadata, debug logs, QA, code index, patterns, cheatsheets) in a consistent JSON format.

## Prerequisites
- Stable Rust toolchain (`rustup`).
- `git` only if you want diff reporting.

## Install
```bash
cargo install claude-kb-cli
```
You can also run directly with `cargo run -- <command>` or install from source with `cargo install --path .`.

## `.claude` layout
The tool keeps everything under `.claude/` and will create the folders on demand:
```
.claude/
  metadata/      component summaries
  debug_history/ debugging timelines
  qa/            question & answer entries
  code_index/    file references
  patterns/      reusable fixes or snippets
  cheatsheets/   quick reference sections
  manifest.json  last sync snapshot
```

## Everyday workflow
```bash
# create a typed entry
claude-kb add pattern --component ui.auth --summary "Retry login" \
  --error "Explain retry UX" --solution "Link to pattern doc"

# modify an existing entry (errors when the item is missing)
claude-kb update pattern --component ui.auth \
  --error "Retry login" --solution "Updated copy"

# list or validate your KB
claude-kb list --type pattern
claude-kb validate

# sync manifest and inspect git drift
claude-kb sync --verbose
claude-kb diff --since HEAD~3

# remove stale data
claude-kb delete pattern --component ui.auth
```
Append `--json` to most commands for machine-readable output.

## Source Directory Structure

```
src/
├── agent/              # Agent-related functionality
│   ├── mod.rs
│   └── protocol.rs
├── commands/           # CLI command implementations
│   ├── mod.rs
│   ├── add.rs
│   ├── delete.rs
│   ├── diff.rs
│   ├── list.rs
│   ├── sync.rs
│   ├── update.rs
│   └── validate.rs
├── io/                 # File I/O operations
│   ├── mod.rs
│   └── file_ops.rs
├── manifest/           # Manifest management
│   ├── mod.rs
│   └── sync.rs
├── models/             # Data models and structures
│   ├── mod.rs
│   ├── cheatsheet.rs
│   ├── code_index.rs
│   ├── debug.rs
│   ├── delta.rs
│   ├── entry_type.rs
│   ├── metadata.rs
│   ├── pattern.rs
│   └── qa.rs
├── schema/             # Schema validation
│   ├── mod.rs
│   └── validator.rs
├── lib.rs              # Library entry point
└── main.rs             # CLI application entry point
```

## Development
- Format: `cargo fmt`
- Lint: `cargo clippy --all-targets -- -D warnings`
- Tests: `cargo test`
