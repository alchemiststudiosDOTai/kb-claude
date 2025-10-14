Perfect — you want a **Rust code philosophy doc**, not setup or config details.
Here’s a concise, opinionated **Rust Code Guidelines** outline — the kind senior teams use to define *how to write good Rust*, not just *how to configure it*.

---

# AGENTS_RUST.md

This document defines standards for writing **modern, clean, and idiomatic Rust** across all projects.

---

## Core Principles

1. **Clarity over cleverness** — favor code others can reason about at a glance.
2. **Explicit > implicit** — make ownership, lifetimes, and side effects visible.
3. **Local reasoning** — every function should be understandable without reading the whole module.
4. **Fail fast, fail loud** — avoid silent fallbacks and implicit coercions.
5. **Frictionless correctness** — lean on the compiler, not runtime checks.
6. **Small, composable abstractions** — one job per module or type.
7. **No magic macros or codegen** unless the benefit is substantial and obvious.

---

## Code Style & Structure

* Follow **Rust 2021 idioms**; no legacy constructs (`try!`, `extern crate`, etc.).
* **Minimize nesting:** use early returns and small helper functions.
* **Prefer `impl` blocks** over free functions for domain behavior.
* **Limit visibility:** `pub(crate)` or `pub(super)` before `pub`.
* **Avoid long chains:** refactor fluent APIs that exceed 3–4 calls.
* **Consistent naming:**

  * `snake_case` for functions/vars
  * `PascalCase` for types/enums
  * `SCREAMING_SNAKE_CASE` for constants
  * modules are singular nouns (`error.rs`, not `errors.rs`)

---

## Typing Philosophy

* **Model intent with types.**

  * Use newtypes (`struct UserId(String);`) for distinct domains.
  * Represent optionality and absence with `Option`, not magic values.
  * Represent fallibility with `Result`, not booleans.
* **Avoid generic bloat:** start concrete, abstract later if duplication demands it.
* **Minimize lifetimes exposure:** only make lifetimes explicit when needed.
* **Trait bounds:** place on functions, not structs, to reduce propagation.
* **Prefer enums over multiple booleans or mode flags.**

---

## Error Handling

* **Library code:** use typed errors (`thiserror`) and return `Result<T, E>`.
* **App/CLI code:** use `anyhow::Result` for ergonomic propagation.
* **No panics in production code.**

  * `unwrap`/`expect` only in tests, examples, or truly unreachable states.
* **Surface actionable errors.**

  * Avoid stringly errors; include context (`context("parsing config file")`).
* **Convert early, handle late.**

  * Don’t map errors layer by layer unless necessary; handle at boundaries.

---

## API Design

* Keep **public APIs minimal and stable**; internal freedom matters more.
* Prefer **traits over inheritance-style hierarchies**.
* **Encapsulate invariants** — don’t expose internal data directly.
* Avoid exposing implementation details like `Arc<Mutex<T>>` publicly.
* Use **builder patterns** for complex struct initialization; avoid optional explosion.
* **Zero-cost abstractions** preferred over runtime checks.

---

## Concurrency & Async

* Use **`Send + Sync`** bounds thoughtfully — don’t over-constrain generics.
* **Async boundaries**:

  * Use `async fn` in APIs only if truly needed; sync is simpler and faster.
  * Never mix blocking and async calls; use `spawn_blocking`.
* **Graceful shutdown:** all async systems must propagate cancellation.
* Avoid unbounded channels or background tasks with no owner.
* Log structured data, not formatted strings.

---

## Testing Philosophy

* **Unit tests co-located** with code (`mod tests`).
* **Integration tests** assert public API behavior only.
* Tests are **fast, isolated, and deterministic** — no external I/O.
* Prefer property tests (`proptest`) for invariants.
* **Don’t overmock:** prefer real logic in memory.
* Panic tests: only to validate invariants (`#[should_panic]`).

---

## Performance & Memory

* Write the simplest version first — measure before optimizing.
* Avoid unnecessary `clone()`; prefer borrowing.
* Use iterators and slices over allocation-heavy collections.
* Profile hot paths before introducing unsafe code.
* Avoid Rc/RefCell in core logic; prefer clear ownership models.
* **Zero-cost boundaries:** don’t serialize/deserialize unnecessarily between layers.

---

## Safety & Security

* `unsafe` requires a **commented justification** and a test covering safety assumptions.
* Don’t re-export unsafe abstractions.
* No dynamic eval, no FFI without a dedicated module and test coverage.
* Avoid panics in Drop implementations.
* Review dependencies: prefer audited, single-purpose crates.

---

## Documentation & Naming

* Each public item has a short doc comment explaining *what* and *why*.
* Examples compile (`cargo test --doc`).
* Prefer **doc tests** for APIs.
* Avoid vague names like `data`, `info`, `handle`. Use domain terms.
* Good naming > excessive commenting.

---

## Project & Module Design

* Each module should represent a **concept**, not a category.

  * ✅ `auth.rs` or `token.rs`
  * ❌ `utils.rs` or `helpers.rs`
* Keep functions < 50 lines; if longer, split into private helpers.
* Avoid global state; prefer dependency injection via traits or structs.
* Consistent import order: `std`, external crates, internal modules.

---

## Other Rules

1. Delete dead code — don’t comment it out.
2. Every new type or function must have a reason to exist.
3. Small, composable PRs.
4. Code should read like it was **written once, refactored thrice**.
5. When in doubt: write less code.

---

