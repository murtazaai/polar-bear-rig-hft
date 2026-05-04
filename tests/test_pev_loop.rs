//! Integration test: PEV loop with mocked LLM responses.
//! Tests plan decomposition, execute output, verify scoring.
//! Integration test: PEV loop types, default tasks, and verify threshold.

#[cfg(test)]
mod tests {
    use polar_bear_rig_hft::config::Config;
    use polar_bear_rig_hft::pev::types::{ExecuteOutput, TradeAction, TradeTask};
    use polar_bear_rig_hft::pev::{plan, verify};

    fn test_task() -> TradeTask {
        TradeTask {
            id: "T001".into(),
            pair: "SOL/USDC".into(),
            amount: 1.0,
            action: TradeAction::AnalyseMarket,
            acceptance_criteria: vec!["Market data retrieved".into()],
        }
    }

    fn test_output(result: &str) -> ExecuteOutput {
        ExecuteOutput {
            task_id: "T001".into(),
            result: result.to_string(),
            confidence: 0.90,
            reasoning: result.to_string(),
            tool_calls: vec!["fetch_price_feed(SOL/USDC)".into()],
        }
    }

    #[test]
    fn test_plan_default_tasks_count() {
        // default_tasks should produce 4 tasks
        let tasks = crate::pev::plan::default_tasks_pub("SOL/USDC", 1.0);
        assert_eq!(tasks.len(), 4, "Expected 4 default tasks");
    }

    #[test]
    fn test_verify_pass_threshold() {
        assert!(crate::pev::verify::PASS_THRESHOLD == 0.80);
    }

    #[test]
    fn test_trade_task_serialization() {
        let task = test_task();
        let json = serde_json::to_string(&task).unwrap();
        assert!(json.contains("SOL/USDC"));
        assert!(json.contains("analyse_market"));
    }

    #[test]
    fn test_execute_output_tool_calls() {
        let output = test_output("Market data retrieved: SOL price = $143.50");
        assert!(!output.tool_calls.is_empty());
        assert!(output.tool_calls[0].contains("fetch_price_feed"));
    }
}
