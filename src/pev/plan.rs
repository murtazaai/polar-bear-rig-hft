//! PLAN phase: decompose the trade into atomic tasks using a cheap LLM.
//! Uses Haiku-class model via rig-core for cost efficiency.
//! Output: structured JSON list of TradeTask items.
//! PLAN phase — trade decomposition via a cheap LLM.
//!
//! Uses `claude-haiku-4-5` (the lowest-cost Anthropic model) to decompose a
//! high-level trade request into exactly four atomic [`types::TradeTask`]
//! objects represented as a JSON array.
//!
//! If the LLM response cannot be parsed, [`default_tasks`] is used as a
//! deterministic fallback so the pipeline always continues.

use anyhow::Result;
use rig::{completion::Prompt, providers::anthropic};
use std::sync::Arc;
use tracing::{debug, info};

use super::types::TradeTask;
use crate::config::Config;

/// System preamble sent to the planning agent.
///
/// Instructs the model to return **only** a JSON array of `TradeTask` objects
/// with no surrounding prose or markdown.
const PLAN_PREAMBLE: &str = r#"
You are the PLAN agent in a PEV (Plan-Execute-Verify) HFT pipeline.
Your role: decompose a trade task into a list of atomic sub-tasks.
Each sub-task must have: id, pair, amount, action, and acceptance_criteria.
Return ONLY a JSON array of TradeTask objects. No prose. No markdown.
Actions must be one of: analyse_market, select_route, validate_slippage,
simulate_execution.
"#;

/// Decompose a trade request into a list of [`TradeTask`] items.
///
/// Calls the Haiku model via `rig-core` with [`PLAN_PREAMBLE`] and a prompt
/// that contains the pair and amount.  The response is stripped of any
/// accidental markdown fences before being deserialised.
///
/// Falls back to [`default_tasks`] on JSON parse failure to ensure the
/// pipeline is never blocked by a malformed LLM response.
///
/// # Arguments
///
/// * `cfg`    — Runtime config; provides the Anthropic API key.
/// * `pair`   — Trading pair string, e.g. `"SOL/USDC"`.
/// * `amount` — Base-token amount to trade.
///
/// # Errors
///
/// Returns an error if the LLM HTTP call itself fails.
pub async fn decompose(cfg: &Config, pair: &str, amount: f64) -> Result<Vec<TradeTask>> {
    info!(pair, amount, "[PLAN] Decomposing trade task");

    // Use claude-haiku-4 (cheap model) for planning — matches PEV cost model
    let client = Arc::new(anthropic::Client::new(&cfg.anthropic_api_key));
    let planner = client
        .agent("claude-haiku-4-5") // cheap model for planning
        .preamble(PLAN_PREAMBLE)
        .build();

    let prompt = format!(
        "Decompose this HFT trade into atomic tasks: pair={pair}, amount={amount} SOL. \
         Return JSON array only."
    );

    let response = planner.prompt(&prompt).await?;
    debug!(raw = %response, "[PLAN] Raw LLM response");

    // Strip any accidental markdown fences before parsing
    let cleaned = response
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let tasks: Vec<TradeTask> = serde_json::from_str(cleaned).unwrap_or_else(|_| {
        // Fallback: construct default tasks if LLM response is malformed
        tracing::warn!("[PLAN] LLM response could not be parsed; using default tasks");
        default_tasks(pair, amount)
    });

    info!(count = tasks.len(), "[PLAN] Tasks decomposed");
    Ok(tasks)
}

/// Public alias for [`default_tasks`], exposed for integration tests.
///
/// Produces the canonical four-task breakdown for any pair and amount without
/// making any LLM call.
pub fn default_tasks_pub(pair: &str, amount: f64) -> Vec<TradeTask> {
    default_tasks(pair, amount)
}

/// Construct a deterministic set of four [`TradeTask`] objects.
///
/// Used as a fallback when the LLM response cannot be parsed, and directly by
/// tests via [`default_tasks_pub`].  Covers the full trade lifecycle:
/// analyse → route → slippage → execute.
fn default_tasks(pair: &str, amount: f64) -> Vec<TradeTask> {
    vec![
        TradeTask {
            id: "T001".into(),
            pair: pair.to_string(),
            amount,
            action: crate::pev::types::TradeAction::AnalyseMarket,
            acceptance_criteria: vec!["Market data retrieved".into()],
        },
        TradeTask {
            id: "T002".into(),
            pair: pair.to_string(),
            amount,
            action: crate::pev::types::TradeAction::SelectRoute,
            acceptance_criteria: vec!["Best DEX venue selected".into()],
        },
        TradeTask {
            id: "T003".into(),
            pair: pair.to_string(),
            amount,
            action: crate::pev::types::TradeAction::ValidateSlippage,
            acceptance_criteria: vec!["Slippage within 0.5% tolerance".into()],
        },
        TradeTask {
            id: "T004".into(),
            pair: pair.to_string(),
            amount,
            action: crate::pev::types::TradeAction::SimulateExecution,
            acceptance_criteria: vec!["Dry-run swap simulation logged".into()],
        },
    ]
}
