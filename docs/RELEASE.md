# Release Guide

This repository ships the `claude-kb-cli` crate, which installs the `kb-claude` binary. Follow the steps below whenever preparing a new release.

## 1. Preflight
- Ensure you have the latest Rust stable toolchain installed (`rustup update`).
- Authenticate with crates.io (one time): `cargo login <api-token>`.
- Confirm a clean worktree: `git status` should show no pending changes.

## 2. Version Bump
1. Update `Cargo.toml`:
   - Increment the `[package] version`.
   - If necessary, adjust dependency versions and run `cargo update`.
2. Regenerate `Cargo.lock` via `cargo check`.
3. Update release notes (e.g., append to `.claude` knowledge base or a CHANGELOG entry).

## 3. Validation
- `cargo fmt`
- `cargo clippy -- -D warnings`
- `cargo test`
- Smoke-test the CLI locally:
  ```bash
  cargo run -- init --dry-run
  cargo run -- new "Release Verification Note" -t metadata
  cargo run -- manifest
  cargo run -- validate --strict
  ```

## 4. Dry Run Publication
- `cargo publish --dry-run` to ensure the crate package builds.
- Inspect the generated `.crate` metadata in `target/package/`.

## 5. Publish
- Tag the release: `git tag vX.Y.Z && git push origin vX.Y.Z`.
- Publish for real: `cargo publish`.
- Verify on crates.io: `https://crates.io/crates/claude-kb-cli`.

## 6. Post-Release
- Push docs/notes updates to `master`.
- Announce availability (internal channel, README badge, etc.).
- Create a `.claude` entry summarizing the release for historical traceability.
