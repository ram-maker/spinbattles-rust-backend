# Quick Start Guide

## For All Candidates — Both Services Must Be Running

The system has two services. Both must be running before you start your task:

- **Game Server** (port 8081) — authoritative source of battle results
- **Backend** (port 8080) — trusted signer authority; calls the game server to verify battles

The backend cannot authorise any reward claim without the game server running.
The Solana program cannot accept any claim without the backend running.

---

## Step 1: Install Prerequisites

```bash
# Rust (if not installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Solana CLI (Smart Contract candidates only)
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"

# Anchor CLI (Smart Contract candidates only)
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install latest && avm use latest
```

---

## Step 2: Start the Game Server

Open a terminal and run:

```bash
cd game-server
cargo run
```

Verify it's running:
```bash
curl http://localhost:8081/health
curl http://localhost:8081/battles/9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM
```

The second command should return a list of battles. Keep this terminal open.

---

## Step 3: Start the Backend

Open a second terminal:

```bash
cd backend
cp .env.example .env

# Generate a backend signer keypair
cargo run --bin keygen
# Paste BACKEND_SIGNER_PRIVATE_KEY into .env, then:

cargo run
```

Verify it's running:
```bash
curl http://localhost:8080/health
curl http://localhost:8080/api/rewards/signer-pubkey
curl http://localhost:8080/api/rewards/pending/9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM
```

All three must return valid JSON. If `pending` returns a 503, the game server is not running.

---

## Step 4: Set Up Your Track

### Smart Contract candidates
```bash
cd program
anchor build
```

The program's `initialize` instruction requires the signer pubkey from Step 3.

### Backend / Security candidates
Both services are running — open your assigned task file and begin.

### DevOps candidates
Both services are running locally — your task is to containerize them. Open `tasks/TASK_DEVOPS.md` and begin.

### Fullstack candidates
Both services are running — your task is to build the frontend UI. Open `tasks/TASK_FULLSTACK.md` and begin. The full API reference is in `docs/PROJECT_STRUCTURE.md`.

---

## Common Issues

**`pending` returns `503 Service Unavailable`**
→ The game server is not running. Go back to Step 2.

**`signer-pubkey` returns an error**
→ `BACKEND_SIGNER_PRIVATE_KEY` is missing in `backend/.env`. Re-run `cargo run --bin keygen`.

**Port already in use**
→ Change `PORT` in `backend/.env` or `GAME_SERVER_PORT` in the game server environment.

**`anchor build` fails**
→ Make sure you're on Anchor 0.29+ and Solana CLI 1.18+. Run `avm use latest`.

---

## Time Guide

- 15 min: Setup and reading your task
- 2 hours: Implementation
- 15 min: Testing and writing your summary

Don't spend more than 3 hours. We value your time.

## Questions?

Contact the technical team at tech@spinbattles.com if you have setup issues or questions about requirements.
