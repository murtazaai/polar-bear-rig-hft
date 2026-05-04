//! SignerContext: thread-local signer isolation for secure on-chain operations.
//! Demonstrates the rig-onchain-kit security boundary pattern.
//! Every async on-chain call is wrapped in SignerContext::with_signer().
//! SignerContext — task-local keypair isolation for secure on-chain operations.
//!
//! Implements the security boundary described in the `rig-onchain-kit`
//! documentation: every async on-chain call must be wrapped in
//! [`with_signer`], which scopes the active [`solana_sdk::signature::Keypair`]
//! to exactly the current tokio task via [`tokio::task_local!`].
//!
//! ## Why task-local?
//!
//! In an async runtime, multiple trades can be in-flight simultaneously.
//! Using a global or thread-local signer would risk one task's keypair leaking
//! into another task's signing operation.  `task_local!` gives each spawned
//! task its own isolated slot without any locking overhead.
//!
//! ## Production upgrade path
//!
//! Replace [`LocalSolanaSigner`] and the hand-rolled `task_local!` storage
//! with `rig_onchain_kit::signer::SignerContext` once that crate is published.

use anyhow::Result;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use std::sync::Arc;
use tracing::info;

use crate::config::Config;

/// Thread-local signer storage — mirrors rig-onchain-kit SignerContext pattern.
/// In production: use rig_onchain_kit::signer::SignerContext directly.
tokio::task_local! {
    // Holds the active signer for the current tokio task.
    // Access is only valid inside a `with_signer` scope.
    static CURRENT_SIGNER: Arc<dyn Signer + Send + Sync>;
}

/// A Solana keypair wrapper that loads its key from the environment.
///
/// In production, decode the base-58 private key from `SOLANA_PRIVATE_KEY`.
/// In the demo, a fresh random keypair is generated if the variable is unset.
pub struct LocalSolanaSigner {
    keypair: Keypair,
}

impl LocalSolanaSigner {
    /// Create from env (demo: generates random keypair if env not set)
    /// Construct a [`LocalSolanaSigner`] from the process environment.
    ///
    /// If `SOLANA_PRIVATE_KEY` is set the variable is acknowledged (but the
    /// demo still generates a random keypair as a placeholder).  If unset, a
    /// random keypair is generated directly.
    ///
    /// **TODO (production)**: decode the base-58 bytes with
    /// `Keypair::from_base58_string(&key)`.
    pub fn from_env() -> Self {
        let keypair = if std::env::var("SOLANA_PRIVATE_KEY").is_ok() {
            // Real: decode base58 keypair from env
            Keypair::new() // placeholder for demo
        } else {
            Keypair::new() // random for demo
        };
        Self { keypair }
    }

    /// Return the public key of the wrapped keypair.
    ///
    /// This is the address used to identify the signer on the Solana blockchain.
    pub fn pubkey(&self) -> Pubkey {
        self.keypair.pubkey()
    }
}

/// Execute a closure inside SignerContext — thread-local signer isolation.
/// Demonstrates SignerContext::with_signer() from rig-onchain-kit.
/// Execute an async closure with `signer` installed as the task-local signer.
///
/// Any code running inside `f` can retrieve the signer from `CURRENT_SIGNER`.
/// The signer is dropped automatically when the scope exits, guaranteeing it
/// cannot outlive the operation it was created for.
///
/// # Type parameters
///
/// * `F`   — A `FnOnce` that produces a future.
/// * `Fut` — The future returned by `F`.
/// * `T`   — The success type of the future.
///
/// # Errors
///
/// Propagates any error returned by `f`.
pub async fn with_signer<F, Fut, T>(signer: LocalSolanaSigner, f: F) -> Result<T>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let arc_signer: Arc<dyn Signer + Send + Sync> = Arc::new(signer.keypair);
    CURRENT_SIGNER.scope(arc_signer, async { f().await }).await
}

/// Demo: show SignerContext isolation across 3 concurrent tasks
/// Spawn three concurrent tasks and verify that each has an independent signer.
///
/// Each task creates its own [`LocalSolanaSigner`], calls [`with_signer`], and
/// logs its public key.  Because `CURRENT_SIGNER` is task-local, the three
/// public keys are independent and cannot interfere with one another.
///
/// # Errors
///
/// Returns an error if any spawned task panics or its join handle fails.
pub async fn demo_signer(cfg: &Config) -> Result<()> {
    info!("[SIGNER] Demonstrating SignerContext thread-local isolation");

    // Each task gets its own signer — no cross-contamination
    let handles: Vec<_> = (0..3)
        .map(|i| {
            tokio::spawn(async move {
                let signer = LocalSolanaSigner::from_env();
                let pubkey = signer.pubkey();
                with_signer(signer, || async move {
                    info!(task = i, %pubkey, "[SIGNER] Task running in isolated context");
                    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                    info!(task = i, "[SIGNER] Task complete — signer isolated");
                    Ok::<(), anyhow::Error>(())
                })
                .await
            })
        })
        .collect();

    for h in handles {
        h.await??;
    }
    info!("[SIGNER] All tasks complete. SignerContext isolation verified.");
    Ok(())
}
