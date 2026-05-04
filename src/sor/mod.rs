//! Smart Order Routing (SOR).
//!
//! Queries Raydium, Orca, and Serum concurrently and selects the execution
//! venue with the lowest **cost-adjusted price**:
//!
//! ```text
//! effective_cost = price × (1 + fee_bps / 10_000)
//! ```
//!
//! The winning [`router::Route`] is returned to the caller and forwarded to
//! the on-chain execution layer.
//!
//! ## Extending to production
//!
//! Each stub file (`raydium.rs`, `orca.rs`, `serum.rs`) is a placeholder.
//! Replace the mock `tokio::sleep` + hard-coded prices in [`router`] with real
//! SDK calls, e.g. the Raydium CLMM SDK or Orca's Whirlpool API.

pub mod orca;
pub mod raydium;
pub mod router;
pub mod serum;

/// Re-export the primary entry point at the module root for ergonomic use:
/// `sor::best_route(pair, amount)`.
pub use router::best_route;
