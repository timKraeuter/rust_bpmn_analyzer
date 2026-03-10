# AGENTS.md

## Project Overview

Rust BPMN Analyzer -- a multi-crate Rust project for analyzing BPMN (Business Process Model and Notation) models. It performs model checking (safeness, option to complete, proper completion, no dead activities) with optional partial order reduction (POR).

**Crate layout** (no workspace -- each crate is independent with its own `Cargo.toml`):

| Crate | Path | Type | Description |
|-------|------|------|-------------|
| `rust_bpmn_analyzer` | `core/` | Library | Core logic: BPMN parsing, state space exploration, model checking |
| `rust_bpmn_analyzer_cli` | `cli/` | Binary | CLI frontend (depends on core) |
| `rust_bpmn_analyzer_wasm` | `wasm/` | cdylib | WASM bindings (depends on core) |
| `rust_bpmn_analyzer_webserver` | `webserver/` | Binary+lib | Axum web server (depends on core) |

**Rust edition**: 2024 (all crates).

## Build / Lint / Test Commands

All commands must be run from within the specific crate directory (e.g., `core/`, `cli/`).

```sh
cargo build                                      # debug build
cargo build --release                            # release build
cargo fmt                                        # auto-format
cargo fmt --check                                # check formatting (CI)
cargo clippy -- -Dwarnings                       # lint (CI)
cargo test -- --show-output                      # run all tests (CI)
cargo test <test_name> -- --show-output          # single test
cargo test <module>::tests -- --show-output      # tests in a module
```

**Single test example** (from `core/`):
```sh
cargo test try_execute_task -- --show-output
cargo test test_por_preserves_properties -- --show-output
```

### CI Pipeline (per-crate)

Each crate runs: `fmt --check` -> `clippy -- -Dwarnings` -> `build` -> `test -- --show-output`.
The `wasm` crate has no tests in CI (build + lint only).

## Project Structure (core)

```
core/src/
  lib.rs                     # Public API facade with re-exports
  bpmn/
    mod.rs                   # Module declarations
    collaboration.rs         # Collaboration (top-level BPMN container)
    flow_node.rs             # FlowNode execution semantics (largest file)
    process.rs               # Process struct
    reader.rs                # BPMN XML reader (quick-xml)
    reader_test.rs           # Reader unit tests
    semantics_test.rs        # Execution semantics unit tests
  model_checking/
    mod.rs                   # Module declarations
    properties.rs            # Properties and on-the-fly checking
    por/
      mod.rs                 # POR module re-exports
      ample_set.rs           # Ample set computation
      explorer.rs            # POR-enabled state space explorer
      independence.rs        # Transition independence relation
      result.rs              # POR result types
  states/
    mod.rs                   # Module declarations
    state_space.rs           # State, ProcessSnapshot, StateSpace
```

## Code Style Guidelines

### Formatting

Use `cargo fmt` (default rustfmt settings). No `rustfmt.toml` or `clippy.toml` exists.

### Imports

Order (no blank lines between groups):
1. `crate::` imports first
2. External crate imports
3. `std::` imports last

```rust
use crate::bpmn::flow_node::{EventType, FlowNode, FlowNodeType};
use crate::bpmn::process::Process;
use std::collections::{BTreeMap, HashMap, VecDeque};
```

- Prefer multi-item imports with `{}` over multiple single-item lines.
- Enum variants may be imported directly when used frequently (e.g., `use FlowNodeType::StartEvent;`).
- Use `pub use` for selective re-exports in `lib.rs` and `mod.rs`.

### Naming Conventions

| Item | Convention | Examples |
|------|-----------|----------|
| Structs/Enums | `PascalCase` | `FlowNode`, `ProcessSnapshot`, `ModelCheckingResult` |
| Enum variants | `PascalCase` | `StartEvent`, `ExclusiveGateway`, `Message` |
| Functions | `snake_case`, verb-first | `try_execute_task`, `create_start_state`, `explore_state_space` |
| Variables | `snake_case`, descriptive | `current_state_hash`, `unexplored_states`, `seen_state_hashes` |
| Constants | `SCREAMING_SNAKE_CASE` | `MAX_TOKEN`, `LOG_LEVEL`, `PATH` |
| DTO types | Suffix with `DTO` | `PropertyDTO`, `StateDTO`, `ProcessSnapshotDTO` |

Common abbreviations used consistently: `sf` (sequence flow), `mf` (message flow), `pg` (parallel gateway), `exg` (exclusive gateway), `evg` (event-based gateway).

### Type Annotations and Lifetimes

- Add explicit type annotations when the type is not obvious from context.
- The codebase is lifetime-heavy (zero-copy architecture). `'a` = model lifetime, `'b` = state reference lifetime.
- Use `&[T]` slice parameters instead of `&Vec<T>`.
- No trait-bounded generics are used; all types are concrete.
- Functions return owned values (`Vec<State<'a>>`, `Option<T>`), not iterators or `impl Trait`.
- Use `BTreeMap` instead of `HashMap` for deterministic state hashing.

### Error Handling

- **No `anyhow` or `thiserror`** -- custom error types with manual `Display` and `Error` impls.
- `Result` is used sparingly for expected failure paths (e.g., unsupported BPMN elements, livelocks).
- `panic!` is used for invariant violations and programmer errors.
- `unwrap_or_else(|| panic!("..."))` with descriptive messages for "should never fail" cases.
- `match` on `Result`/`Option` is the dominant pattern over the `?` operator.

### Documentation and Comments

- `///` doc comments with `# Arguments` / `# Returns` sections for public APIs (especially in the POR module).
- `//!` module-level doc comments for module files.
- `//` inline comments for implementation notes in core logic.
- `///` on CLI struct fields for clap help text generation.

### Module Organization

- All directories use `mod.rs` as the entry point (not the `foo.rs` + `foo/` pattern).
- Test-only modules are declared as private `mod` in the parent (e.g., `mod reader_test;`).
- Internal modules are private; `lib.rs` selectively re-exports the public API via `pub use`.

### Visibility

- Struct fields are generally `pub`.
- Use `pub(crate)` for cross-module-but-not-external methods.
- Keep implementation helpers `fn` (private).

### Derive Macros and Traits

- Domain types: `#[derive(Debug, PartialEq)]` at minimum; add `Hash`, `Clone` as needed.
- Config/CLI types: `#[derive(Parser, Debug)]` with `#[command(...)]` attributes.
- Serde (`Serialize`/`Deserialize`) only on webserver/wasm DTOs -- never on core domain types.
- Implement `From` trait for conversions between core types and DTOs.
- Use `..Default::default()` for partial struct construction.

### Test Conventions

- **Unit tests**: `#[cfg(test)] mod tests { ... }` in dedicated test files (e.g., `reader_test.rs`).
- **Integration tests**: in each crate's `tests/` directory.
- **Parameterized tests**: use `rstest` with `#[case]` attributes.
- **Test naming**: `snake_case`, descriptive. Older code omits `test_` prefix; newer POR code uses it.
- **Test resources**: BPMN fixture files in `tests/resources/unit/` and `tests/resources/integration/`.
- **Assertions**: `assert_eq!` with full expected struct construction.
- **CLI tests**: `assert_cmd` crate with predicate assertions.
- **Webserver tests**: `tower::ServiceExt::oneshot` for testing axum handlers.

### Async

`async` is confined to the webserver crate (axum + tokio). The core library is fully synchronous.
