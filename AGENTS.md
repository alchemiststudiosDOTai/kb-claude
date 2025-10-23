# Repository Guidelines

## Project Structure & Module Organization
The Rust crate lives in `src/`, with `main.rs` orchestrating Clap subcommands and helper modules under `src/cli/`, `src/fs/`, `src/model/`, and `src/ops/`. Integration-style smoke tests belong in `tests/` (e.g., `tests/cli_init.rs`). Runtime assets, including sample `.claude/` fixtures, should sit inside `fixtures/` to keep the workspace tidy. Documentation such as `PRD.md` and `PLAN.md` stays at the repository root.

## Build, Test, and Development Commands
- `cargo fmt`: format the entire codebase; run before every commit.
- `cargo clippy -- -D warnings`: lint using Rust 2021 idioms and fail on warnings.
- `cargo build --release`: produce an optimized `kb-claude` binary for distribution.
- `cargo run -- <subcommand>`: execute the CLI locally, e.g., `cargo run -- init`.
- `cargo test`: execute smoke and unit tests; add `-- --nocapture` to debug output.

## Coding Style & Naming Conventions
Embrace idiomatic Rust 2021 style enforced by `rustfmt`. Use 4-space indentation, snake_case for files, modules, and functions, and CamelCase for type names. Keep command handlers short by delegating logic to modules. YAML keys and Markdown filenames must mirror the schema in `PRD.md`. Prefer `anyhow::Result` for error propagation and `tracing` instrumentation only when necessary.

## Testing Guidelines
Focus on post-MVP smoke tests once manual verification passes. Use `assert_cmd` and `tempfile` to spin up ephemeral `.claude/` trees, naming tests after the command under exercise (e.g., `test_init_creates_layout`). Aim to cover happy paths and critical validation errors; branch coverage is nice-to-have, not mandatory.

## Commit & Pull Request Guidelines
Favor conventional commits (`feat:`, `fix:`, `refactor:`) followed by a succinct description of the change. Group related edits per commit and keep diffs focused. Pull requests should summarize intent, list the commands executed (`cargo fmt`, `cargo clippy`, `cargo test`), and reference associated issues or tasks. Include before/after snippets or CLI output when behavior changes, and call out any follow-up work.

## Agent Workflow Notes
Agents should sync with `PLAN.md` before making changes, keep dependencies minimal, and document manual test steps in PR descriptions. Avoid introducing new tools unless justified, and flag any schema adjustments back in the PRD.
