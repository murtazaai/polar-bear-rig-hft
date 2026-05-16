# polar-bear-rig-hft

**Optimal High-Frequency Trading Platform using Rig (Rust Inference Gateway / ARC)**

> Technology Lead: Murtaza Ali Imtiaz · Polar Bear Systems · July 2019–Present

A production-grade Rust implementation of an HFT agent framework powered by
[Rig (ARC)](https://rig.rs), the high-performance, enterprise-alternative to
Python LLM frameworks. Demonstrates **rig-core agent pipelines**, **ARC Virtual
Machine (AVM)** execution, **rig-onchain-kit** Solana/EVM operations, **Smart Order
Routing**, and **SignerContext** thread-local signer isolation with full **PEV loop**
(Plan-Execute-Verify) governance across the Agentic Web.

---

## Architecture

```
  CLI Entry (main.rs)
       │
  ┌────┴─────────────┐
  │   PEV Loop        │──── rig-core (claude-haiku plan, claude-sonnet execute)
  │   Plan → Execute  │
  │   → Verify        │
  └────┬─────────────┘
       │
  ┌────┴─────────────┐
  │   Smart Order     │──── Raydium │ Orca │ Serum (concurrent venue comparison)
  │   Routing (SOR)   │
  └────┬─────────────┘
       │
  ┌────┴─────────────┐
  │  rig-onchain-kit  │──── SignerContext │ Jupiter swap │ Balance query
  │  + SignerContext   │
  └────┬─────────────┘
       │
  ┌────┴─────────────┐
  │  AVM Layer        │──── JIT benchmark vs EVM │ Reactor GUI audit log
  └───────────────────┘
```

---

## Tech Stack

| Layer | Technology |
|---|---|
| AI Agent Framework | [rig-core](https://crates.io/crates/rig-core) (Rig / ARC) |
| On-chain Bridge | rig-onchain-kit (Solana + EVM) |
| Async Runtime | Tokio |
| Blockchain | Solana (devnet) |
| Cryptography | ed25519-dalek, k256 (ECDSA) |
| CLI | clap |
| Logging | tracing + tracing-subscriber |

---

## Simple Build

```bash
# Prerequisites: Rust stable (rustup.rs)
git clone https://github.com/murtazaai/polar-bear-rig-hft
cd polar-bear-rig-hft
cp .env.example .env
# Add your ANTHROPIC_API_KEY to .env

# Build (release)
cargo build --release

# Run full pipeline (dry-run by default)
cargo run -- --mode full --pair SOL/USDC --amount 1.0

# Run individual modes
cargo run -- --mode pev
cargo run -- --mode sor
cargo run -- --mode signer
cargo run -- --mode reactor

# Tests
cargo test
cargo clippy -- -D warnings
```

---

## Complete Test and Build

---

### Prerequisites

| Requirement | Version | Notes |
|---|---|---|
| Rust stable | >= 1.93.1 (MSRV) | `rustup update stable` |
| `rustfmt` | bundled | `rustup component add rustfmt` |
| `clippy` | bundled | `rustup component add clippy` |
| `ANTHROPIC_API_KEY` | — | Only needed for `--mode full` and `#[ignore]` live tests |

---

### Setup

```text
git clone https://github.com/murtazaai/polar-bear-rig-hft
cd polar-bear-rig-hft
cp .env.example .env
# Edit .env: ANTHROPIC_API_KEY=sk-ant-...
```

---

### Building

```text
cargo clean                  # remove target/ directory
cargo build                  # debug build
cargo build --release        # optimised — required for meaningful benchmark timing
cargo check                  # type-check only; no linking
```

**Release profile (`Cargo.toml`):**
```toml
[profile.release]
opt-level     = 3
lto           = true
codegen-units = 1
panic         = "abort"
strip         = "debuginfo"
```

---

### Tests

All tests in `tests/*.rs` run without a live API key:

```text
cargo test                                          # all deterministic tests
cargo test -- --nocapture                           # with log output
cargo test --test test_avm_benchmark                # single file
cargo test --test test_pev_loop                     # single file
cargo test --test test_signer_context               # single file
cargo test --test test_sor                          # single file
<!--cargo test --test test_best_route_returns_known_venue      # single function-->
```

**Live provider tests** (API key required, skipped in CI):
```text
ANTHROPIC_API_KEY=sk-ant-... \
    cargo test --test providers -- --ignored --test-threads=1
```

Use `--test-threads=1` to avoid concurrent API calls hitting rate limits.

---

### Full test inventory

#### `tests/test_pev_loop.rs` — unit, no API key

| Test | Asserts |
|---|---|
| `test_plan_default_tasks_count` | 4 tasks returned |
| `test_verify_pass_threshold` | `PASS_THRESHOLD == 0.80` |
| `test_trade_task_serialization` | JSON round-trip preserves pair and action |
| `test_execute_output_tool_calls` | `tool_calls` populated |

#### `tests/test_sor.rs` — async integration, no API key

| Test | Asserts |
|---|---|
| `test_best_route_returns_known_venue` | venue in {Raydium, Orca, Serum}; price > 0 |
| `test_sor_latency_recorded` | `latency_ms > 0` |
| `test_cost_ordering_lower_fee_wins` | cost formula correct |

#### `tests/test_signer_context.rs` — async integration, no API key

| Test | Asserts |
|---|---|
| `test_signer_context_isolation` | 2 concurrent tasks; independent signers; no error |
| `test_jupiter_dry_run_returns_simulated_sig` | `is_dry_run`, sig starts with `SIM_`, output > 0 |

#### `tests/test_avm_benchmark.rs` — unit, no API key

| Test | Asserts |
|---|---|
| `test_benchmark_completes_without_error` | `run_benchmark()` returns `Ok(())` |

#### `tests/providers/anthropic.rs` — live, `#[ignore]`

| Test | Asserts |
|---|---|
| `test_live_plan_decomposes_four_tasks` | Haiku returns 4 tasks with correct pair |
| `test_live_pev_loop_passes` | full loop passes with score >= 0.80 |
| `test_live_sor_returns_known_venue` | valid venue name |

---

### Lint, format, docs

```text
cargo fmt --all                                     # format
cargo fmt --all -- --check                          # CI format check
cargo clippy --all-targets -- -D warnings           # lint (CI mode)
cargo doc --open                                    # browse rustdoc locally
RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo doc   # CI docs check
```
---

### Running the binary

```text
# Full pipeline
cargo run --release -- --mode full --pair SOL/USDC --amount 1.0

# Individual subsystems
cargo run --release -- --mode pev
cargo run --release -- --mode sor
cargo run --release -- --mode signer
cargo run --release -- --mode reactor

# Help
cargo run --release -- --help
```

---

## Related

- [Star Story](./docs/star_story.md)
- [High-Level Architecture Diagram](./docs/architecture.md)
- [Key Outputs (screen capture)](./docs/screen_capture_guide.md)

- [Rig Framework](https://rig.rs) · [0xPlaygrounds](https://github.com/0xPlaygrounds/rig)
- [arc.fun](https://arc.fun) · [Ryzome](https://ryzome.ai)
- [Solana Program Library](https://spl.solana.com/)
- [Jupiter](https://jup.ag/) · [Raydium](https://raydium.io/)

---

## License

PBS License: [PBS License](./LICENSE-PBS)

---

## Author

**Murtaza Ali Imtiaz**

- LinkedIn: [LinkedIn](https://linkedin.com/in/murtazai)
- GitHub: [@murtazaai](https://github.com/murtazaai)
- Portfolio: [murtazai.com](https://murtazai.com)
