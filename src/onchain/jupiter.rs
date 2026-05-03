//! Jupiter swap simulation (dry-run) via rig-onchain-kit pattern.
//! In production: replace mock with rig_onchain_kit::tools::JupiterSwap.

use anyhow::Result;
use tracing::info;

use crate::sor::router::Route;

#[derive(Debug, Clone)]
pub struct SwapResult {
    pub simulated_sig: String,
    pub input_amount: f64,
    pub output_amount: f64,
    pub fee_paid: f64,
    pub is_dry_run: bool,
}

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
