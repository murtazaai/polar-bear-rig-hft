//! polar-bear-rig-hft — CLI entry point
//! Polar Bear Systems | Technology Lead: Murtaza Ali Imtiaz
//! Rig (Rust Inference Gateway / ARC) · AVM · SignerContext · PEV Loop

use anyhow::Result;
use clap::{Parser, ValueEnum};
use tracing::info;
use tracing_subscriber::EnvFilter;

mod avm;
mod config;
mod onchain;
mod pev;
mod sor;

#[derive(Debug, Clone, ValueEnum)]
enum Mode {
    /// Run full pipeline: PEV → SOR → OnChain → AVM audit
    Full,
    /// Run only the PEV loop with rig-core
    Pev,
    /// Run only Smart Order Routing comparison
    Sor,
    /// Run only SignerContext + rig-onchain-kit demo
    Signer,
    /// Run only AVM benchmark + Reactor GUI audit log
    Reactor,
}

#[derive(Parser, Debug)]
#[command(name = "polar-bear-rig-hft")]
#[command(about = "Optimal HFT platform using Rig (ARC) — Polar Bear Systems")]
struct Args {
    /// Operating mode
    #[arg(short, long, default_value = "full")]
    mode: Mode,

    /// Enable live on-chain transactions (default: dry-run)
    #[arg(long, default_value_t = false)]
    live: bool,

    /// Trade pair for SOR/onchain (e.g. SOL/USDC)
    #[arg(short, long, default_value = "SOL/USDC")]
    pair: String,

    /// Amount to trade (in base token units)
    #[arg(short, long, default_value_t = 1.0)]
    amount: f64,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("polar_bear_rig_hft=debug".parse()?),
        )
        .init();

    let args = Args::parse();
    let cfg = config::Config::from_env()?;

    info!("╔══════════════════════════════════════════════╗");
    info!("║  POLAR BEAR RIG HFT  ·  Rig (ARC) Platform  ║");
    info!("╚══════════════════════════════════════════════╝");
    info!(mode = ?args.mode, pair = %args.pair, amount = args.amount,
          live = args.live, "Starting platform");

    match args.mode {
        Mode::Full => {
            // 1. PEV loop — plan and reason about the trade
            let pev_result = pev::run(&cfg, &args.pair, args.amount).await?;
            info!(score = pev_result.verify_score, "PEV loop complete");

            // 2. Smart Order Routing — find best execution venue
            let route = sor::best_route(&args.pair, args.amount).await?;
            info!(venue = %route.venue, price = route.effective_price,
                  fee_bps = route.fee_bps, latency_ms = route.latency_ms,
                  "SOR: best route selected");

            // 3. On-chain execution (dry-run unless --live)
            let swap_result = onchain::execute_swap(&cfg, &route, args.live).await?;
            info!(tx_sig = %swap_result.simulated_sig, "Swap simulation complete");

            // 4. AVM audit log (Reactor GUI simulation)
            avm::audit_log(&route, &swap_result)?;
        }
        Mode::Pev => {
            pev::run(&cfg, &args.pair, args.amount).await?;
        }
        Mode::Sor => {
            sor::best_route(&args.pair, args.amount).await?;
        }
        Mode::Signer => {
            onchain::demo_signer(&cfg).await?;
        }
        Mode::Reactor => {
            avm::run_benchmark()?;
        }
    }

    info!("Platform run complete. All operations logged.");
    Ok(())
}
