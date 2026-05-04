//! On-chain execution layer.
//!
//! Exposes two public entry points:
//!
//! * [`execute_swap`] — wraps a Jupiter swap simulation inside an isolated
//!   [`signer::LocalSolanaSigner`] context and returns the [`jupiter::SwapResult`].
//! * [`demo_signer`] — spawns three concurrent tasks to demonstrate that
//!   [`signer::CURRENT_SIGNER`] is fully isolated per task.
//!
//! ## Security boundary
//!
//! All on-chain operations are wrapped in [`signer::with_signer`], which uses
//! `tokio::task_local!` to scope the active keypair to exactly one async task.
//! This mirrors the `rig-onchain-kit` `SignerContext` pattern and prevents
//! concurrent tasks from accidentally sharing or overwriting each other's
//! signing credentials.

pub mod balance;
pub mod jupiter;
pub mod signer;
pub mod types;

use anyhow::Result;
use tracing::info;

use crate::config::Config;
use crate::sor::router::Route;
use jupiter::SwapResult;

/// Execute a swap for the given route inside an isolated signer context.
///
/// Loads a [`signer::LocalSolanaSigner`] from the environment, logs the
/// public key, then runs the Jupiter simulation inside [`signer::with_signer`]
/// to ensure the keypair is scoped to this task only.
///
/// The `live` flag is inverted before being passed to [`jupiter::simulate_swap`]
/// because the swap function takes `dry_run` (the logical complement).
///
/// # Errors
///
/// Returns an error if the signer context or Jupiter simulation fails.
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

/// Demonstrate [`signer::with_signer`] isolation across concurrent tasks.
///
/// Delegates directly to [`signer::demo_signer`].
pub async fn demo_signer(cfg: &Config) -> Result<()> {
    signer::demo_signer(cfg).await
}
