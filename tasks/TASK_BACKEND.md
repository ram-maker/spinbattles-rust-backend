# Backend Developer Assessment Task

## Your Assignment

You have been assigned the **Rust Backend** track.

## Time Estimate
2-3 hours

## Context

SpinBattles uses a backend-signed authorization pattern for reward claims. Before a player can call `claim_reward` on the Solana program, they must obtain a signature from this backend. The backend is the trusted off-chain authority — the program will reject the transaction without a valid backend signature.

Your job is to complete the incomplete service implementations so the system works end-to-end.

## Setup (Required)

```bash
# Terminal 1 — Game Server (must be running first)
cd game-server
cargo run

# Terminal 2 — Backend
cd backend
cp .env.example .env
cargo run --bin keygen
# Paste BACKEND_SIGNER_PRIVATE_KEY into .env
cargo run
```

Verify both are running:
```bash
curl http://localhost:8081/health
curl http://localhost:8080/health
curl http://localhost:8080/api/rewards/signer-pubkey
```

The backend calls the game server to verify battle results — if the game server is down, the `/api/rewards/sign` endpoint returns `503`.

---

## Your Tasks

### 1. Implement Wallet Signature Verification (Priority: HIGH)

**File:** `backend/src/services/wallet_service.rs`
**Function:** `verify_signature(address, signature, message)`

This is a hard dependency for the `/api/rewards/sign` endpoint. Without it, the signing flow returns `SignatureVerificationFailed` and no claim signatures can be issued.

**Requirements:**
- Decode the base58 `address` into a `solana_sdk::pubkey::Pubkey`
- Decode the base58 `signature` into a `solana_sdk::signature::Signature`
- Call `signature.verify(pubkey.as_ref(), message.as_bytes())`
- Return `Ok(true)` / `Ok(false)` — do not panic on malformed input

**Test it:**
```bash
# First generate a real signature using the Solana CLI or a small Rust snippet:
# (see backend/src/bin/keygen.rs for keypair generation)

curl -X POST http://localhost:8080/api/wallet/verify \
  -H "Content-Type: application/json" \
  -d '{
    "address": "<your_pubkey>",
    "signature": "<base58_signature>",
    "message": "Verify wallet ownership"
  }'
# Expected: { "verified": true }
```

### 2. Implement Token Balance Checking (Priority: HIGH)

**File:** `backend/src/services/wallet_service.rs`
**Function:** `get_token_balance(address)`

**Requirements:**
- When `SOLANA_RPC_URL` and `SBR_TOKEN_MINT` are set, query the real SPL token account balance
- Derive the associated token account address for `(wallet, mint)` using the SPL ATA derivation
- Call the Solana JSON-RPC `getTokenAccountBalance` method
- Fall back to `mock_data::get_mock_balance()` when no RPC is configured (already in place)

**Test it:**
```bash
curl http://localhost:8080/api/wallet/9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM/balance
# Should return a balance string (mock or real)
```

### 3. Improve Claim Recording Validation (Priority: MEDIUM)

**File:** `backend/src/services/reward_service.rs`
**Function:** `record_claim(...)`

**Current issues:**
- No duplicate submission check (same `tx_signature` submitted twice)
- No `tx_signature` format validation (should be base58, 64 bytes decoded)
- No amount sanity check (zero or unreasonably large values accepted)

Choose ONE improvement to implement properly and explain your choice in your summary.

### 4. Error Handling & Logging (Priority: LOW)

Improve error messages and logging across any service or route file. Errors should be informative but must not leak the signer private key, internal Rust panics, or stack traces.

---

## What We're Evaluating

- Can you implement the core Web3 primitives (Ed25519 signature verification, RPC calls)?
- Do you understand why the authorized signer pattern exists?
- Is your validation and error handling production-minded?
- Is your Rust idiomatic — proper use of `Result`, `?`, `thiserror`?

---

## Submission

Submit from a **fork** (required for read-only access) or a **`candidate/<your-name>`** branch if given write access — do not push to `main`. Send a pull request or ZIP.

1. Updated code (ZIP, GitHub repo, or pull request)
2. Brief summary (3-5 sentences): what you implemented, key decisions, tradeoffs
3. Commands to test your changes
