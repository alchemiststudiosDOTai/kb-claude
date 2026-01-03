---
title: Tests Directory
path: tests/
type: directory
depth: 1
description: Integration test suite using assert_cmd and assert_fs for CLI verification
seams: [S]
---

# Tests Directory (`tests/`)

## Purpose
Contains integration tests that verify the `kb-claude` CLI tool's behavior by executing the binary and asserting filesystem state. Tests follow the project guideline: "Favor integration coverage that drives the binary via `assert_cmd` and temporary directories from `assert_fs`."

## Organization

### Test Files

#### `smoke.rs` - End-to-End Smoke Tests
Verifies the core happy path workflow functions correctly.

**Key Test:**
- `end_to_end_flow()` - Tests complete CLI lifecycle:
  1. Initialize knowledge base with `kb-claude init`
  2. Create entries with `kb-claude new`
  3. Link documents with `kb-claude link`
  4. Generate manifest with `kb-claude manifest`
  5. Validate entries with `kb-claude validate`
  6. Search knowledge base with `kb-claude search`

**Approach:**
- Uses `assert_fs::TempDir` for isolated test environments
- Uses `assert_cmd::Command` to run binary
- Asserts both exit codes and filesystem state
- Tests actual user-facing CLI behavior, not internal Rust APIs

#### `command_matrix.rs` - Command Interaction Tests
Tests interactions between multiple CLI commands and edge cases.

**Likely Contents:**
- Matrix of command sequences and expected outcomes
- Cross-command validation (e.g., manifest reflects new entries)
- Error handling across command boundaries
- State verification after command chains

## Testing Strategy

### Integration-First Approach
Per `CLAUDE.md` guidelines:
- **Primary**: Integration tests driving the actual binary
- **Secondary**: Unit tests within modules (if needed)
- **Rationale**: CLI tools are primarily defined by external behavior

### Test Naming Convention
- Files named after behavior they verify
- `smoke.rs` for critical path validation
- `command_matrix.rs` for interaction testing
- Future tests would follow pattern: `feature_name.rs`

### Tools Used
- **`assert_cmd`** - Run CLI commands and assert exit codes, stdout, stderr
- **`assert_fs`** - Create temporary directories and assert file state
- **Standard Rust testing** - `cargo test` framework

## Test Execution

### Running Tests
```bash
# Run all tests
cargo test

# Run tests with output (for debugging)
cargo test -- --nocapture

# Run specific test file
cargo test --test smoke

# Run specific test
cargo test end_to_end_flow
```

### Test Isolation
- Each test uses `TempDir` for clean filesystem state
- No shared state between tests
- Tests can run in parallel (no reliance on execution order)

## Architectural Significance

### Binary Testing vs Unit Testing
Tests verify the **binary** (`target/debug/kb-claude`), not the library:
- Ensures CLI argument parsing works end-to-end
- Validates filesystem operations from user perspective
- Catches issues in `main.rs` → `lib.rs` integration
- Tests actual CLI behavior, not Rust APIs

### Regression Test Addition
Per `CLAUDE.md`:
> "Add regression cases when fixing bugs"

Workflow:
1. Bug discovered in production use
2. Add failing test case to `tests/`
3. Fix bug in `src/`
4. Verify test passes
5. Bug cannot reoccur without test failure

### Output Assertion
For tests verifying CLI output:
```bash
cargo test -- --nocapture
```
This allows inspection of stdout/stderr during test runs, useful for debugging formatting or search result display.

## Relationships

### Code Coverage
- **Tests**: `src/` directory behavior
- **Commands**: All subcommands in `src/cli/`
- **Filesystem**: `.claude/` directory creation and modification
- **Models**: YAML front matter parsing and generation

### Documentation Mapping
- Test coverage reflects `README.md` command documentation
- Tests verify documented commands work as advertised
- Discrepancies indicate documentation or implementation bugs

### Development Workflow
- Tests run before commits (CI gate)
- Tests run before releases (verification)
- Tests added for bug fixes (regression prevention)

## Extension Points

### Adding New Tests
1. Create new file matching behavior: `tests/feature_name.rs`
2. Use `assert_cmd::Command::cargo_bin("kb-claude")`
3. Use `assert_fs::TempDir::new()` for isolation
4. Assert both exit codes and filesystem state
5. Run with `cargo test -- --nocapture` during development

### Test Categories
- **Smoke tests** (`smoke.rs`): Critical path validation
- **Matrix tests** (`command_matrix.rs`): Command interactions
- **Regression tests**: Added when bugs are fixed
- **Feature tests**: Verify new functionality works

## Naming Conventions

### Test Functions
- `snake_case` names (Rust standard)
- Descriptive: `end_to_end_flow`, `test_init_creates_directory`
- Test case suffixes: `with_valid_input`, `with_missing_flags`

### Test Modules
- Each file is a test module
- `mod tests;` convention for inline tests (not used here - separate files preferred)

### Assertion Messages
- Descriptive failure messages for debugging
- Include expected vs actual values
- Context about test scenario

## Best Practices (from `CLAUDE.md`)

1. **Integration Coverage**: Drive the binary, don't test internal APIs
2. **Temporary Directories**: Always use `assert_fs::TempDir`
3. **Descriptive Names**: Name test files after behavior verified
4. **Regression Tests**: Add cases when fixing bugs
5. **Output Capturing**: Use `--nocapture` when debugging output assertions
6. **Clean State**: Each test should clean up after itself (TempDir handles this)
