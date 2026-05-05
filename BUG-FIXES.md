# Bug Fixes

1. Fix: Removed all the features from rig-core depdendency in Cargo.toml. 
   Root Cause: The features = ["anthropic", "openai", "cohere"] lines in Cargo.toml were fictitious and cargo rejected them.
2. Fix: Upgraded the three Solana crates to 3.x, that was the series where Solana migrated to ed25519-dalek ^2.1.1 (via the new solana-keypair sub-crate). solana-sdk 3.x still re-exported Pubkey, Keypair, and Signer from the same paths, so the signer.rs compiled unchanged. spl-token needed a matching bump to 9.x (the version aligned to solana 3.x).
   Root Cause: solana-sdk 1.18 hard-pinned ed25519-dalek = "=1.0.1" (exact version, v1). The project directly required ed25519-dalek = "^2". Cargo could not unify a v1 exact pin with a v2 requirement, they were different semver epochs and incompatible. Every version in the ^1.18 range had this same pin, so no 1.18.x release could ever resolve.
3. Fix: Deleted Cargo.lock. Cargo regenerated it cleanly on the next cargo build.
   Root Cause: The locked graph pinned the old rig-core 0.9.1 and solana-sdk 1.18 resolutions. 
4. Fix: All crate:: references replaced with polar_bear_rig_hft::. Integration tests are external crates; crate:: in them refered to the test crate itself, not the project being tested.
   Files: tests/test_pev_loop.rs, test_sor.rs, & test_signer_context.rs
5. Fix: format! had the string "Acceptance criteria: {:?}\nExecute this task now." as the first positional arg (filling in Task ID: {}), then 5 more args for 4 placeholders: compile error argument never used. Reordered to build the prompt correctly.
   File: src/pev/execute.rs
6. Fix: Removed unused rig::tool::Tool import.
   File: src/pev/execute.rs
7. Fix: format!("…{pair}…{amount}…", "Return JSON array only."), the second string literal was being passed as a positional argument with no {} to land in: compiled error argument never used. Merged into one string.
   File: src/pev/plan.rs
8. Fix: default_tasks was private; added pub fn default_tasks alias that the integration test calls.
   File: src/pev/plan.rs
7. Fix: Added pub mod {orca,raydium,router,serum} and pub use router::best_route. main.rs calls sor::best_route(...) 
   File: src/sor/mod.rs
8. Fix: Replaced mod avm; mod config; … with use polar_bear_rig_hft::{avm, config, onchain, pev, sor};. The binary now re-used the lib's compiled modules instead of redeclaring them.
   File: src/main.rs
9. Fix: Added a library root that re-exports all five modules (avm, config, onchain, pev, sor). Integration tests in tests/ compile as a separate crate, they count not use crate:: to reach into a [[bin]]. Adding a lib target made polar_bear_rig_hft:: the correct prefix.
   File: New file src/lib.rs
10. Fix: Used rig-core ^0.36. the providers::anthropic API used in the code matched the 0.36 surface
    Root Cause: rig-core 0.9.1 was stale; 
11. Fix: Used reqwest ^0.13
    Root Cause: rig-core 0.36 transitively required reqwest ^0.13; mismatching majors caused two copies and potential API collisions
12. Fix: Changed the reqwest feature to rustls. rustls is also pulled in automatically via the default feature set (default → default-tls → rustls), so never needed to be explicit, we can just write features = ["json"] and TLS comes for free. Keept it explicit as "rustls" made the intent clear.
    Root Cause: `reqwest` with feature `rustls-tls` but `reqwest` does not have that feature.  In reqwest 0.13 the feature was renamed: rustls-tls → rustls.
13. Rust Doc comments added.
14. Fix: Rust Doc Comments Syntax and Semantics corrected.
15. Fix: The Arc was also unnecessary, client consumed immediately by .agent().preamble().build() in the very next line and never shared across tasks, so removed it has no effect on behaviour. The use std::sync::Arc; import was also removed from all three files to keep them warning-free.
    Root Cause: In rig-core 0.36, anthropic::Client::new(&key) was made fallible, it now returns Result<Client<AnthropicExt>, Error> rather than a bare Client. The code was wrapping the call in Arc::new(...), which produced Arc<Result<…>>. Rust's method resolution couldn't find .agent() on that type, hence the E0599.
16. To be continued.
