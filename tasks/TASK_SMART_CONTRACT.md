# Smart Contract Developer Assessment Task

## Your Assignment

You have been assigned the **Solana / Anchor Program** track.

## Time Estimate
2-3 hours

> Focus on HIGH priority items first. This is the most demanding track — completing all tasks is not expected. A strong submission covers meaningful program security fixes, at least two passing integration tests, and a clear summary.

## Context

`program/src/lib.rs` uses the **authorized signer pattern**: every `claim_reward` instruction must include a signature from the backend. The backend is the trusted off-chain authority that verifies battle results before signing.

The baseline already includes a working core on-chain flow:

- Backend Ed25519 authorization via the **instructions sysvar** (`verify_ed25519_signature`)
- Basic claim validation (`amount > 0`, player token account owner/mint checks)
- `claim_record` PDA with double-claim protection
- Vault constraint tied to `config.vault`

Your job is to **harden the program and complete integration tests** — not re-implement what is already there.

You **must have the game server and backend running** to test end-to-end — `initialize` needs the backend signer pubkey, and `claim_reward` needs a signature from `POST /api/rewards/sign`.

## Setup (Required)

### 1. Start the game server and backend

```bash
# Terminal 1 — Backend
cd backend
cp .env.example .env
cargo run

# Terminal 2 — Game Server
cd game-server
cargo run
```

### 2. Get the signer pubkey

```bash
curl http://localhost:8080/api/rewards/signer-pubkey
# Returns: { "signer_pubkey": "..." }
# You need this pubkey for the initialize instruction
```

### 3. Set up the program

```bash
cd program
anchor build
anchor test --skip-local-validator   # or: solana-test-validator in another terminal
```

---

## Your Tasks

### 1. Program Security Review & Fixes (Priority: HIGH)

**File:** `program/src/lib.rs`

Review the program for remaining vulnerabilities and fix the ones you consider most important. The baseline is not production-ready.

**Examples to investigate (not an exhaustive list):**
- Can a claim drain more tokens than the vault holds?
- Is there a sensible maximum reward cap per claim?
- Does `initialize` validate that the vault token account uses the expected mint and authority?
- Can the same backend authorization be abused across unintended contexts (wrong vault, wrong player token account, etc.)?
- Are account constraints on `ClaimReward` complete and minimal?

Implement at least **two** concrete fixes with brief inline comments explaining each one.

### 2. Complete Anchor Integration Tests (Priority: HIGH)

**File:** `program/tests/spinbattles.ts`

The test skeleton is intentionally incomplete. Finish it so it exercises the real backend + program flow.

**Requirements:**
- Deploy and `initialize` the program with the backend signer pubkey from `/api/rewards/signer-pubkey`
- Fetch a pending battle from `/api/rewards/pending/:address`
- Sign the exact message `"Verify wallet ownership"` with the test wallet (backend verifies this)
- Call `POST /api/rewards/sign`, then invoke `claim_reward` on-chain with the returned signature
- Add at least **two passing tests**, for example:
  - successful claim with valid backend signature
  - rejection without a valid backend signature **or** double-claim prevention

The backend wallet verification is already implemented — use `"Verify wallet ownership"` exactly.

### 3. Transaction Construction for Ed25519 Sysvar Verification (Priority: MEDIUM)

**Files:** `program/tests/spinbattles.ts` (and helper code if needed)

`claim_reward` expects a **prior Ed25519 verify instruction** in the same transaction (read via the instructions sysvar). Your client/test code must prepend that instruction correctly when calling the program.

**Requirements:**
- Build a transaction that prepends the Ed25519 instruction matching the backend-signed message layout:
  `player_pubkey (32) || battle_id_hash (32) || amount_lamports (8 LE)`
- Document briefly in your summary how your test constructs this transaction

### 4. Compute & Account Layout Review (Priority: LOW)

Review `claim_reward` for compute and account-size improvements. The program already uses the native Ed25519 program via the sysvar — focus on redundant work, account sizes, or unnecessary allocations.

---

## What We're Evaluating

- Can you identify real Solana/Anchor vulnerabilities beyond the baseline?
- Do you understand the authorized signer pattern and the Ed25519 sysvar flow?
- Can you write tests that integrate with the running backend?
- Is your Rust idiomatic and your Anchor usage correct?

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
1. Updated `program/src/lib.rs` with fixes and comments
2. Completed `program/tests/spinbattles.ts` with `anchor test` output
3. Summary (5-7 sentences): vulnerabilities found, fixes applied, tradeoffs

**Email subject:** `SpinBattles Smart Contract Assessment - [Your Name]`
