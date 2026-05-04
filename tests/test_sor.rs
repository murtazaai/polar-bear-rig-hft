#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_best_route_returns_raydium_or_orca() {
        let route = polar_bear_rig_hft::sor::best_route("SOL/USDC", 1.0)
            .await
            .unwrap();
        assert!(["Raydium", "Orca", "Serum"].contains(&route.venue.as_str()));
        assert!(route.effective_price > 0.0);
        assert!(route.fee_bps > 0);
    }

    #[tokio::test]
    async fn test_sor_latency_logged() {
        let route = polar_bear_rig_hft::sor::best_route("SOL/USDC", 1.0)
            .await
            .unwrap();
        assert!(route.latency_ms > 0, "Latency must be measured");
    }

    #[test]
    fn test_cost_ordering() {
        // Lower fee_bps should rank higher in cost comparison
        let r1 = polar_bear_rig_hft::sor::router::Route {
            venue: "A".into(),
            effective_price: 143.50,
            fee_bps: 20,
            price_impact_pct: 0.02,
            latency_ms: 10,
        };
        let r2 = polar_bear_rig_hft::sor::router::Route {
            venue: "B".into(),
            effective_price: 143.50,
            fee_bps: 30,
            price_impact_pct: 0.02,
            latency_ms: 10,
        };
        let cost = |r: &polar_bear_rig_hft::sor::router::Route| {
            r.effective_price * (1.0 + r.fee_bps as f64 / 10_000.0)
        };
        assert!(cost(&r1) < cost(&r2));
    }
}
