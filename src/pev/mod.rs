//! PEV Loop orchestration: Plan → Execute → Verify (max 2 retries on fail).

pub mod execute;
pub mod plan;
pub mod types;
pub mod verify;

use anyhow::Result;
use tracing::{info, warn};

use crate::config::Config;
use types::PEVResult;

pub const MAX_RETRIES: u32 = 2;

pub async fn run(cfg: &Config, pair: &str, amount: f64) -> Result<PEVResult> {
    info!(pair, amount, "╔══ PEV LOOP START ══╗");

    // ── PLAN ─────────────────────────────────────────────────────
    let tasks = plan::decompose(cfg, pair, amount).await?;
    info!(count = tasks.len(), "[PLAN] Complete");

    let mut outputs = vec![];
    let mut final_score = 0.0f64;
    let mut final_feedback = String::new();
    let mut total_retries = 0u32;

    for task in &tasks {
        let mut retries = 0u32;
        loop {
            // ── EXECUTE ──────────────────────────────────────────
            let output = execute::run_task(cfg, task).await?;

            // ── VERIFY ───────────────────────────────────────────
            let (score, feedback, passed) = verify::score(cfg, task, &output).await?;

            if passed {
                info!(task_id = %task.id, score, "[VERIFY] PASS");
                final_score = score;
                final_feedback = feedback;
                outputs.push(output);
                break;
            }

            retries += 1;
            total_retries += 1;
            if retries > MAX_RETRIES {
                warn!(task_id = %task.id, score, %feedback, "[VERIFY] FAIL — max retries reached");
                final_score = score;
                final_feedback = feedback.clone();
                outputs.push(output);
                break;
            }

            warn!(task_id = %task.id, score, retry = retries,
                  "[VERIFY] FAIL — retrying with error context injected");
        }
    }

    info!(
        final_score,
        retries = total_retries,
        "╚══ PEV LOOP COMPLETE ══╝"
    );
    Ok(PEVResult {
        tasks,
        outputs,
        verify_score: final_score,
        passed: final_score >= verify::PASS_THRESHOLD,
        feedback: final_feedback,
        retries: total_retries,
    })
}
