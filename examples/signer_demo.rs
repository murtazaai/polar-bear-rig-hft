//! SignerContext isolation demo.
//!
//! Spawns three concurrent Tokio tasks and demonstrates that each task holds an
//! independent `LocalSolanaSigner` in its task-local storage. No keypair leaks
//! between tasks even though they overlap in wall-clock time.
//!
//! ```text
//! cargo run --example signer_demo
//! ```
//!
//! No `ANTHROPIC_API_KEY` is required; no on-chain call is made.

use anyhow::Result;
use polar_bear_rig_hft::config::Config;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("polar_bear_rig_hft=debug"))
        .init();

    // Config::from_env requires ANTHROPIC_API_KEY; provide a dummy value for
    // this demo since we only exercise the signer, not the LLM path.
    std::env::set_var("ANTHROPIC_API_KEY", "demo-not-used");
    let cfg = Config::from_env()?;

    polar_bear_rig_hft::onchain::demo_signer(&cfg).await?;
    println!("SignerContext isolation demo complete.");
    Ok(())
}
