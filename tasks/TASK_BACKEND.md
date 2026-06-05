# Backend Developer Assessment Task

## Your Assignment

You have been assigned the **Rust Backend** track.

## Time Estimate
2-3 hours

## Context

SpinBattles uses a backend-signed authorization pattern for reward claims. Before a player can call `claim_reward` on the Solana program, they must obtain a signature from this backend. The backend is the trusted off-chain authority — the program will reject the transaction without a valid backend signature.

The baseline already includes a working end-to-end off-chain flow:

- Ed25519 wallet signature verification (`wallet_service::verify_signature`)
- Battle lookup and eligibility checks via the game server
- Backend claim signing (`signer_service::sign_claim_authorisation`)
- Claim recording with amount, `tx_signature`, and duplicate checks (`reward_service::record_claim`)

Your job is to **extend and harden** the backend — not re-implement what is already there.

## Setup (Required)

```bash
# Terminal 1 — Backend (must be running first)
cd backend
cp .env.example .env
cargo run

# Terminal 2 — Game Server
cd game-server
cargo run
```

Verify both are running:
```bash
curl http://localhost:8080/health
curl http://localhost:8081/health
curl http://localhost:8080/api/rewards/signer-pubkey
```

The backend calls the game server to verify battle results — if the game server is down, the `/api/rewards/sign` endpoint returns `503`.

---

## Your Tasks

### 1. Implement On-Chain Token Balance Lookup (Priority: HIGH)

**File:** `backend/src/services/wallet_service.rs`
**Function:** `get_token_balance(address)`

When `SOLANA_RPC_URL` and `SBR_TOKEN_MINT` are set, the baseline still falls back to mock data. Implement the real path:

**Requirements:**
- Derive the associated token account address for `(wallet, mint)` using the SPL ATA derivation
- Call the Solana JSON-RPC `getTokenAccountBalance` method (via `reqwest` or `solana_client`)
- Return the on-chain balance when the token account exists
- Keep the existing `mock_data::get_mock_balance()` fallback when RPC env vars are missing or the account does not exist

**Test it:**
```bash
curl http://localhost:8080/api/wallet/9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM/balance
# Without RPC configured: mock balance
# With SOLANA_RPC_URL + SBR_TOKEN_MINT set: real balance when account exists
```

### 2. Harden the Game Server Client (Priority: MEDIUM)

**File:** `backend/src/services/game_client.rs`

The game client calls the game server with bare `reqwest::get` and no timeout. Improve reliability:

**Requirements:**
- Use a shared `reqwest::Client` with a reasonable request timeout (e.g. 5 seconds)
- Map timeout and connection errors to `AppError::GameServerUnavailable` with clear log lines
- Do not change the game server API contract

**Test it:** Stop the game server and confirm `/api/rewards/sign` fails fast with a clear error instead of hanging.

### 3. Choose One Security Hardening (Priority: MEDIUM)

Pick **one** of the following and implement it properly. Explain your choice in your summary.

| Option | File | Gap |
|--------|------|-----|
| **A. Standard wallet message** | `reward_service.rs` or `routes/rewards.rs` | Reject `/api/rewards/sign` unless `wallet_message` equals `"Verify wallet ownership"` exactly |
| **B. Signature expiry enforcement** | `reward_service.rs` | Reject or flag claims when recording a claim whose backend signature would have expired |
| **C. Tighter CORS** | `main.rs` | Replace `CorsLayer::permissive()` with an explicit allowlist for local frontend dev (e.g. `http://localhost:5173`) |

### 4. Error Handling & Logging (Priority: LOW)

Improve error messages and logging across any service or route file. Errors should be informative but must not leak the signer private key, internal Rust panics, or stack traces.

---

## What We're Evaluating

- Can you implement the core Web3 primitives (SPL ATA derivation, JSON-RPC calls)?
- Do you understand why the authorized signer pattern exists?
- Is your validation and error handling production-minded?
- Is your Rust idiomatic — proper use of `Result`, `?`, `thiserror`?

---

## Submission

**Choose ONE of these methods:**

### Option 1: GitHub Repository (Recommended)
1. Create a new public GitHub repository
2. Copy this project and add your changes
3. Email the repository link to: **tech@spinbattles.com**

### Option 2: File Sharing
1. Create a ZIP file with your completed work
2. Upload to Google Drive, Dropbox, or WeTransfer
3. Email the download link to: **tech@spinbattles.com**

### Option 3: Pull Request
If you have write access:
1. Create branch `candidate/<your-name>`
2. Submit pull request to `main`

---

**Your submission must include:**
1. Updated code (with your implementation)
2. Brief summary (3-5 sentences): what you implemented, key decisions, tradeoffs
3. Commands to test your changes

**Email subject:** `SpinBattles Backend Assessment - [Your Name]`
