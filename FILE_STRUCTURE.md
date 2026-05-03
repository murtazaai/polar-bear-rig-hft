polar-bear-rig-hft/
├── Cargo.toml                   # workspace + all dependencies
├── Cargo.lock
├── README.md                    # architecture + STAR story + build instructions
├── .env.example                 # ANTHROPIC_API_KEY, SOLANA_RPC_URL, etc.
├── src/
│   ├── main.rs                  # CLI entry: mode selector
│   ├── pev/
│   │   ├── mod.rs               # PEV loop orchestration
│   │   ├── plan.rs              # Plan phase: cheap model task decomposition
│   │   ├── execute.rs           # Execute phase: rig-core tool-calling agent
│   │   ├── verify.rs            # Verify phase: score output 0.00–1.00
│   │   └── types.rs             # TradeTask, PEVResult, VerifyScore
│   ├── sor/
│   │   ├── mod.rs               # Smart Order Routing engine
│   │   ├── raydium.rs           # Raydium price/fee lookup (mock + real)
│   │   ├── orca.rs              # Orca price/fee lookup (mock + real)
│   │   ├── serum.rs             # Serum orderbook spread lookup
│   │   └── router.rs            # Route comparator: select best venue
│   ├── onchain/
│   │   ├── mod.rs               # rig-onchain-kit integration
│   │   ├── signer.rs            # SignerContext + LocalSolanaSigner
│   │   ├── jupiter.rs           # Jupiter swap dry-run
│   │   ├── balance.rs           # SOL + SPL token balance query
│   │   └── types.rs             # SwapParams, BalanceResult
│   ├── avm/
│   │   ├── mod.rs               # AVM execution layer simulation
│   │   ├── benchmark.rs         # AVM vs EVM execution time comparison
│   │   └── reactor.rs           # Reactor GUI audit log simulation
│   └── config.rs                # Config loader from .env
├── tests/
│   ├── test_pev_loop.rs
│   ├── test_sor.rs
│   ├── test_signer_context.rs
│   └── test_avm_benchmark.rs
└── docs/
    ├── architecture.md
    ├── star_story.md            # Interview evidence STAR narrative
    └── screen_capture_guide.md  # What to record for the demo video
