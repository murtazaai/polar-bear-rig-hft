//! Smart Order Routing: compare Raydium, Orca, Serum — select best venue.
//! Logs: venue name, effective price, fee bps, routing latency in ms.

use anyhow::Result;
use std::time::Instant;
use tracing::info;

#[derive(Debug, Clone)]
pub struct Route {
    pub venue: String,
    pub effective_price: f64,
    pub fee_bps: u16,
    pub price_impact_pct: f64,
    pub latency_ms: u128,
}

/// Query all three venues concurrently and return the best (lowest effective cost).
pub async fn best_route(pair: &str, amount: f64) -> Result<Route> {
    info!(pair, amount, "[SOR] Starting route comparison");
    let t0 = Instant::now();

    // Concurrent venue queries (mock data — swap for real API calls)
    let (raydium, orca, serum) = tokio::join!(
        query_raydium(pair, amount),
        query_orca(pair, amount),
        query_serum(pair, amount),
    );

    let mut candidates = vec![];
    if let Ok(r) = raydium {
        candidates.push(r);
    }
    if let Ok(r) = orca {
        candidates.push(r);
    }
    if let Ok(r) = serum {
        candidates.push(r);
    }

    // Sort by lowest effective cost (price × (1 + fee_bps/10000))
    candidates.sort_by(|a, b| {
        let cost_a = a.effective_price * (1.0 + a.fee_bps as f64 / 10_000.0);
        let cost_b = b.effective_price * (1.0 + b.fee_bps as f64 / 10_000.0);
        cost_a.partial_cmp(&cost_b).unwrap()
    });

    let mut best = candidates
        .into_iter()
        .next()
        .unwrap_or_else(|| fallback_route(pair));
    best.latency_ms = t0.elapsed().as_millis();

    info!(
        venue = %best.venue,
        price = best.effective_price,
        fee_bps = best.fee_bps,
        latency_ms = best.latency_ms,
        "[SOR] Best route selected"
    );
    Ok(best)
}

async fn query_raydium(pair: &str, amount: f64) -> Result<Route> {
    // Mock: replace with Raydium SDK / REST call in production
    tokio::time::sleep(std::time::Duration::from_millis(12)).await;
    Ok(Route {
        venue: "Raydium".into(),
        effective_price: 143.52,
        fee_bps: 25,
        price_impact_pct: 0.03,
        latency_ms: 0,
    })
}

async fn query_orca(pair: &str, amount: f64) -> Result<Route> {
    tokio::time::sleep(std::time::Duration::from_millis(9)).await;
    Ok(Route {
        venue: "Orca".into(),
        effective_price: 143.48,
        fee_bps: 30,
        price_impact_pct: 0.02,
        latency_ms: 0,
    })
}

async fn query_serum(pair: &str, amount: f64) -> Result<Route> {
    tokio::time::sleep(std::time::Duration::from_millis(15)).await;
    Ok(Route {
        venue: "Serum".into(),
        effective_price: 143.61,
        fee_bps: 20,
        price_impact_pct: 0.05,
        latency_ms: 0,
    })
}

fn fallback_route(pair: &str) -> Route {
    Route {
        venue: "Raydium-fallback".into(),
        effective_price: 143.50,
        fee_bps: 25,
        price_impact_pct: 0.03,
        latency_ms: 0,
    }
}
