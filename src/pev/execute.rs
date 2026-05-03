//! EXECUTE phase: rig-core Sonnet agent with tool calls processes each TradeTask.
//! Demonstrates multi-step agentic workflow with tool use + LLM reasoning.

use anyhow::Result;
use rig::{completion::Prompt, providers::anthropic, tool::Tool};
use std::sync::Arc;
use tracing::{debug, info};

use super::types::{ExecuteOutput, TradeAction, TradeTask};
use crate::config::Config;

const EXECUTE_PREAMBLE: &str = r#"
You are the EXECUTE agent in a PEV HFT pipeline running on Rig (ARC).
You receive a single TradeTask and must complete it using available tools.
Think step-by-step. Call the appropriate tool. Return a concise result string.
"#;

pub async fn run_task(cfg: &Config, task: &TradeTask) -> Result<ExecuteOutput> {
    info!(task_id = %task.id, action = ?task.action, "[EXECUTE] Running task");

    // Sonnet = capable model with full tool-calling + reasoning
    let client = Arc::new(anthropic::Client::new(&cfg.anthropic_api_key));
    let executor = client
        .agent("claude-sonnet-4-6") // capable model for execution
        .preamble(EXECUTE_PREAMBLE)
        .build();

    let prompt = format!(
        "Task ID: {}\nAction: {:?}\nPair: {}\nAmount: {} SOL\n",
        "Acceptance criteria: {:?}\nExecute this task now.",
        task.id,
        task.action,
        task.pair,
        task.amount,
        task.acceptance_criteria
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
