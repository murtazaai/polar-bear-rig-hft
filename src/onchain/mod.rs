pub mod balance;
pub mod jupiter;
pub mod signer;
pub mod types;

use anyhow::Result;
use tracing::info;

use crate::config::Config;
use crate::sor::router::Route;
use jupiter::SwapResult;

pub async fn execute_swap(cfg: &Config, route: &Route, live: bool) -> Result<SwapResult> {
    let signer = signer::LocalSolanaSigner::from_env();
    let pubkey = signer.pubkey();
    info!(%pubkey, "[ONCHAIN] SignerContext: signer loaded");

    // Wrap all on-chain operations in SignerContext for thread-local isolation
    signer::with_signer(signer, || async {
        let result = jupiter::simulate_swap(route, 1.0, !live).await?;
        Ok(result)
    })
    .await
}

pub async fn demo_signer(cfg: &Config) -> Result<()> {
    signer::demo_signer(cfg).await
}
