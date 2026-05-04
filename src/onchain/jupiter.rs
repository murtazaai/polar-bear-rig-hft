//! Jupiter swap simulation (dry-run) via rig-onchain-kit pattern.
//! In production: replace mock with rig_onchain_kit::tools::JupiterSwap.
//! Jupiter swap simulation (dry-run).
//!
//! Provides a single async function [`simulate_swap`] that mimics the
//! Jupiter Aggregator V6 API without sending any real transaction.  In
//! production, replace this module with `rig_onchain_kit::tools::JupiterSwap`
//! once `rig-onchain-kit` is published to crates.io.
//!
//! ## Dry-run mode
//!
//! When `dry_run = true` (the default via `Config::dry_run`), the function:
//!
//! 1. Computes `output_amount = input * effective_price`.
//! 2. Computes `fee_paid = input * fee_bps / 10_000`.
//! 3. Generates a deterministic-looking simulated signature prefixed `SIM_`.
//!
//! No RPC call is made.  The `SwapResult` is identical in shape to what a real
//! Jupiter swap would return, making the demo easy to upgrade to live mode.

use anyhow::Result;
use tracing::info;

use crate::sor::router::Route;

/// The result of a Jupiter swap (real or simulated).
#[derive(Debug, Clone)]
pub struct SwapResult {
    /// Transaction signature.  Prefixed with `SIM_` in dry-run mode.
    pub simulated_sig: String,

    /// Amount of the base token sent into the swap.
    pub input_amount: f64,

    /// Amount of the quote token received (after fees).
    pub output_amount: f64,

    /// Absolute fee paid in the input token.
    pub fee_paid: f64,

    /// `true` if this was a simulation; `false` if a real transaction was sent.
    pub is_dry_run: bool,
}

/// Simulate (or execute live) a Jupiter swap for the given route and amount.
///
/// In dry-run mode the function is purely computational — no network call is
/// made.  In live mode the function bails with an error because real
/// transaction signing is not implemented in this demo.
///
/// # Arguments
///
/// * `route`   — The selected SOR route (venue, price, fee).
/// * `amount`  — Base-token amount to swap.
/// * `dry_run` — When `true`, simulate only; when `false`, attempt live tx.
///
/// # Errors
///
/// * Returns `Err` if `dry_run = false` (live mode not implemented in demo).
pub async fn simulate_swap(route: &Route, amount: f64, dry_run: bool) -> Result<SwapResult> {
    info!(
        venue = %route.venue,
        price = route.effective_price,
        amount,
        dry_run,
        "[JUPITER] Simulating swap"
    );

    if !dry_run {
        // Production path: use rig-onchain-kit JupiterSwap tool
        // let agent = create_solana_agent();
        // let result = agent.prompt("Swap 1 SOL to USDC via Jupiter").await?;
        anyhow::bail!("Live mode not enabled in demo. Use --dry-run.");
    }

    // Dry-run simulation
    let output = amount * route.effective_price;
    let fee = amount * (route.fee_bps as f64 / 10_000.0);
    let sig = format!("SIM_{:016x}", rand::random::<u64>());

    let result = SwapResult {
        simulated_sig: sig.clone(),
        input_amount: amount,
        output_amount: output,
        fee_paid: fee,
        is_dry_run: true,
    };

    info!(
        sig = %result.simulated_sig,
        input  = result.input_amount,
        output = result.output_amount,
        fee    = result.fee_paid,
        "[JUPITER] Swap simulation complete (DRY RUN)"
    );

    Ok(result)
}
