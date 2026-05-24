# SpinBattles Rust Web3 Assessment

## About This Repository

This is a **technical assessment project** used by SpinBattles for evaluating Rust Web3 developer candidates. This code is intentionally simplified and is **not used in production**. It exists solely for fair, practical evaluation of technical skills.

## About SpinBattles

SpinBattles is a game development company expanding into Web3-related product areas. This assessment reflects real scenarios we work with: battle rewards, tournament systems, wallet authentication, and blockchain-based game economies.

Website: [www.spinbattles.com](https://www.spinbattles.com)

## Project Overview

A simplified **Battle Rewards Distribution System** built in Rust:

- Players earn SBR tokens by winning battles
- A Rust backend acts as the trusted off-chain authority
- It signs reward claims that a Solana program verifies on-chain
- The backend must be running for any track to function

## Repository Structure

```
spinbattles-rust-assessment/
├── game-server/      # Rust game server (battle results authority)
├── backend/          # Rust/Axum REST API (reward signer)
├── program/          # Solana/Anchor smart program
├── tasks/            # Role-specific assessment tasks (one per candidate track)
├── docs/             # Architecture and project documentation
└── frontend/         # Created by Fullstack candidates during assessment
```

## For Candidates

You will be assigned **ONE specific task** based on your role:

- `tasks/TASK_BACKEND.md` — Rust backend developers
- `tasks/TASK_SMART_CONTRACT.md` — Solana/Anchor program developers
- `tasks/TASK_SECURITY_REVIEW.md` — Security-focused developers
- `tasks/TASK_DEVOPS.md` — DevOps / infrastructure engineers
- `tasks/TASK_FULLSTACK.md` — Fullstack developers

Please only complete your assigned task. Other task files are included because we use the same repository for different candidate profiles.

**Expected Time:** 2-3 hours

## Submission Workflow

This repository uses a shared baseline on `main`. To keep evaluation fair for all candidates:

1. **Fork this repository** (required if you have read-only access) or use a branch named `candidate/<your-name>` on this repo if you were given write access.
2. Complete **only your assigned task** in your fork or branch.
3. Submit your work as a **pull request** to `main` or send a **ZIP** of your changes.
4. **Do not push directly to `main`.** The `main` branch is the official assessment baseline.

See `docs/ASSESSMENT_GUIDELINES.md` for the full checklist and FAQ.

Include a brief summary (3-5 sentences) and commands to test your changes.

## Quick Start

```bash
# Terminal 1 — Game Server
cd game-server
cargo run

# Terminal 2 — Backend
cd backend
cp .env.example .env
cargo run --bin keygen
# Paste BACKEND_SIGNER_PRIVATE_KEY into .env, then:
cargo run
```

Verify both are running:
```bash
curl http://localhost:8081/health
curl http://localhost:8080/health
curl http://localhost:8080/api/rewards/signer-pubkey
```

See `QUICK_START.md` for full setup instructions.

## Questions?

If anything is unclear about your assigned task, contact the technical team at tech@spinbattles.com.

## License

This assessment project is © SpinBattles. For evaluation purposes only.
