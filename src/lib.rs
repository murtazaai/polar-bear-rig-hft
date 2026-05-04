//! # polar-bear-rig-hft
//!
//! **Polar Bear Systems** — HFT agent framework built on [Rig (ARC)](https://rig.rs).
//!
//! Combines LLM-driven agent pipelines with on-chain DeFi execution on Solana.
//! Technology Lead: Murtaza Ali Imtiaz.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────┐
//! │                  polar-bear-rig-hft                 │
//! ├──────────┬──────────┬───────────┬────────┬──────────┤
//! │   pev    │   sor    │  onchain  │  avm   │  config  │
//! │ Plan–    │ Smart    │ Jupiter   │ AVM    │ Env      │
//! │ Execute– │ Order    │ swap +    │ bench- │ loader   │
//! │ Verify   │ Routing  │ Signer    │ mark   │          │
//! └──────────┴──────────┴───────────┴────────┴──────────┘
//! ```
//!
//! ## Pipeline (full mode)
//!
//! 1. **PEV loop** — Haiku decomposes the trade into 4 [`pev::types::TradeTask`] objects;
//!    Sonnet executes each with tool calls; Haiku verifies the output (pass ≥ 0.80).
//! 2. **SOR** — Raydium, Orca, and Serum are queried concurrently; the lowest
//!    cost-adjusted venue wins.
//! 3. **On-chain** — Jupiter swap is simulated (dry-run) inside an isolated
//!    [`onchain::signer::LocalSolanaSigner`] context.
//! 4. **AVM audit** — Reactor GUI audit log records state-before / execution /
//!    state-after.
//!
//! ## Quick start
//!
//! ```bash
//! cp .env.example .env   # set ANTHROPIC_API_KEY
//! cargo run --release -- --mode full --pair SOL/USDC --amount 1.0
//! ```

pub mod avm;
pub mod config;
pub mod onchain;
pub mod pev;
pub mod sor;
