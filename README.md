# polar-bear-rig-hft

**Optimal High-Frequency Trading Platform using Rig (Rust Inference Gateway / ARC)**

> Technology Lead: Murtaza Ali Imtiaz · Polar Bear Systems · July 2019–Present

A production-grade Rust implementation of an HFT agent framework powered by
[Rig (ARC)](https://rig.rs), the high-performance, enterprise-alternative to
Python LLM frameworks. Demonstrates **rig-core agent pipelines**, **ARC Virtual
Machine (AVM)** execution, **rig-onchain-kit** Solana/EVM operations, **Smart Order
Routing**, and **SignerContext** thread-local signer isolation with full **PEV loop**
(Plan-Execute-Verify) governance across the Agentic Web.

---

## Architecture

```
  CLI Entry (main.rs)
       │
  ┌────┴─────────────┐
  │   PEV Loop        │──── rig-core (claude-haiku plan, claude-sonnet execute)
  │   Plan → Execute  │
  │   → Verify        │
  └────┬─────────────┘
       │
  ┌────┴─────────────┐
  │   Smart Order     │──── Raydium │ Orca │ Serum (concurrent venue comparison)
  │   Routing (SOR)   │
  └────┬─────────────┘
       │
  ┌────┴─────────────┐
  │  rig-onchain-kit  │──── SignerContext │ Jupiter swap │ Balance query
  │  + SignerContext   │
  └────┬─────────────┘
       │
  ┌────┴─────────────┐
  │  AVM Layer        │──── JIT benchmark vs EVM │ Reactor GUI audit log
  └───────────────────┘
```

---

## Tech Stack

| Layer | Technology |
|---|---|
| AI Agent Framework | [rig-core](https://crates.io/crates/rig-core) (Rig / ARC) |
| On-chain Bridge | rig-onchain-kit (Solana + EVM) |
| Async Runtime | Tokio |
| Blockchain | Solana (devnet) |
| Cryptography | ed25519-dalek, k256 (ECDSA) |
| CLI | clap |
| Logging | tracing + tracing-subscriber |

---

## Build

```bash
# Prerequisites: Rust stable (rustup.rs)
git clone https://github.com/murtazaai/polar-bear-rig-hft
cd polar-bear-rig-hft
cp .env.example .env
# Add your ANTHROPIC_API_KEY to .env

# Build (release)
cargo build --release

# Run full pipeline (dry-run by default)
cargo run -- --mode full --pair SOL/USDC --amount 1.0

# Run individual modes
cargo run -- --mode pev
cargo run -- --mode sor
cargo run -- --mode signer
cargo run -- --mode reactor

# Tests
cargo test
cargo clippy -- -D warnings
```

---

## Related

- [Star Story](./docs/star_story.md)
- [High-Level Architecture Diagram](./docs/architecture.md)
- [Key Outputs (screen capture)](./docs/screen_capture_guide.md)

- [Rig Framework](https://rig.rs) · [0xPlaygrounds](https://github.com/0xPlaygrounds/rig)
- [arc.fun](https://arc.fun) · [Ryzome](https://ryzome.ai)
- [Solana Program Library](https://spl.solana.com/)
- [Jupiter](https://jup.ag/) · [Raydium](https://raydium.io/)

---

## 📝 License

PBS License: [PBS License](./LICENSE-PBS)

---

## 👤 Author

**Murtaza Ali Imtiaz**

- LinkedIn: [LinkedIn](https://linkedin.com/in/murtazai)
- GitHub: [@murtazaai](https://github.com/murtazaai)
- Portfolio: [murtazai.com](https://murtazai.com)
