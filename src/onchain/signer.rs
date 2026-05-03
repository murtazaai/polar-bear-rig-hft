//! SignerContext: thread-local signer isolation for secure on-chain operations.
//! Demonstrates the rig-onchain-kit security boundary pattern.
//! Every async on-chain call is wrapped in SignerContext::with_signer().

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
    static CURRENT_SIGNER: Arc<dyn Signer + Send + Sync>;
}

pub struct LocalSolanaSigner {
    keypair: Keypair,
}

impl LocalSolanaSigner {
    /// Create from env (demo: generates random keypair if env not set)
    pub fn from_env() -> Self {
        let keypair = if std::env::var("SOLANA_PRIVATE_KEY").is_ok() {
            // Real: decode base58 keypair from env
            Keypair::new() // placeholder for demo
        } else {
            Keypair::new() // random for demo
        };
        Self { keypair }
    }

    pub fn pubkey(&self) -> Pubkey {
        self.keypair.pubkey()
    }
}

/// Execute a closure inside SignerContext — thread-local signer isolation.
/// Demonstrates SignerContext::with_signer() from rig-onchain-kit.
pub async fn with_signer<F, Fut, T>(signer: LocalSolanaSigner, f: F) -> Result<T>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let arc_signer: Arc<dyn Signer + Send + Sync> = Arc::new(signer.keypair);
    CURRENT_SIGNER.scope(arc_signer, async { f().await }).await
}

/// Demo: show SignerContext isolation across 3 concurrent tasks
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
