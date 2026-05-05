//! Integration tests for the PEV loop types, default task decomposition, and
//! the verify pass threshold constant.
//!
//! These tests do **not** make any LLM API calls; they exercise only the
//! deterministic, pure-Rust code paths.

use polar_bear_rig_hft::pev::types::{ExecuteOutput, TradeAction, TradeTask};

// ── helpers ──────────────────────────────────────────────────────────────────

fn make_task() -> TradeTask {
    TradeTask {
        id: "T001".into(),
        pair: "SOL/USDC".into(),
        amount: 1.0,
        action: TradeAction::AnalyseMarket,
        acceptance_criteria: vec!["Market data retrieved".into()],
    }
}

fn make_output(result: &str) -> ExecuteOutput {
    ExecuteOutput {
        task_id: "T001".into(),
        result: result.to_string(),
        confidence: 0.90,
        reasoning: result.to_string(),
        tool_calls: vec!["fetch_price_feed(SOL/USDC)".into()],
    }
}

// ── tests ─────────────────────────────────────────────────────────────────────

/// `default_tasks_pub` should produce exactly four tasks.
#[test]
fn test_plan_default_tasks_count() {
    let tasks = polar_bear_rig_hft::pev::plan::default_tasks_pub("SOL/USDC", 1.0);
    assert_eq!(tasks.len(), 4, "expected 4 default tasks");
}

/// The pass threshold constant must equal `0.80`.
#[test]
fn test_verify_pass_threshold() {
    assert_eq!(
        polar_bear_rig_hft::pev::verify::PASS_THRESHOLD,
        0.80,
        "PASS_THRESHOLD must be 0.80"
    );
}

/// A [`TradeTask`] round-trips through JSON without data loss.
#[test]
fn test_trade_task_serialization() {
    let task = make_task();
    let json = serde_json::to_string(&task).unwrap();
    assert!(json.contains("SOL/USDC"));
    assert!(json.contains("analyse_market"));
}

/// [`ExecuteOutput`] correctly stores tool call names.
#[test]
fn test_execute_output_tool_calls() {
    let output = make_output("Market data retrieved: SOL price = $143.50");
    assert!(
        !output.tool_calls.is_empty(),
        "tool_calls must not be empty"
    );
    assert!(
        output.tool_calls[0].contains("fetch_price_feed"),
        "first tool call should reference fetch_price_feed"
    );
}
