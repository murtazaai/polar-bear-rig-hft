use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct Config {
    pub anthropic_api_key: String,
    pub solana_rpc_url: String,
    pub solana_private_key: String, // base58 keypair for dry-run demo
    pub dry_run: bool,
}

impl Config {
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
