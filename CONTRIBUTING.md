# Contributing to polar-bear-rig-hft

> **Polar Bear (🍨)** · Technology Lead: Murtaza Ali Imtiaz
>
> Licensed under [MIT OR Apache-2.0](LICENSE-MIT). Contributions are welcome
> under the same dual licence.

---

## Development environment

### Prerequisites

| Tool | Version | Install |
|---|---|---|
| Rust stable toolchain | ≥ 1.93.1 | `rustup update stable` |
| `rustfmt` | (with toolchain) | `rustup component add rustfmt` |
| `clippy` | (with toolchain) | `rustup component add clippy` |

### Setup

```bash
    git clone https://github.com/murtazaai/polar-bear-rig-hft
cd polar-bear-rig-hft
cp .env.example .env
# Edit .env: optionally set ANTHROPIC_API_KEY=sk-ant-...
```

---

## Workflow

### Build

```bash
cargo build           # debug
cargo build --release # optimised (use for benchmarks)
```

### Run

```bash
# Full pipeline (dry-run)
cargo run --release -- --mode full --pair SOL/USDC --amount 1.0

# Individual subsystems
cargo run --release -- --mode pev
cargo run --release -- --mode sor
cargo run --release -- --mode signer
cargo run --release -- --mode reactor
```

### Examples

```bash
cargo run --example sor_demo
cargo run --example signer_demo
cargo run --example avm_demo --release
cargo run --example jupiter_dry_run
```

### Tests (no API key required)

```bash
cargo test                   # all deterministic tests
SKIP_LLM=1 cargo test        # explicit offline mode
cargo test --test test_sor   # specific test file
```

### Live provider tests (API key required)

```bash
ANTHROPIC_API_KEY=sk-ant-... cargo test --test providers -- --ignored --test-threads=1
```

### Format, lint, docs

```bash
cargo fmt --all                                      # format
cargo clippy --all-targets -- -D warnings            # lint (CI-strict)
cargo doc --open                                     # browse API docs
RUSTDOCFLAGS="--cfg docsrs" cargo doc                # with docsrs conditional items
```

---

## Code style

- **Edition**: Rust 2024
- **MSRV**: 1.93.1 (enforced by `rust-version` in `Cargo.toml` and `msrv` in
  `.clippy.toml`)
- **Max line width**: 100 characters (enforced by `rustfmt.toml`)
- **Imports**: use the multi-line form for `rig` imports; both
  `rig::client::CompletionClient` and `rig::client::ProviderClient` must be imported
  when calling `.agent()` on any rig-core 0.36+ Anthropic client
- **Doc comments**: `//!` for module-level docs; `///` for items; never `///` on
  macro invocation sites (triggers `unused_doc_comments`)
- **Error handling**: always `anyhow::Result`; propagate with `?`; no `unwrap` in
  library code

---

## Adding a new venue adapter (SOR)

1. Add the SDK crate to `Cargo.toml`
2. Implement the `query_*` function in the relevant stub file (`src/sor/raydium.rs`, etc.)
3. Make it `pub` and re-export from `src/sor/mod.rs`
4. Remove the stub query in `src/sor/router.rs` and import the real one
5. Add a test in `tests/test_sor.rs` and optionally a live test in
   `tests/providers/anthropic.rs`

---

## CI

The CI pipeline runs on every push and pull request to `main`:

1. `cargo fmt --all -- --check` - enforces code style
2. `cargo clippy --all-targets -- -D warnings` - enforces lint rules
3. `cargo build --release` - ensures the release binary compiles
4. `SKIP_LLM=1 cargo test --workspace` - runs all deterministic tests
5. `RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo doc` - ensures documentation
   compiles without warnings
6. MSRV check against Rust 1.93.1

---

## Publishing

```bash
# Verify the published package looks correct (no upload)
cargo publish --dry-run

# Publish to crates.io
cargo publish
```

Set a crates.io API token via `cargo login` or `CARGO_REGISTRY_TOKEN`.
