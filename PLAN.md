# kb-claude MVP Implementation Plan

## Guiding Principles
- Deliver the required CLI behavior with the smallest, clearest Rust 2021 surface area.
- Centralize reusable I/O helpers so each command stays thin.
- Keep dependencies limited to the PRD list unless a blocker arises.
- Run `cargo fmt` and `cargo clippy -- -D warnings` after each feature slice.
- Defer automated tests until the happy-path CLI flow is working; add smoke tests afterwards.

## Scope & Constraints
- MVP must ship the six documented subcommands (`init`, `new`, `search`, `link`, `validate`, `manifest`).
- `search` can remain simple keyword/tag matching via in-memory scan; no index.
- `link` writes reciprocal YAML relations only; no graph inference.
- `manifest` rebuilds the Markdown table in `.claude/manifest.md`.
- No TDD: validate behavior manually first, then capture high-value tests.
- Out of scope: networking, LLM features, Tantivy/SQLite integration.

## Work Plan
1. **Baseline Project Setup**
   - Initialize a Rust binary crate (`cargo new kb-claude`), enable Rust 2021 edition.
   - Add dependencies: `clap` (derive), `serde`, `serde_yaml`, `serde_json` (if needed), `uuid`, `chrono`, `walkdir`, `glob`, `anyhow` (for ergonomic errors).
   - Configure `cargo fmt` and `cargo clippy` in CI hook or local script (manual invocation for MVP).

2. **Shared Types & Utilities**
   - Create a `model` module for the document schema mirroring the PRD YAML.
   - Implement serialization/deserialization helpers, slug generation, timestamp + UUID utilities.
   - Add file-system helpers for `.claude` root detection, directory creation, and manifest path resolution.

3. **CLI Surface (`main.rs` + `cli` module)**
   - Define Clap-powered enum for subcommands with arguments/prompts configuration.
   - Implement command dispatcher calling feature-specific handlers.

4. **`init` Command**
   - Ensure `.claude/` exists with required subdirectories; create missing ones.
   - Gracefully handle already-initialized repos (idempotent output).

5. **`new` Command**
   - Prompt for metadata using lightweight stdin helpers (no external crates).
   - Assemble front matter with timestamps + UUID, enforce slug == link.
   - Write Markdown file under `.claude/<type>/<link>.md`; ensure directory names validated.

6. **`validate` Command**
   - Traverse `.claude/` with `walkdir`.
   - Parse YAML front matter, confirm required fields, type-directory alignment, slug-match, timestamp format.
   - Aggregate and report validation issues.

7. **`manifest` Command**
   - Reuse traversal to pull document metadata.
   - Render Markdown table (matching PRD example) and overwrite `.claude/manifest.md`.

8. **`search` Command**
   - Parse documents once (reuse utilities).
  - Implement simple case-insensitive substring search across title, tags, body.
   - Format ranked list (title, path, tags, type) without fancy scoring.

9. **`link` Command**
   - Validate both target files exist.
   - Update each document’s `ontological_relations` with a `relates_to` entry referencing the other’s `link`.
   - Avoid duplicates; maintain front matter formatting.

10. **Testing & Validation**
   - Manually exercise each command in a sample `.claude` tree.
   - Capture smoke tests: one integration test per command using `assert_cmd` + temp directories.
   - Run `cargo fmt`, `cargo clippy`, and `cargo test` before handoff.

11. **Documentation & Wrap-Up**
   - Provide concise README usage section aligned with PRD.
   - Summarize manual verification steps and outstanding nice-to-haves.

## Post-MVP Follow-Ups
- Enhance search ranking (e.g., weighting tags).
- Structured prompt-based metadata collection (e.g., guided templates).
- Manifest diff mode (git-aware) and optional JSON outputs.
