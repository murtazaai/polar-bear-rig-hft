use serde::{Deserialize, Serialize};

/// A single atomic trade task produced by the Plan phase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeTask {
    pub id: String,
    pub pair: String,
    pub amount: f64,
    pub action: TradeAction,
    pub acceptance_criteria: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TradeAction {
    AnalyseMarket,
    SelectRoute,
    ValidateSlippage,
    SimulateExecution,
}

/// Output produced by the Execute phase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteOutput {
    pub task_id: String,
    pub result: String,
    pub confidence: f64,
    pub reasoning: String,
    pub tool_calls: Vec<String>,
}

/// Final result of a full PEV loop iteration
#[derive(Debug, Clone)]
pub struct PEVResult {
    pub tasks: Vec<TradeTask>,
    pub outputs: Vec<ExecuteOutput>,
    pub verify_score: f64, // 0.00 – 1.00; pass threshold = 0.80
    pub passed: bool,
    pub feedback: String,
    pub retries: u32,
}
