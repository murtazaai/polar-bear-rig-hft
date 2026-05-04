//! EXECUTE phase: rig-core Sonnet agent with tool calls processes each TradeTask.
//! Demonstrates multi-step agentic workflow with tool use + LLM reasoning.
//! EXECUTE phase — agentic task execution via Sonnet.
//!
//! Each [`types::TradeTask`] is handed to a `claude-sonnet-4-6` agent that
//! reasons step-by-step and invokes the appropriate tool.  Tool calls are
//! simulated in this demo; in production they would be real `rig-core`
//! [`rig::tool::Tool`] implementations backed by live market APIs.
//!
//! Sonnet is the most capable model in the PEV pipeline and is deliberately
//! kept to the Execute phase only — Plan and Verify use Haiku to minimise cost.

use anyhow::Result;
use rig::{completion::Prompt, providers::anthropic};
use std::sync::Arc;
use tracing::{debug, info};

use super::types::{ExecuteOutput, TradeAction, TradeTask};
use crate::config::Config;

/// System preamble for the executor agent.
///
/// Instructs Sonnet to think step-by-step, call a tool, and return a concise
/// result string that the Verify phase can score against acceptance criteria.
const EXECUTE_PREAMBLE: &str = r#"
You are the EXECUTE agent in a PEV HFT pipeline running on Rig (ARC).
You receive a single TradeTask and must complete it using available tools.
Think step-by-step. Call the appropriate tool. Return a concise result string.
"#;

/// Execute a single [`TradeTask`] using the Sonnet agent.
///
/// Builds a structured prompt from the task fields, sends it to
/// `claude-sonnet-4-6`, then maps the [`TradeAction`] to a list of simulated
/// tool call names for the audit log.
///
/// # Arguments
///
/// * `cfg`  — Runtime config; provides the Anthropic API key.
/// * `task` — The atomic task to execute.
///
/// # Errors
///
/// Propagates any LLM API error from `rig-core`.
pub async fn run_task(cfg: &Config, task: &TradeTask) -> Result<ExecuteOutput> {
    info!(task_id = %task.id, action = ?task.action, "[EXECUTE] Running task");

    // Sonnet = capable model with full tool-calling + reasoning
    let client = Arc::new(anthropic::Client::new(&cfg.anthropic_api_key));
    let executor = client
        .agent("claude-sonnet-4-6") // capable model for execution
        .preamble(EXECUTE_PREAMBLE)
        .build();

    let prompt = format!(
        "Task ID: {}\nAction: {:?}\nPair: {}\nAmount: {} SOL\n \
         Acceptance criteria: {:?}\nExecute this task now.",
        task.id, task.action, task.pair, task.amount, task.acceptance_criteria
    );

    let response = executor.prompt(&prompt).await?;
    debug!(raw = %response, task_id = %task.id, "[EXECUTE] Raw response");

    // Simulate tool call log based on task action
    let tool_calls = match task.action {
        TradeAction::AnalyseMarket => vec!["fetch_price_feed(SOL/USDC)".into()],
        TradeAction::SelectRoute => vec!["query_raydium_pool()".into(), "query_orca_pool()".into()],
        TradeAction::ValidateSlippage => vec!["calculate_slippage(amount)".into()],
        TradeAction::SimulateExecution => vec!["jupiter_swap_dry_run()".into()],
    };

    for tool in &tool_calls {
        info!(tool = %tool, "[EXECUTE] Tool called");
    }

    Ok(ExecuteOutput {
        task_id: task.id.clone(),
        result: response.clone(),
        confidence: 0.87, // fixed in demo; real: parse from LLM or compute
        reasoning: response,
        tool_calls,
    })
}
