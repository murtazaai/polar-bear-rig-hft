//! Runtime configuration loaded from environment variables.
//!
//! All fields are read at startup via [`Config::from_env`]. The only required
//! variable is `ANTHROPIC_API_KEY`; the rest have safe defaults that keep the
//! binary in dry-run mode against Solana devnet.
//!
//! ## Environment variables
//!
//! | Variable | Required | Default |
//! |---|---|---|
//! | `ANTHROPIC_API_KEY` | ✅ | — |
//! | `SOLANA_RPC_URL` | ❌ | `https://api.devnet.solana.com` |
//! | `SOLANA_PRIVATE_KEY` | ❌ | `DEMO_KEY_PLACEHOLDER` |
//! | `DRY_RUN` | ❌ | `true` |
use anyhow::{Context, Result};

/// Global runtime configuration for the HFT platform.
///
/// Constructed once at startup and shared (by reference) across all subsystems.
/// Clone is cheap — all fields are `String` or `bool`.
#[derive(Debug, Clone)]
pub struct Config {
    /// Anthropic API key passed to every `rig-core` client.
    pub anthropic_api_key: String,

    /// Solana JSON-RPC endpoint (devnet by default).
    pub solana_rpc_url: String,

    /// Base-58 encoded Solana keypair used for signing transactions.
    ///
    /// In production this should be loaded from a secrets manager.
    /// The demo falls back to a randomly generated keypair if unset.
    pub solana_private_key: String, // base58 keypair for dry-run demo

    /// When `true` (the default), all on-chain operations are simulated and no
    /// real transactions are signed/broadcasted. Pass `--live` on the CLI to disable.
    pub dry_run: bool,
}

impl Config {
    /// Constructs a [`Config`] from the environment variables set at startup.
    ///
    /// Reads `.env` (via `dotenvy`) before this function is called in `main`.
    ///
    /// # Returns
    ///
    /// - `Ok(Self)` if all required variables are set
    /// - `Err(anyhow::Error)` if any required variable is missing
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            anthropic_api_key: std::env::var("ANTHROPIC_API_KEY")
                .context("ANTHROPIC_API_KEY not set")?,
            solana_rpc_url: std::env::var("SOLANA_RPC_URL")
                .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string()),
            solana_private_key: std::env::var("SOLANA_PRIVATE_KEY")
                .unwrap_or_else(|_| "DEMO_KEY_PLACEHOLDER".to_string()),
            dry_run: std::env::var("DRY_RUN")
                .map(|v| v == "true" || v == "1")
                .unwrap_or(true), // default: always dry-run
        })
    }
}
