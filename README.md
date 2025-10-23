# kb-claude

`kb-claude` is a Rust CLI that manages Markdown-based knowledge bases organized under a `.claude/` hierarchy. Each command aligns with the MVP scope captured in `PRD.md` and the Implementation Plan in `PLAN.md`. The crate published to crates.io is named `claude-kb-cli` and installs the `kb-claude` binary.

## Getting Started

```bash
cargo install claude-kb-cli # or cargo build --release within this repo
kb-claude init
kb-claude new "auth module broken after drizzle kit upgrade" -t debug_history
```

Generated entries land inside `.claude/<type>/<slug>.md` with YAML front matter, UUIDs, and timestamps already populated. Interactive prompts collect tags, relations, and body copy when flags are omitted.

## Core Commands

- `cargo run -- init [--dry-run] [--directory PATH]` – create or preview the `.claude/` layout.
- `cargo run -- new <TITLE> [-t TYPE] [--tag TAG] [--relates-to LINK]` – add a new entry; prompts fill missing metadata.
- `cargo run -- link <SOURCE> <TARGET> [--force]` – insert bidirectional `relates_to` links between two slugs.
- `cargo run -- search <TERM...> [--tag TAG]` – keyword and tag search across titles, metadata, and bodies.
- `cargo run -- validate [--strict] [--directory PATH]` – verify schema compliance and report warnings/errors.
- `cargo run -- manifest [--output PATH] [--directory PATH]` – regenerate `.claude/manifest.md` with a Markdown table snapshot.

## Development Workflow

- Format: `cargo fmt`
- Lint: `cargo clippy -- -D warnings`
- Test: `cargo test`

The integration test at `tests/smoke.rs` exercises the full CLI flow (init → new → link → manifest → validate → search) using temporary directories. Manual smoke checks for new features should follow the same pattern before adding focused tests.

## Notes

- Stick to the dependency set already in `Cargo.toml`; add new crates only when the MVP requires them.
- If you tweak the schema, update both `PRD.md` and `model.rs` so validation and manifest generation stay in sync.
- Keep the codebase formatted and lint-clean between phases to mirror the workflow described in `PLAN.md`.
