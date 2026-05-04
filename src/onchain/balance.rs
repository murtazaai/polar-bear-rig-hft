//! On-chain balance queries — stub for future implementation.
//!
//! In production this module will provide helpers such as:
//!
//! ```rust,ignore
//! pub async fn sol_balance(rpc: &RpcClient, pubkey: &Pubkey) -> Result<u64>;
//! pub async fn token_balance(rpc: &RpcClient, ata: &Pubkey) -> Result<u64>;
//! ```
//!
//! These will be called before and after each swap to confirm the on-chain
//! state matches the [`crate::onchain::jupiter::SwapResult`] figures.
