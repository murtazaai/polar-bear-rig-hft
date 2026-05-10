//! Live Anthropic integration tests - require a real `ANTHROPIC_API_KEY`.
//!
//! These tests are marked `#[ignore]` and will **not** run in CI. Run them
//! manually when you have a key and want to verify live LLM connectivity:
//!
//! ```text
//! ANTHROPIC_API_KEY=sk-ant-... cargo test --test providers -- --ignored --test-threads=1
//! ```
//!
//! Use `--test-threads=1` to avoid concurrent API calls hitting rate limits.

use polar_bear_rig_hft::{config::Config, pev, sor};

/// Verify that the PEV plan phase successfully decomposes a trade via Haiku.
///
/// Checks that exactly four tasks are returned and that every task id follows
/// the `T00N` convention.
#[tokio::test]
#[ignore = "requires ANTHROPIC_API_KEY - run with: cargo test --test providers -- --ignored"]
async fn test_live_plan_decomposes_four_tasks() {
    dotenvy::dotenv().ok();
    let cfg = Config::from_env().expect("ANTHROPIC_API_KEY must be set");

    let tasks = pev::plan::decompose(&cfg, "SOL/USDC", 1.0)
        .await
        .expect("plan::decompose should not fail with a valid key");

    assert_eq!(tasks.len(), 4, "Haiku should return exactly 4 tasks");

    for task in &tasks {
        assert!(!task.id.is_empty(), "task id must not be empty");
        assert_eq!(task.pair, "SOL/USDC", "pair must be preserved");
        assert!(
            (task.amount - 1.0).abs() < f64::EPSILON,
            "amount must be preserved"
        );
    }
}

/// Verify that the full PEV loop passes (score ≥ 0.80) for a simple trade.
#[tokio::test]
#[ignore = "requires ANTHROPIC_API_KEY - run with: cargo test --test providers -- --ignored"]
async fn test_live_pev_loop_passes() {
    dotenvy::dotenv().ok();
    let cfg = Config::from_env().expect("ANTHROPIC_API_KEY must be set");

    let result = pev::run(&cfg, "SOL/USDC", 1.0)
        .await
        .expect("pev::run should not error with a valid key");

    assert!(
        result.passed,
        "PEV loop should pass: score={:.2}, feedback={}",
        result.verify_score, result.feedback
    );
    assert!(
        result.verify_score >= pev::verify::PASS_THRESHOLD,
        "verify_score must be >= {PASS}",
        PASS = pev::verify::PASS_THRESHOLD,
    );
}

/// Verify that SOR returns a real venue name (smoke test - no LLM needed).
#[tokio::test]
#[ignore = "requires ANTHROPIC_API_KEY - run with: cargo test --test providers -- --ignored"]
async fn test_live_sor_returns_known_venue() {
    let route = sor::best_route("SOL/USDC", 1.0)
        .await
        .expect("sor::best_route should never error");

    assert!(
        ["Raydium", "Orca", "Serum"].contains(&route.venue.as_str()),
        "Unexpected venue: {}",
        route.venue
    );
}
