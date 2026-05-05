//! Runtime configuration loaded from environment variables.
//!
//! All fields are read at startup via [`Config::from_env`]. The only required
//! variable is `ANTHROPIC_API_KEY`; every other variable has a safe default
//! that keeps the binary in dry-run mode against Solana devnet.
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
/// Constructed once at startup and shared by reference across all subsystems.
/// Cloning is cheap — all fields are either `String` or `bool`.
#[derive(Debug, Clone)]
pub struct Config {
    /// Anthropic API key forwarded to every `rig-core` client.
    pub anthropic_api_key: String,

    /// Solana JSON-RPC endpoint. Defaults to Solana devnet.
    pub solana_rpc_url: String,

    /// Base-58 encoded Solana keypair used for signing transactions.
    ///
    /// In production this should be loaded from a secrets manager rather than
    /// an environment variable. The demo falls back to a freshly generated
    /// random keypair when this variable is absent.
    pub solana_private_key: String,

    /// When `true` (the default), all on-chain operations are simulated and no
    /// real transactions are signed or broadcast.
    ///
    /// Pass `--live` on the CLI to set this to `false`.
    pub dry_run: bool,
}

impl Config {
    /// Construct a [`Config`] from the process environment.
    ///
    /// Call [`dotenvy::dotenv`] before this function to load variables from a
    /// `.env` file; `main` does this automatically.
    ///
    /// # Errors
    ///
    /// Returns `Err` if `ANTHROPIC_API_KEY` is not present in the environment.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use polar_bear_rig_hft::config::Config;
    ///
    /// let cfg = Config::from_env().expect("ANTHROPIC_API_KEY must be set");
    /// ```
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
                .unwrap_or(true),
        })
    }
}
