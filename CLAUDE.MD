
###Instruction###
 Your task is to ensure all code you write or review strictly follows these standards for **modern, clean, and idiomatic Rust**.

You MUST adhere to the following structured principles, or you will be penalized in code review. Prioritize **clarity, correctness, and composability** in all aspects of code.

###Core Coding Principles###

* Clarity over cleverness — code must be understandable at a glance.
* Explicit > implicit — always surface lifetimes, ownership, and side effects.
* Functions must support local reasoning — no hidden context dependencies.
* Fail fast, fail loud — reject silent failures or implicit conversions.
* Trust the compiler over runtime guards.
* Favor small, focused abstractions.
* Avoid magic macros or codegen unless there's a strong and clear justification.

###Code Style & Structure###

* Only use Rust 2021 idioms — no deprecated constructs.
* Minimize nesting — use early returns and helper functions.
* Prefer `impl` blocks for domain behavior.
* Keep visibility tight: `pub(crate)` or `pub(super)` over `pub`.
* Break fluent chains >4 calls into named helpers.
* Enforce naming conventions:

  * `snake_case` — functions and variables
  * `PascalCase` — types and enums
  * `SCREAMING_SNAKE_CASE` — constants
  * Module files must use singular nouns

###Typing Philosophy###

* Encode intent through types.

  * Use newtypes for domain distinction.
  * Use `Option` for absence, `Result` for failure.
* Avoid premature generics — start concrete.
* Expose lifetimes only when necessary.
* Use trait bounds on functions, not types.
* Prefer enums over flags or booleans.

###Error Handling###

* Libraries: use `thiserror` and typed `Result`.
* Applications: use `anyhow::Result`.
* No `unwrap` or `expect` outside tests or unreachable branches.
* Include actionable error context (`context("loading user config")`).
* Convert errors once — handle at domain boundaries.

###API Design###

* Public APIs must be minimal, stable, and well-encapsulated.
* Prefer traits to inheritance patterns.
* Do not expose internal types like `Arc<Mutex<T>>`.
* Use builder patterns for complex struct creation.
* Favor zero-cost abstractions over runtime validation.

###Concurrency & Async###

* Apply `Send + Sync` bounds only when required.
* Avoid `async` unless you need it. Prefer sync by default.
* Use `spawn_blocking` to isolate blocking operations.
* Ensure all async code supports cancellation and shutdown.
* Avoid unbounded channels and orphaned background tasks.
* Structure logs with data, not formatted strings.

###Testing Guidelines###

* Co-locate unit tests (`mod tests`) with code.
* Integration tests must only test the public API.
* All tests must be fast, deterministic, and isolated (no I/O).
* Use `proptest` for property-based tests.
* Avoid overmocking — test real logic where possible.
* Only use `#[should_panic]` to verify invariant enforcement.

###Performance & Memory###

* Write the correct version first, optimize later.
* Avoid unnecessary `clone()`s — prefer references.
* Use iterators and slices over allocations.
* Profile before using `unsafe`.
* Avoid `Rc`/`RefCell` in core logic.
* Do not serialize/deserialize between layers unless required.

###Safety & Security###

* All `unsafe` code must have clear justification and tests for assumptions.
* Do not re-export unsafe abstractions.
* No FFI without isolation and test coverage.
* Avoid panics in `Drop` implementations.
* Depend only on audited, single-purpose crates.

###Documentation & Naming###

* Every public item must include a brief doc comment stating *what* and *why*.
* Ensure all examples compile via `cargo test --doc`.
* Favor doc tests over regular examples.
* Avoid generic names like `data`, `info`, `thing`.
* Use domain-specific terminology — good names reduce comment need.

###Project & Module Organization###

* Modules should reflect domain concepts (e.g., `auth.rs`, not `utils.rs`).
* Keep functions under 50 LOC — use private helpers to decompose.
* Avoid globals — inject dependencies using traits or struct fields.
* Maintain consistent import ordering: std → external crates → internal modules.

###Final Rules###

* Delete dead code — don’t comment it out.
* Every new type or function must justify its existence.
* Small, composable PRs only.
* Code must read like it was written once, refactored thrice.
* You MUST lint, format and runt test after every change.
* When in doubt, write less code.

