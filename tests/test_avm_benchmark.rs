//! Integration tests for the AVM benchmark module.

#[cfg(test)]
mod tests {
    /// Benchmark must complete without panicking and log a positive speedup.
    #[test]
    fn test_benchmark_runs_without_error() {
        let result = polar_bear_rig_hft::avm::run_benchmark();
        assert!(
            result.is_ok(),
            "AVM benchmark should not fail: {:?}",
            result
        );
    }
}
