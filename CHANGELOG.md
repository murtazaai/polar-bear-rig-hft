# Changelog

All notable changes to `polar-bear-rig-hft` are documented here.

This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added
- `examples/sor_demo.rs` - standalone Smart Order Routing demo
- `examples/signer_demo.rs` - SignerContext task-local isolation demo
- `examples/avm_demo.rs` - AVM vs EVM micro-benchmark demo
- `examples/jupiter_dry_run.rs` - Jupiter dry-run swap + Reactor audit log demo
- `tests/providers/anthropic.rs` - live Anthropic integration tests (gated behind `#[ignore]`)
- `.github/workflows/ci.yml` - CI: fmt → clippy → build → test → docs → MSRV check
- `rustfmt.toml` - code-style configuration (mirrors rig upstream)
- `.clippy.toml` - Clippy configuration with MSRV and complexity thresholds
- `CHANGELOG.md` - this file
- `CONTRIBUTING.md` - contribution guide

### Changed
- `src/pev/{plan,execute,verify}.rs` - expanded `use rig::client::CompletionClient` to
  the multi-line form `use rig::{ client::CompletionClient` (Fix 17).
  Fix 16 documented both traits in module-level `//!` comments but only `CompletionClient` was
  present in the actual `use` statement; `CompletionClient` must be *imported*, not just mentioned.
- `Cargo.toml` - upgraded to Rust **2024 edition**; added `rust-version = "1.93.1"`,
  `[package.metadata.docs.rs]`, and `[lints]` tables; removed unused `futures` dependency;
  aligned `thiserror` to `^2`; added `strip = "debuginfo"` to release profile
- `src/pev/{plan,execute,verify}.rs` - added `rig::client::CompletionClient` to imports;
  `.agent()` in rig-core ≥ 0.36 requires both `CompletionClient` 
  in scope
- `src/pev/plan.rs` - converted PLAN_PREAMBLE from `r#"..."#` to `r"..."` (no `#` needed;
  matches 2024 edition style)
- `src/pev/execute.rs` - same preamble conversion
- `.gitignore` - replaced the sprawling multi-project `.gitignore` with a focused
  Rust-only ignore file

### Fixed (historical - see BUG-FIXES.md for full details)
- **Fix 1** - removed fictitious `features = ["anthropic", "openai", "cohere"]` from
  `rig-core` dep; no such features exist
- **Fix 2** - upgraded Solana crates to `^3` / `spl-token` to `^9` to resolve the
  `ed25519-dalek` v1 vs v2 semver conflict
- **Fix 3** - deleted stale `Cargo.lock` with pinned old resolution graph
- **Fix 4** - replaced `crate::` with `polar_bear_rig_hft::` in integration tests
- **Fix 5** - corrected `format!` positional-arg mismatch in `execute.rs`
- **Fix 6** - removed unused `rig::tool::Tool` import from `execute.rs`
- **Fix 7** - merged stray second string literal into the `format!` call in `plan.rs`
- **Fix 8** - added `pub fn default_tasks_pub` alias callable from integration tests
- **Fix 9** - populated empty `src/sor/mod.rs` with `pub use router::best_route`
- **Fix 10** - added `src/lib.rs`; replaced `mod` re-declarations in `main.rs` with
  `use polar_bear_rig_hft::*`
- **Fix 11** - bumped `rig-core` from stale `0.9.1` to `^0.36`
- **Fix 12** - bumped `reqwest` from `^0.12` to `^0.13` to match rig-core transitive dep
- **Fix 13** - renamed `reqwest` feature `rustls-tls` → `rustls` (renamed in 0.13)
- **Fix 14** - corrected Rust doc comment syntax throughout (no `///` on macro sites,
  no duplicate `//!` blocks, correct code-fence languages)
- **Fix 15** - removed `Arc::new(anthropic::Client::new(...))`;  `Client::new` is fallible
  in rig-core 0.36+ and must be unwrapped with `?`; `Arc` was unnecessary
- **Fix 16** - added `rig::client::CompletionClient` import 
  in all three PEV phase files

---

## [0.1.0] - 2025-07-01

Initial implementation of the polar-bear-rig-hft platform:

- PEV loop (Plan → Execute → Verify) with Haiku / Sonnet agents via rig-core
- Smart Order Routing across Raydium, Orca, and Serum (concurrent, cost-adjusted)
- SignerContext task-local keypair isolation (mirrors `rig-onchain-kit` pattern)
- Jupiter swap dry-run simulation
- AVM vs EVM execution micro-benchmark
- Reactor GUI audit log
- CLI with `--mode` flag for running individual subsystems
