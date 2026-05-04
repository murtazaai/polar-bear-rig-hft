//! AVM execution layer: JIT-compilation benchmark vs standard EVM.
//! Reactor GUI simulation: audit log showing contract state before/after.
//! AVM (Agave Virtual Machine) execution layer.
//!
//! Contains two sub-modules:
//!
//! * [`benchmark`] — micro-benchmark comparing AVM JIT-compiled execution
//!   against EVM bytecode-style interpretation.  Demonstrates the ~8–12×
//!   throughput advantage of the Agave runtime.
//! * [`reactor`] — Reactor GUI audit-log simulation.  Emits a structured
//!   before/after log of a smart contract deployment to give operators a
//!   human-readable execution trace.
//!
//! ## Public API
//!
//! | Function | Description |
//! |---|---|
//! | [`run_benchmark`] | Run 10 000-iteration AVM vs EVM benchmark |
//! | [`audit_log`] | Emit Reactor GUI audit log for a completed swap |

pub mod benchmark;
pub mod reactor;

use crate::{onchain::jupiter::SwapResult, sor::router::Route};
use anyhow::Result;

/// Run the AVM vs EVM execution benchmark and log the speedup factor.
///
/// Delegates to [`benchmark::run`].
///
/// # Errors
///
/// Currently infallible; returns `Ok(())`.  The signature uses `Result` for
/// forward-compatibility with real AVM instrumentation.
pub fn run_benchmark() -> Result<()> {
    benchmark::run()
}

/// Emit a Reactor GUI audit log entry for the given route and swap result.
///
/// Logs state-before, execution details, and state-after at `INFO` level so
/// the output is visible in the default tracing subscriber configuration.
///
/// Delegates to [`reactor::emit_audit_log`].
///
/// # Errors
///
/// Currently infallible.
pub fn audit_log(route: &Route, swap: &SwapResult) -> Result<()> {
    reactor::emit_audit_log(route, swap)
}
