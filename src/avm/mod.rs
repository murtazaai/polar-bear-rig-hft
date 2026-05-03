//! AVM execution layer: JIT-compilation benchmark vs standard EVM.
//! Reactor GUI simulation: audit log showing contract state before/after.

pub mod benchmark;
pub mod reactor;

use crate::{onchain::jupiter::SwapResult, sor::router::Route};
use anyhow::Result;

pub fn run_benchmark() -> Result<()> {
    benchmark::run()
}

pub fn audit_log(route: &Route, swap: &SwapResult) -> Result<()> {
    reactor::emit_audit_log(route, swap)
}
