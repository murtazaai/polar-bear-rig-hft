# polar-bear-rig-hft

**Optimal High-Frequency Trading Platform - Rig (Rust Inference Gateway / ARC)**

[![Rust](https://img.shields.io/badge/rust-1.93.1%2B-orange.svg)](https://www.rust-lang.org/)
[![Edition](https://img.shields.io/badge/edition-2024-blue.svg)](https://doc.rust-lang.org/edition-guide/rust-2024/)
[![rig-core](https://img.shields.io/badge/rig--core-%5E0.36-purple.svg)](https://rig.rs)
[![Solana](https://img.shields.io/badge/solana-devnet%2Fmainnet-9945FF.svg)](https://solana.com)
[![License: PBS](https://img.shields.io/badge/license-PBS-blue.svg)](LICENSE-PBS)

> **Polar Bear Systems** · Technology Lead: [Murtaza Ali Imtiaz](https://github.com/murtazaai) · July 2019 – Present

A production-grade Rust implementation of an LLM-driven HFT agent framework powered by
[Rig (ARC)](https://rig.rs) - the high-performance enterprise alternative to Python LLM
frameworks. The platform demonstrates end-to-end **agentic trade governance** through a
**Plan → Execute → Verify (PEV) loop**, concurrent **Smart Order Routing (SOR)** across
three Solana DEXs, **task-local SignerContext keypair isolation**, **Jupiter swap
simulation**, **AVM vs EVM execution benchmarking**, and a **Reactor GUI audit log** - all
within a single Rust crate that compiles as both a library and a binary.

---

## Architecture

```
  CLI Entry  ─────────────── main.rs (clap: --mode, --pair, --amount, --skip-llm, --live)
       │
  ┌────┴─────────────────────────────────────────────────────────┐
  │  PEV Loop  (src/pev/)                                        │
  │                                                              │
  │   PLAN      claude-haiku-4-5  → Vec<TradeTask> (4 tasks)    │
  │     │                                                        │
  │   EXECUTE   claude-sonnet-4-6 → ExecuteOutput per task      │
  │     │                                                        │
  │   VERIFY    claude-haiku-4-5  → score ∈ [0,1]; pass ≥ 0.80 │
  │     │  score < 0.80: retry up to 2× with feedback injected  │
  └────┬─────────────────────────────────────────────────────────┘
       │
  ┌────┴─────────────────────────────────────────────────────────┐
  │  Smart Order Routing  (src/sor/)                             │
  │                                                              │
  │   tokio::join! ──► Raydium  (25 bps, ~143.52 USDC/SOL)     │
  │                ──► Orca     (30 bps, ~143.48 USDC/SOL)      │
  │                ──► Serum    (20 bps, ~143.61 USDC/SOL)      │
  │                                                              │
  │   Cost formula:  price × (1 + fee_bps / 10_000)            │
  │   Winner: lowest effective cost                              │
  └────┬─────────────────────────────────────────────────────────┘
       │
  ┌────┴─────────────────────────────────────────────────────────┐
  │  On-chain Execution  (src/onchain/)                          │
  │                                                              │
  │   SignerContext (tokio::task_local!) ── keypair isolation    │
  │   Jupiter swap simulation            ── SIM_<hex> signature │
  │   DRY_RUN=true by default            ── no live txns sent   │
  └────┬─────────────────────────────────────────────────────────┘
       │
  ┌────┴─────────────────────────────────────────────────────────┐
  │  AVM Layer  (src/avm/)                                       │
  │                                                              │
  │   Benchmark: AVM JIT (~1–3 ns/op) vs EVM (~10–30 ns/op)    │
  │   Reactor GUI audit log: STATE BEFORE → EXECUTION → AFTER   │
  └──────────────────────────────────────────────────────────────┘
```

---

## Tech Stack

| Layer | Technology | Notes |
|---|---|---|
| AI Agent Framework | [rig-core](https://crates.io/crates/rig-core) `^0.36` | Rig / ARC - Rust Inference Gateway |
| LLM - Plan & Verify | `claude-haiku-4-5` | Low-cost model for structured decomposition and scoring |
| LLM - Execute | `claude-sonnet-4-6` | High-capability model for agentic tool-use reasoning |
| Async Runtime | [Tokio](https://tokio.rs) `^1` (full features) | Drives all async tasks and `task_local!` storage |
| Blockchain | Solana (`solana-sdk ^3`, `solana-client ^3`) | Devnet by default; mainnet-ready |
| Token | `spl-token ^9` | Aligned with Solana SDK 3.x |
| Cryptography | `ed25519-dalek ^2`, `k256 ^0.13` | ECDSA and Ed25519 signing primitives |
| HTTP / TLS | `reqwest ^0.13` (rustls) | JSON requests; rustls avoids OpenSSL dependency |
| CLI | `clap ^4` (derive) | `--mode`, `--pair`, `--amount`, `--skip-llm`, `--live` |
| Logging | `tracing ^0.1` + `tracing-subscriber ^0.3` | Structured fields; env-filter for `RUST_LOG` |
| Error handling | `anyhow ^1`, `thiserror ^2` | `?`-based propagation throughout |
| Env config | `dotenvy ^0.15` | Loads `.env` before `Config::from_env()` |
| IDE | [Zed](https://zed.dev) | `.zed/settings.json`, `tasks.json`, `debug.json` |

---

## Quick Start

```bash
# 1. Clone and enter
git clone https://github.com/murtazaai/polar-bear-rig-hft
cd polar-bear-rig-hft

# 2. Configure environment
cp .env.example .env
# Optional: set ANTHROPIC_API_KEY=sk-ant-... for live LLM mode
# All subsystems work without a key - see Offline/Stub Mode below

# 3. Build
cargo build --release

# 4. Run - full pipeline, offline stub (no API key needed)
cargo run --release -- --mode full --skip-llm --pair SOL/USDC --amount 1.0

# 5. Run - full pipeline, live LLM (requires ANTHROPIC_API_KEY in .env)
cargo run --release -- --mode full --pair SOL/USDC --amount 1.0
```

---

## CLI Reference

```
USAGE:
    polar-bear-rig-hft [OPTIONS]

OPTIONS:
    -m, --mode <MODE>        Operating mode [default: full]
                             [possible values: full, pev, sor, signer, reactor]
    -p, --pair <PAIR>        Trading pair, e.g. SOL/USDC [default: SOL/USDC]
    -a, --amount <AMOUNT>    Base-token amount [default: 1.0]
        --skip-llm           Force offline stub mode for PEV phases
                             Implied automatically when ANTHROPIC_API_KEY is absent
        --live               Enable live on-chain transactions (dry-run by default)
    -h, --help               Print help
```

### Modes

| Mode | Subsystems exercised | API key required? |
|---|---|---|
| `full` | PEV loop → SOR → Jupiter swap → AVM audit log | No (stub) / Yes (live LLM) |
| `pev` | PEV loop only | No (stub) / Yes (live LLM) |
| `sor` | Smart Order Routing only | No |
| `signer` | SignerContext isolation demo | No |
| `reactor` | AVM benchmark + Reactor audit log | No |

---

## Offline / Stub Mode

Every subsystem runs without an `ANTHROPIC_API_KEY`. When a key is absent (or
`--skip-llm` is passed), the PEV phases substitute deterministic offline stubs:

| Phase | Live path | Stub path |
|---|---|---|
| **Plan** | Haiku decomposes to JSON via API | Returns `default_tasks()` - 4 canonical tasks |
| **Execute** | Sonnet reasons and calls tools | Returns fixed `ExecuteOutput`; confidence = 0.90 |
| **Verify** | Haiku scores against criteria | Returns score = 0.90, feedback = "all criteria assumed met" |

SOR, SignerContext, Jupiter dry-run, and AVM benchmark are unaffected - they never
require an API key.

Stub mode is activated by **any** of the following:

| Method | When to use |
|---|---|
| Leave `ANTHROPIC_API_KEY` blank in `.env` | Default; no key provisioned |
| `--skip-llm` CLI flag | Force stub for a single `cargo run` invocation |
| `SKIP_LLM=1 cargo run` | Force stub via environment variable |
| `SKIP_LLM=1` set permanently in `.env` | Always-on for a whole checkout |

> **Important - `cargo test`:** `--skip-llm` is a flag for the compiled binary's clap
> parser. **Never** pass it after `--` in a `cargo test` invocation - the test harness
> uses getopts and will reject it with "Unrecognised option: 'skip-llm'".
>
> ```text
> cargo test -- --skip-llm   # ✗ WRONG - test harness rejects it
> SKIP_LLM=1 cargo test      # ✓ correct
> ```

---

## Environment Variables

All variables are optional. `Config::from_env()` is infallible - every variable has a
safe default. Call `dotenvy::dotenv()` first (done automatically by `main.rs`) to pick up
`.env` from disk.

| Variable | Default | Description |
|---|---|---|
| `ANTHROPIC_API_KEY` | `""` | Anthropic API key for rig-core. Absent → `skip_llm = true`; all PEV phases use stubs. |
| `SKIP_LLM` | `false` | Set to `1` or `true` to force stub mode even when a key is present. |
| `SOLANA_RPC_URL` | `https://api.devnet.solana.com` | Solana JSON-RPC endpoint. |
| `SOLANA_PRIVATE_KEY` | `DEMO_KEY_PLACEHOLDER` | Base-58 encoded keypair for signing. In production load from a secrets manager. |
| `DRY_RUN` | `true` | When `true`, all on-chain operations are simulated and no real transactions are signed or broadcast. |

Log level defaults to `polar_bear_rig_hft=debug`. Override at runtime:

```bash
RUST_LOG=debug cargo run --release -- --mode full
RUST_LOG=polar_bear_rig_hft=trace cargo run --release -- --mode pev
```

---

## PEV Loop - Plan → Execute → Verify

The PEV loop governs every trade decision. A single `pev::run()` call:

1. **PLAN** - `claude-haiku-4-5` decomposes the trade into exactly **4 atomic
   `TradeTask` objects**:

   | Task ID | `TradeAction` | Acceptance criteria |
   |---|---|---|
   | `T001` | `analyse_market` | Market data retrieved |
   | `T002` | `select_route` | Best DEX venue selected |
   | `T003` | `validate_slippage` | Slippage within 0.5 % tolerance |
   | `T004` | `simulate_execution` | Dry-run swap simulation logged |

2. **EXECUTE** - `claude-sonnet-4-6` processes each task, invoking the mapped tool:

   | Action | Tool call |
   |---|---|
   | `analyse_market` | `fetch_price_feed(SOL/USDC)` |
   | `select_route` | `query_raydium_pool()`, `query_orca_pool()` |
   | `validate_slippage` | `calculate_slippage(amount)` |
   | `simulate_execution` | `jupiter_swap_dry_run()` |

3. **VERIFY** - `claude-haiku-4-5` scores the output against criteria.
   - Pass threshold: **≥ 0.80**
   - On failure: feedback is injected into the next attempt
   - Max retries per task: **2**

Cost model: Haiku handles cheap plan and verify work; Sonnet is reserved for
reasoning-heavy execution. This cuts LLM cost by roughly 60–70 % compared with an
all-Sonnet pipeline.

---

## Smart Order Routing (SOR)

`sor::best_route()` fans out to all three venues **simultaneously** using
`tokio::join!`, then selects the winner by effective cost:

```
effective_cost = price × (1 + fee_bps / 10_000)
```

| Venue | Price (SOL/USDC) | Fee | Price impact | Simulated latency |
|---|---|---|---|---|
| Raydium | 143.52 | 25 bps | 0.03 % | 12 ms |
| Orca | 143.48 | 30 bps | 0.02 % | 9 ms |
| Serum (OpenBook) | 143.61 | 20 bps | 0.05 % | 15 ms |

If all venue queries fail, a `Raydium-fallback` route is returned so the
pipeline is never blocked.

---

## On-chain Execution & SignerContext

### SignerContext (`src/onchain/signer.rs`)

Uses `tokio::task_local!` to scope a `solana_sdk::signature::Keypair` to exactly
one Tokio task. Multiple concurrent trades cannot share or leak each other's signing
keys, with no mutex overhead.

```rust
let signer = LocalSolanaSigner::from_env();
let result = with_signer(signer, || async {
    // CURRENT_SIGNER is only visible inside this async block
    Ok::<_, anyhow::Error>(execute_trade().await?)
}).await?;
```

### Jupiter Swap Simulation (`src/onchain/jupiter.rs`)

`simulate_swap()` computes output without sending any RPC call:

```
output_amount = input_amount × effective_price
fee_paid      = input_amount × fee_bps / 10_000
simulated_sig = "SIM_" + 16-char random hex
```

`is_dry_run = true` is always set in demo mode. Pass `--live` to the binary to
attempt live mode (currently returns `Err` - production wiring is in progress).

---

## AVM Benchmark & Reactor Audit Log

### AVM vs EVM Benchmark (`src/avm/benchmark.rs`)

Runs 10 000 iterations of each engine and logs ns/op and the speedup ratio:

| Engine | Method | Typical result |
|---|---|---|
| AVM (Agave JIT) | `#[inline(always)]`, stack-only arithmetic, zero heap allocation | ~1–3 ns/op |
| EVM (bytecode) | `#[inline(never)]`, one `Vec` allocation per call | ~10–30 ns/op |

Run with `--release` for meaningful timing; debug builds omit optimisations.

### Reactor GUI Audit Log (`src/avm/reactor.rs`)

Emits a structured three-phase execution trace at `INFO` level:

```
[REACTOR GUI] ── STATE BEFORE ──  Balance, pool, price, fee
[REACTOR GUI] ── EXECUTION ──     Method, compute units, AVM mode, signature
[REACTOR GUI] ── STATE AFTER ──   Output amount, fee paid, status: SUCCESS
```

---

## Build & Test

### Prerequisites

| Requirement | Version | Install |
|---|---|---|
| Rust stable | ≥ 1.93.1 (MSRV) | `rustup update stable` |
| `rustfmt` | bundled with toolchain | `rustup component add rustfmt` |
| `clippy` | bundled with toolchain | `rustup component add clippy` |
| `ANTHROPIC_API_KEY` | - | Optional. Absent → offline stub mode. Required only for `#[ignore]` live tests. |

### Setup

```bash
git clone https://github.com/murtazaai/polar-bear-rig-hft
cd polar-bear-rig-hft
cp .env.example .env
# Edit .env: optionally set ANTHROPIC_API_KEY=sk-ant-...
```

### Build commands

```bash
cargo build                 # debug build (CARGO_INCREMENTAL=1)
cargo build --release       # optimised - required for meaningful benchmark timing
cargo check --all-targets   # type-check only, no linking (fastest feedback)
cargo clean                 # remove target/
```

**Release profile** (`Cargo.toml`):

```toml
[profile.release]
opt-level     = 3
lto           = true
codegen-units = 1
panic         = "abort"
strip         = "debuginfo"
```

### Test commands

All tests in `tests/*.rs` are fully deterministic and pass without an API key.

```bash
cargo test                                    # all deterministic tests
SKIP_LLM=1 cargo test                         # explicit offline mode
cargo test -- --nocapture                     # with log output to stdout
cargo test -- --test-threads=8               # parallel (default)
cargo test --test test_avm_benchmark          # single file
cargo test --test test_pev_loop               # single file
cargo test --test test_signer_context         # single file
cargo test --test test_sor                    # single file
```

**Live provider tests** (API key required, `#[ignore]` in CI):

```bash
ANTHROPIC_API_KEY=sk-ant-... \
    cargo test --test providers -- --ignored --test-threads=1
```

Use `--test-threads=1` to avoid concurrent requests hitting rate limits.

### Full test inventory

#### `tests/test_pev_loop.rs` - unit + stub integration (10 tests)

| Test | Asserts |
|---|---|
| `test_config_from_env_succeeds_without_key` | `from_env()` never returns `Err`; `skip_llm == !has_api_key()` |
| `test_config_has_api_key_empty` | empty key → `has_api_key() == false` |
| `test_config_has_api_key_present` | non-empty key → `has_api_key() == true` |
| `test_pev_run_stub_mode_passes` | full loop, `skip_llm=true` → `passed=true`; 4 tasks, 4 outputs |
| `test_plan_decompose_stub_mode` | stub returns 4 tasks; amount preserved |
| `test_verify_score_stub_mode` | stub score ≥ 0.80; feedback contains "stub" |
| `test_plan_default_tasks_count` | `default_tasks_pub` returns exactly 4 tasks |
| `test_verify_pass_threshold` | `PASS_THRESHOLD == 0.80` |
| `test_trade_task_serialization` | JSON round-trip preserves pair and action |
| `test_execute_output_tool_calls` | `tool_calls` is non-empty and references the correct tool |

#### `tests/test_sor.rs` - async integration (3 tests)

| Test | Asserts |
|---|---|
| `test_best_route_returns_known_venue` | venue ∈ {Raydium, Orca, Serum}; price > 0; fee\_bps > 0 |
| `test_sor_latency_recorded` | `latency_ms > 0` |
| `test_cost_ordering_lower_fee_wins` | cost formula correct |

#### `tests/test_signer_context.rs` - async integration (2 tests)

| Test | Asserts |
|---|---|
| `test_signer_context_isolation` | 2 concurrent tasks; independent signers; no error |
| `test_jupiter_dry_run_returns_simulated_sig` | `is_dry_run=true`; sig starts with `SIM_`; output > 0 |

#### `tests/test_avm_benchmark.rs` - unit (1 test)

| Test | Asserts |
|---|---|
| `test_benchmark_completes_without_error` | `run_benchmark()` returns `Ok(())` |

#### `tests/providers/anthropic.rs` - live, `#[ignore]` (2 tests)

| Test | Asserts |
|---|---|
| `test_live_plan_decomposes_four_tasks` | Haiku returns 4 tasks with correct pair and amount |
| `test_live_pev_loop_passes` | full loop passes with score ≥ 0.80 |

### Lint, format, docs

```bash
cargo fmt --all                                      # apply formatting (rustfmt.toml)
cargo fmt --all -- --check                           # CI format check
cargo clippy --all-targets -- -D warnings            # lint in CI mode (zero warnings)
cargo clippy --all-targets --fix --allow-dirty       # auto-fix suggestions
cargo doc --open --document-private-items            # browse rustdoc locally
RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo doc   # CI docs check
```

### Running the binary

```bash
# Full pipeline - offline stub
cargo run --release -- --mode full --skip-llm --pair SOL/USDC --amount 1.0

# Full pipeline - live LLM (requires ANTHROPIC_API_KEY)
cargo run --release -- --mode full --pair SOL/USDC --amount 1.0

# Individual subsystems (never need an API key)
cargo run --release -- --mode pev --skip-llm
cargo run --release -- --mode sor
cargo run --release -- --mode signer
cargo run --release -- --mode reactor

# Help
cargo run --release -- --help
```

### Standalone examples

Each example is self-contained and runnable with a single `cargo run` command.

```bash
cargo run --release --example sor_demo        # concurrent SOR, prints winning route
cargo run --release --example signer_demo     # SignerContext isolation across 3 tasks
cargo run --release --example avm_demo        # AVM vs EVM benchmark (use --release)
cargo run --release --example jupiter_dry_run # Jupiter swap simulation + Reactor log
```

---

## Zed IDE Configuration (`.zed/`)

Three project-local config files are provided for [Zed](https://zed.dev):

| File | Contents |
|---|---|
| `.zed/settings.json` | rust-analyzer tuned to `rustfmt.toml` and `.clippy.toml`; format-on-save; inlay hints; import grouping matching `imports_granularity = "Crate"` |
| `.zed/tasks.json` | 29 tasks covering build, test (per-file + live providers), lint, fmt, doc, run (all 5 modes), all 4 examples, and a one-shot local CI simulation |
| `.zed/debug.json` | 15 CodeLLDB debug configurations: all binary modes (dev + release), all 4 integration test files (via `--no-run` + glob program path), and tooling checks |

---

## CI Pipeline

`.github/workflows/ci.yml` runs on every push and pull request to `main`:

| Step | Command | What it enforces |
|---|---|---|
| 1 | `cargo fmt --all -- --check` | Code style (rustfmt.toml) |
| 2 | `cargo clippy --all-targets -- -D warnings` | Zero lint warnings |
| 3 | `cargo build --release` | Release binary compiles |
| 4 | `SKIP_LLM=1 cargo test --workspace` | All deterministic tests pass |
| 5 | `RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo doc` | Docs compile without warnings |
| 6 | MSRV check | Compiles on Rust 1.93.1 |

---

## Repository Structure

```
polar-bear-rig-hft/
├── Cargo.toml              Rust 2024; MSRV 1.93.1; all deps; [lints] table
├── Cargo.lock              Committed (binary crate)
├── rustfmt.toml            100-col, Rust 2024 edition, crate-level import grouping
├── .clippy.toml            MSRV 1.93.1, cognitive-complexity 30, API-breakage protection
├── .env.example            All optional env vars (ANTHROPIC_API_KEY, SOLANA_RPC_URL, …)
├── .gitignore              Focused Rust-only ignore file
├── LICENSE-PBS             Proprietary licence - Polar Bear Systems
├── README.md               This file
├── CHANGELOG.md            All 18 bug fixes + version history
├── BUG-FIXES.md            Root-cause analysis for all fixes
├── CONTRIBUTING.md         Development workflow and code-style guide
├── FILE_STRUCTURE.md       Annotated directory tree
│
├── .github/workflows/
│   └── ci.yml              fmt → clippy → build → test → docs → MSRV
│
├── .zed/
│   ├── settings.json       rust-analyzer, format-on-save, inlay hints
│   ├── tasks.json          29 Zed tasks (build/test/lint/fmt/doc/run/CI)
│   └── debug.json          15 CodeLLDB debug configurations
│
├── docs/
│   ├── architecture.md     System architecture deep-dive
│   ├── star_story.md       Project narrative
│   └── screen_capture_guide.md
│
├── examples/
│   ├── sor_demo.rs         SOR across 3 venues - no API key needed
│   ├── signer_demo.rs      SignerContext isolation - no API key needed
│   ├── avm_demo.rs         AVM benchmark - run with --release
│   └── jupiter_dry_run.rs  Jupiter swap + Reactor log - no API key needed
│
├── src/
│   ├── lib.rs              Crate root; re-exports all 5 modules
│   ├── main.rs             Binary entry; CLI (clap)
│   ├── config.rs           Config::from_env(); skip_llm; has_api_key()
│   ├── pev/                Plan → Execute → Verify loop
│   │   ├── mod.rs              Orchestrator; MAX_RETRIES = 2
│   │   ├── types.rs            TradeTask, TradeAction, ExecuteOutput, PEVResult
│   │   ├── plan.rs             Haiku decomposition; default_tasks_pub()
│   │   ├── execute.rs          Sonnet execution; action_tool_calls()
│   │   └── verify.rs           Haiku scoring; PASS_THRESHOLD = 0.80
│   ├── sor/                Smart Order Routing
│   │   ├── mod.rs              pub use router::best_route
│   │   ├── router.rs           tokio::join! fan-out; cost ranking; Route struct
│   │   ├── raydium.rs          Stub - Raydium CLMM SDK adapter
│   │   ├── orca.rs             Stub - Orca Whirlpool adapter
│   │   └── serum.rs            Stub - OpenBook CLOB adapter
│   ├── onchain/            On-chain execution
│   │   ├── mod.rs              execute_swap(); demo_signer()
│   │   ├── signer.rs           LocalSolanaSigner; with_signer(); task_local!
│   │   ├── jupiter.rs          simulate_swap(); SwapResult; SIM_ signature
│   │   ├── balance.rs          Stub - sol_balance / token_balance
│   │   └── types.rs            Stub - Lamports, TokenAmount, TxStatus
│   └── avm/                AVM execution layer
│       ├── mod.rs              run_benchmark(); audit_log()
│       ├── benchmark.rs        AVM JIT vs EVM, 10 000 iterations
│       └── reactor.rs          Reactor GUI audit log (3-phase structured trace)
│
└── tests/
    ├── test_pev_loop.rs        10 tests - Config, PEV stub paths, types
    ├── test_sor.rs             3 tests  - best_route, cost ordering, latency
    ├── test_signer_context.rs  2 tests  - SignerContext isolation, Jupiter dry-run
    ├── test_avm_benchmark.rs   1 test   - benchmark smoke test
    └── providers/
        └── anthropic.rs        2 tests  - live, #[ignore], requires ANTHROPIC_API_KEY
```

---

## Key Design Decisions

| Decision | Rationale |
|---|---|
| Lib + bin targets from the same source tree | Integration tests are separate crates; `polar_bear_rig_hft::` is the correct import prefix |
| Rust 2024 edition | Matches the rig upstream repository; required by MSRV 1.93.1+ |
| Haiku for Plan + Verify, Sonnet for Execute | 60–70 % cost reduction vs all-Sonnet; Haiku handles structured, low-complexity steps |
| `CompletionClient` import in all PEV files | Required by rig-core ≥ 0.36 for `.agent()` method resolution (Fixes 16–18) |
| `Client::new(&key)?` not `Arc::new(Client::new(...))` | `Client::new` is fallible in rig-core 0.36+ (Fix 15) |
| `tokio::task_local!` with `//` not `///` | rustdoc cannot attach to macro invocation sites - triggers `unused_doc_comments` |
| `#[ignore]` on live provider tests | Prevents CI failures when `ANTHROPIC_API_KEY` is absent |
| `strip = "debuginfo"` in release profile | Smaller binary; mirrors rig's own release profile |
| `CARGO_INCREMENTAL=0` for release builds | Required when `lto = true` |
| Fallback route on all-venue failure | Pipeline never blocked by transient DEX outages |

---

## Related

- [Star Story](./docs/star_story.md) - project narrative
- [Architecture Diagram](./docs/architecture.md) - deep-dive system design
- [Screen Capture Guide](./docs/screen_capture_guide.md) - key output walkthroughs
- [Rig Framework](https://rig.rs) · [0xPlaygrounds/rig](https://github.com/0xPlaygrounds/rig)
- [arc.fun](https://arc.fun) · [Ryzome](https://ryzome.ai)
- [Solana Program Library](https://spl.solana.com/)
- [Jupiter Aggregator](https://jup.ag/) · [Raydium](https://raydium.io/) · [Orca](https://www.orca.so/)

---

## License

Proprietary - © 2026 Murtaza Ali Imtiaz / Polar Bear Systems  
See [LICENSE-PBS](LICENSE-PBS) for permitted use.

---

## Author

**Murtaza Ali Imtiaz** - Technology Lead, Polar Bear Systems (July 2019 – Present)

- GitHub: [@murtazaai](https://github.com/murtazaai)
- LinkedIn: [linkedin.com/in/murtazai](https://linkedin.com/in/murtazai)
- Portfolio: [murtazai.com](https://murtazai.com)
