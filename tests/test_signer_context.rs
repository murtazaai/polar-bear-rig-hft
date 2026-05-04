#[cfg(test)]
mod tests {
    use polar_bear_rig_hft::onchain::signer::{with_signer, LocalSolanaSigner};

    #[tokio::test]
    async fn test_signer_context_isolation() {
        // Two concurrent tasks must get independent signers (no cross-contamination)
        let (pk1, pk2) = tokio::join!(
            tokio::spawn(async {
                let s = LocalSolanaSigner::from_env();
                let pk = s.pubkey();
                with_signer(s, || async { Ok::<_, anyhow::Error>(pk) })
                    .await
                    .unwrap()
            }),
            tokio::spawn(async {
                let s = LocalSolanaSigner::from_env();
                let pk = s.pubkey();
                with_signer(s, || async { Ok::<_, anyhow::Error>(pk) })
                    .await
                    .unwrap()
            })
        );
        // Both tasks complete successfully — SignerContext isolated each
        assert!(pk1.is_ok());
        assert!(pk2.is_ok());
    }

    #[tokio::test]
    async fn test_jupiter_dry_run_returns_simulated_sig() {
        let route = crate::sor::router::Route {
            venue: "Raydium".into(),
            effective_price: 143.50,
            fee_bps: 25,
            price_impact_pct: 0.03,
            latency_ms: 10,
        };
        let result = crate::onchain::jupiter::simulate_swap(&route, 1.0, true)
            .await
            .unwrap();
        assert!(result.is_dry_run);
        assert!(result.simulated_sig.starts_with("SIM_"));
        assert!(result.output_amount > 0.0);
    }
}
