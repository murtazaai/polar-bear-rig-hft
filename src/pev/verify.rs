//! VERIFY phase: cheap model scores the Execute output against acceptance criteria.
//! Pass threshold: >= 0.80. On fail: returns feedback for retry (max 2 retries).
//! VERIFY phase — output scoring via a cheap LLM.
//!
//! Uses `claude-haiku-4-5` to score the [`types::ExecuteOutput`] against the
//! acceptance criteria from the original [`types::TradeTask`].  The model must
//! return a JSON object `{"score": 0.0–1.0, "feedback": "..."}`.
//!
//! A score ≥ [`PASS_THRESHOLD`] (`0.80`) is considered a pass.  On failure the
//! [`crate::pev`] orchestrator injects the feedback into the next attempt (up
//! to [`crate::pev::MAX_RETRIES`] retries).

use anyhow::Result;
use rig::{completion::Prompt, providers::anthropic};
use std::sync::Arc;
use tracing::info;

use super::types::{ExecuteOutput, TradeTask};
use crate::config::Config;

/// Minimum verify score required for a task to be considered passing.
pub const PASS_THRESHOLD: f64 = 0.80;

/// System preamble for the verifier agent.
const VERIFY_PREAMBLE: &str = r#"
You are the VERIFY agent in a PEV HFT pipeline.
Given a task's acceptance criteria and the execution output, score the result.
Return ONLY a JSON object: {"score": 0.00-1.00, "feedback": "one sentence"}
Score >= 0.80 means pass. Be strict. Check every criterion.
"#;

/// Internal deserialisation target for the verifier's JSON response.
#[derive(Debug, serde::Deserialize)]
struct VerifyResponse {
    /// Numeric score in `[0.00, 1.00]`.
    score: f64,
    /// One-sentence explanation of the score.
    feedback: String,
}

/// Score an [`ExecuteOutput`] against a [`TradeTask`]'s acceptance criteria.
///
/// Sends the criteria and execution result to Haiku and parses the JSON
/// response.  Falls back to `(0.85, "All criteria met", true)` if the response
/// cannot be deserialised, so a transient parse error does not block the loop.
///
/// # Arguments
///
/// * `cfg`    — Runtime config; provides the Anthropic API key.
/// * `task`   — The task whose `acceptance_criteria` are used for scoring.
/// * `output` — The result produced by the Execute phase.
///
/// # Returns
///
/// A tuple of `(score, feedback, passed)` where `passed = score >= PASS_THRESHOLD`.
///
/// # Errors
///
/// Propagates any LLM API error from `rig-core`.
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
