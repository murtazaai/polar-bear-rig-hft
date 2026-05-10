//! PLAN phase - trade decomposition via a cheap LLM.
//!
//! Uses `claude-haiku-4-5` (the lowest-cost Anthropic model) to decompose a
//! high-level trade request into exactly four atomic [`types::TradeTask`]
//! objects, returned as a JSON array.
//!
//! When the LLM response cannot be parsed, [`default_tasks`] is used as a
//! deterministic fallback so the PEV pipeline always continues forward.
//!
//! ## Rig client trait requirements (rig-core ≥ 0.36)
//!
//! Calling `.agent()` on `anthropic::Client` requires **both** traits in scope:
//! - [`rig::client::CompletionClient`] - provides the `.agent()` builder method.
//! - [`rig::client::ProviderClient`] - required by the rig provider-client pattern;
//!   omitting either causes `E0599: no method named 'agent'` even though the type
//!   implements both traits.

use anyhow::Result;
use rig::{
    client::{CompletionClient, ProviderClient},
    completion::Prompt,
    providers::anthropic,
};
use tracing::{debug, info};

use super::types::TradeTask;
use crate::config::Config;

/// System preamble sent to the planning agent on every call.
///
/// Instructs the model to return **only** a JSON array of `TradeTask` objects
/// with no surrounding prose or Markdown fences.
const PLAN_PREAMBLE: &str = r"
You are the PLAN agent in a PEV (Plan-Execute-Verify) HFT pipeline.
Your role: decompose a trade task into a list of atomic sub-tasks.
Each sub-task must have: id, pair, amount, action, and acceptance_criteria.
Return ONLY a JSON array of TradeTask objects. No prose. No markdown.
Actions must be one of: analyse_market, select_route, validate_slippage,
simulate_execution.
";

/// Decompose a trade request into a [`Vec`] of [`TradeTask`] items.
///
/// Calls the Haiku model via `rig-core` with [`PLAN_PREAMBLE`] and a prompt
/// containing the pair and amount. The response is stripped of any accidental
/// Markdown fences before deserialisation. On JSON parse failure the function
/// falls back to [`default_tasks`] so the pipeline is never blocked.
///
/// # Arguments
///
/// * `cfg`    - Runtime config; provides the Anthropic API key.
/// * `pair`   - Trading pair string, e.g. `"SOL/USDC"`.
/// * `amount` - Base-token amount to trade.
///
/// # Errors
///
/// Returns `Err` if `anthropic::Client::new` fails or the LLM HTTP call fails
/// (network error, authentication failure, etc.).  A malformed JSON response is
/// handled internally by falling back to [`default_tasks`].
pub async fn decompose(cfg: &Config, pair: &str, amount: f64) -> Result<Vec<TradeTask>> {
    info!(pair, amount, "[PLAN] Decomposing trade task");

    // Client::new is fallible in rig-core 0.36+ - unwrap with `?`.
    // Haiku is chosen deliberately: cheapest model per the PEV cost model.
    let client = anthropic::Client::new(&cfg.anthropic_api_key)?;
    let planner = client
        .agent("claude-haiku-4-5")
        .preamble(PLAN_PREAMBLE)
        .build();

    let prompt = format!(
        "Decompose this HFT trade into atomic tasks: pair={pair}, amount={amount} SOL. \
         Return JSON array only."
    );

    let response = planner.prompt(&prompt).await?;
    debug!(raw = %response, "[PLAN] Raw LLM response");

    // Strip any accidental Markdown code fences before parsing.
    let cleaned = response
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let tasks: Vec<TradeTask> = serde_json::from_str(cleaned).unwrap_or_else(|_| {
        tracing::warn!("[PLAN] LLM response could not be parsed; using default tasks");
        default_tasks(pair, amount)
    });

    info!(count = tasks.len(), "[PLAN] Tasks decomposed");
    Ok(tasks)
}

/// Public alias for [`default_tasks`], exposed for integration tests.
///
/// Returns the canonical four-task breakdown for any pair and amount without
/// making any LLM network call, making it suitable for unit and integration
/// tests that do not require a live API key.
pub fn default_tasks_pub(pair: &str, amount: f64) -> Vec<TradeTask> {
    default_tasks(pair, amount)
}

/// Construct a deterministic four-task breakdown for the given pair and amount.
///
/// Called as a fallback when [`decompose`] cannot parse the LLM response, and
/// directly by tests via [`default_tasks_pub`]. The four tasks cover the
/// complete trade lifecycle: analyse → route → slippage check → execution.
fn default_tasks(pair: &str, amount: f64) -> Vec<TradeTask> {
    use crate::pev::types::TradeAction;
    vec![
        TradeTask {
            id: "T001".into(),
            pair: pair.to_string(),
            amount,
            action: TradeAction::AnalyseMarket,
            acceptance_criteria: vec!["Market data retrieved".into()],
        },
        TradeTask {
            id: "T002".into(),
            pair: pair.to_string(),
            amount,
            action: TradeAction::SelectRoute,
            acceptance_criteria: vec!["Best DEX venue selected".into()],
        },
        TradeTask {
            id: "T003".into(),
            pair: pair.to_string(),
            amount,
            action: TradeAction::ValidateSlippage,
            acceptance_criteria: vec!["Slippage within 0.5% tolerance".into()],
        },
        TradeTask {
            id: "T004".into(),
            pair: pair.to_string(),
            amount,
            action: TradeAction::SimulateExecution,
            acceptance_criteria: vec!["Dry-run swap simulation logged".into()],
        },
    ]
}
