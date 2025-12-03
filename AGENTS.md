# Repository Guidelines

Inner Development Loop
While working on our project, we will be going through the same steps over and over again:
• Make a change;
• Compile the application;
• Run tests;
• Run the application.

## Using This Project to Manage This Project
This project was designed through dogfooding—we install the `claude-kb-cli` crate, which provides the `kb-claude` binary used to manage our knowledge base. Developers should invoke `kb-claude` to capture debugging sessions, architecture decisions, and recurring insights. Use `kb-claude new`, `kb-claude search`, and the other subcommands to maintain our institutional memory about this project's development.

Knowledge base structure:
```
.claude/
├── metadata/          # component summaries
├── debug_history/     # debugging timelines
├── qa/               # Q&A and learning notes
├── code_index/       # file or module references
├── patterns/         # reusable fixes or design motifs
├── plans/            # project and release plans
├── other/            # scratch notes ignored by the CLI
├── cheatsheets/      # quick references or how-tos
├── memory_anchors/   # core concepts tracked by UUID
└── manifest.md       # automatically generated summary
```

Think of it as your project's institutional memory: capture debugging sessions, architecture decisions, and recurring insights as searchable, version-controlled knowledge.

## Project Structure & Module Organization
`src/main.rs` pipes the binary into `claude_kb_cli::cli::run`, while `src/lib.rs`, `src/model.rs`, and `src/fs.rs` host domain logic and filesystem helpers. Subcommands sit in `src/cli/` as one file per action (`init.rs`, `manifest.rs`, `validate.rs`, etc.), so mirror that pattern for new features. Integration tests live under `tests/`, docs under `docs/`, and build output in `target/`; keep scratch `.claude/` folders out of commits.

## Workflow Overview
Replace “prompt → copy → paste → hope it works” with a repeatable loop: 1) Define – state the problem, inputs/outputs, and success gates before requesting help. 2) Test – outline the minimal CLI transcripts or integration cases that must pass. 3) Build – combine AI drafts with hand-written Rust to keep modules focused and reusable. 4) Document – log intent, assumptions, and tool usage in comments or `.claude/` notes. 5) Review – run format, lint, and test commands, then confirm the solution still matches the original definition.

## Build, Test, and Development Commands
Use `cargo build` for regular work and `cargo build --release` when benchmarking. `cargo run -- <subcommand>` (e.g., `cargo run -- validate --strict`) exercises the CLI end-to-end. `cargo test` runs the integration suite, `cargo fmt` enforces formatting, `cargo clippy -- -D warnings` keeps lint debt at zero, and `cargo doc --open` reviews public API docs.

## Coding Style & Naming Conventions
Follow Rust defaults: four-space indent, snake_case functions, PascalCase types, SCREAMING_SNAKE_CASE constants. Align CLI flag names with the terminology in `README.md`, and ensure new `.claude/` types match the ontology already documented. Always format with `cargo fmt` before pushing.

## Testing Guidelines
Favor integration coverage that drives the binary via `assert_cmd` and temporary directories from `assert_fs`. Name new test files after the behavior they verify. Add regression cases when fixing bugs and run `cargo test -- --nocapture` if output assertions matter.

## Commit & Pull Request Guidelines
Commits follow lightweight Conventional Commit prefixes (`docs:`, `release:`, `chore:`); keep subjects imperative and scoped. For PRs, describe user impact, list any `.claude/` artifacts touched, attach relevant CLI transcripts or screenshots, and note the validation commands you ran so reviewers can reproduce quickly.
