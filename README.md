# SpinBattles Rust Solana Assessment

## About

This is a technical assessment for Rust Solana developers at SpinBattles. You'll work with a simplified battle rewards system built with Rust backend, Solana smart contracts, and game server integration.

**Time estimate:** 2-3 hours

## Prerequisites

- Rust 1.70+
- Node.js 16+ (smart contract track only)
- Linux: `pkg-config` and `libssl-dev` (or equivalent) for OpenSSL linking

## Your Task

Check the `tasks/` folder for your assigned task:
- `TASK_BACKEND.md` — Rust backend developers
- `TASK_SMART_CONTRACT.md` — Solana/Anchor developers  
- `TASK_SECURITY.md` — Security engineers
- `TASK_DEVOPS.md` — DevOps engineers
- `TASK_FULLSTACK.md` — Fullstack developers

**Complete only your assigned task.** Other files are for different candidate profiles.

## Quick Start

### 1. Start Backend (Terminal 1)
```bash
cd backend
cp .env.example .env
cargo run
```

Verify: `curl http://localhost:8080/health`

### 2. Start Game Server (Terminal 2)
```bash
cd game-server
cargo run
```

Verify: `curl http://localhost:8081/health`

Both services must be running before you start your task.

## Project Structure

```
├── game-server/        # Rust game server (port 8081, battle results authority)
├── backend/            # Rust/Axum REST API (port 8080, reward signer)
├── program/            # Solana/Anchor smart contract
└── tasks/              # Your assignment (pick one)
```

### How It Works

1. Game server provides battle results
2. Backend verifies battles and signs reward claims  
3. Solana program verifies backend signatures and distributes rewards

## Submission

**Choose ONE method:**

### Option 1: GitHub Repository (Recommended)
1. Create a new public GitHub repository
2. Copy this project and complete your task
3. Email the link to: **tech@spinbattles.com**

### Option 2: File Sharing
1. Complete your task and create a ZIP file
2. Upload to Google Drive, Dropbox, or WeTransfer
3. Email the link to: **tech@spinbattles.com**

### Option 3: Pull Request
If you have write access to this repo:
1. Create branch `candidate/<your-name>`  
2. Submit pull request to `main`

**Email format:**
```
Subject: SpinBattles Assessment - [Your Name] - [Track]

Repository/Download Link: [link]

Summary: [3-5 sentences describing your work]

Test Commands:
[how to verify your solution]
```

## Tips

✅ Read your task file carefully  
✅ Focus on high-priority items first  
✅ Test that your code compiles and runs  
✅ Explain key decisions in comments  
✅ Don't spend more than 3 hours  

## Common Issues

**Backend returns 503:** Game server isn't running  
**Port in use:** Change `PORT` in `backend/.env`  
**Need persistent signer key:** Run `cargo run --bin keygen` in backend/ and add key to `.env`  

## Questions?

Email: **tech@spinbattles.com**

---

© SpinBattles. For evaluation purposes only.
