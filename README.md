# Claude KB CLI

Lightweight command-line helper for managing structured `.claude` knowledge base entries (metadata, debug logs, QA, code index, patterns, cheatsheets) in a consistent JSON format.

## Prerequisites
- Stable Rust toolchain (`rustup`).
- `git` only if you want diff reporting.

## Install
```bash
cargo install --path .
```
You can also run directly with `cargo run -- <command>`.

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

## Development
- Format: `cargo fmt`
- Lint: `cargo clippy --all-targets -- -D warnings`
- Tests: `cargo test`
