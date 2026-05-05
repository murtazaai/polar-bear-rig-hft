//! # polar-bear-rig-hft
//!
//! **Polar Bear Systems** — Optimal HFT agent framework built on
//! [Rig (ARC)](https://rig.rs).
//!
//! Technology Lead: Murtaza Ali Imtiaz (July 2019 – present).
//!
//! Combines LLM-driven agentic pipelines with on-chain DeFi execution on
//! Solana, enabling statefully supervised, multi-step agent workflows with
//! full PEV loop governance across the Agentic Web.
//!
//! ## Architecture
//!
//! ```text
//! ┌──────────────────────────────────────────────────────┐
//! │                  polar-bear-rig-hft                  │
//! ├──────────┬──────────┬───────────┬─────────┬──────────┤
//! │   pev    │   sor    │  onchain  │   avm   │  config  │
//! │ Plan –   │ Smart    │ Jupiter   │ AVM     │ Env      │
//! │ Execute–│ Order    │ swap +    │ bench-  │ loader   │
//! │ Verify  │ Routing  │ Signer    │ mark    │          │
//! └──────────┴──────────┴───────────┴─────────┴──────────┘
//! ```
//!
//! ## Full pipeline
//!
//! 1. **PEV loop** ([`pev`]) — Haiku decomposes the trade into four
//!    [`pev::types::TradeTask`] objects; Sonnet executes each one using tool
//!    calls; Haiku verifies the output (pass ≥ 0.80). Up to two retries on
//!    failure.
//! 2. **SOR** ([`sor`]) — Raydium, Orca, and Serum are queried concurrently;
//!    the lowest cost-adjusted venue wins.
//! 3. **On-chain** ([`onchain`]) — Jupiter swap is simulated (dry-run by
//!    default) inside an isolated [`onchain::signer::LocalSolanaSigner`]
//!    context.
//! 4. **AVM audit** ([`avm`]) — Reactor GUI audit log records state-before,
//!    execution details, and state-after for every swap.
//!
//! ## Quick start
//!
//! ```text
//! cp .env.example .env   # set ANTHROPIC_API_KEY
//! cargo run --release -- --mode full --pair SOL/USDC --amount 1.0
//! ```

pub mod avm;
pub mod config;
pub mod onchain;
pub mod pev;
pub mod sor;
