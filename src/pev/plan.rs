//! PLAN phase: decompose the trade into atomic tasks using a cheap LLM.
//! Uses Haiku-class model via rig-core for cost efficiency.
//! Output: structured JSON list of TradeTask items.

use anyhow::Result;
use rig::{completion::Prompt, providers::anthropic};
use std::sync::Arc;
use tracing::{debug, info};

use super::types::TradeTask;
use crate::config::Config;

const PLAN_PREAMBLE: &str = r#"
You are the PLAN agent in a PEV (Plan-Execute-Verify) HFT pipeline.
Your role: decompose a trade task into a list of atomic sub-tasks.
Each sub-task must have: id, pair, amount, action, and acceptance_criteria.
Return ONLY a JSON array of TradeTask objects. No prose. No markdown.
Actions must be one of: analyse_market, select_route, validate_slippage,
simulate_execution.
"#;

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

pub fn default_tasks(pair: &str, amount: f64) -> Vec<TradeTask> {
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
