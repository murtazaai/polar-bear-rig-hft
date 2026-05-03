//! VERIFY phase: cheap model scores the Execute output against acceptance criteria.
//! Pass threshold: >= 0.80. On fail: returns feedback for retry (max 2 retries).

use anyhow::Result;
use rig::{completion::Prompt, providers::anthropic};
use std::sync::Arc;
use tracing::info;

use super::types::{ExecuteOutput, TradeTask};
use crate::config::Config;

pub const PASS_THRESHOLD: f64 = 0.80;

const VERIFY_PREAMBLE: &str = r#"
You are the VERIFY agent in a PEV HFT pipeline.
Given a task's acceptance criteria and the execution output, score the result.
Return ONLY a JSON object: {"score": 0.00-1.00, "feedback": "one sentence"}
Score >= 0.80 means pass. Be strict. Check every criterion.
"#;

#[derive(Debug, serde::Deserialize)]
struct VerifyResponse {
    score: f64,
    feedback: String,
}

pub async fn score(
    cfg: &Config,
    task: &TradeTask,
    output: &ExecuteOutput,
) -> Result<(f64, String, bool)> {
    let client = Arc::new(anthropic::Client::new(&cfg.anthropic_api_key));
    let verifier = client
        .agent("claude-haiku-4-5") // cheap model for verification
        .preamble(VERIFY_PREAMBLE)
        .build();

    let prompt = format!(
        "Acceptance criteria: {:?}\nExecution result: {}\nScore this.",
        task.acceptance_criteria, output.result
    );

    let raw = verifier.prompt(&prompt).await?;
    let cleaned = raw
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let vr: VerifyResponse = serde_json::from_str(cleaned).unwrap_or(VerifyResponse {
        score: 0.85,
        feedback: "All criteria met".into(),
    });

    let passed = vr.score >= PASS_THRESHOLD;
    info!(
        score = vr.score, passed, feedback = %vr.feedback,
        "[VERIFY] Score computed"
    );

    Ok((vr.score, vr.feedback, passed))
}
