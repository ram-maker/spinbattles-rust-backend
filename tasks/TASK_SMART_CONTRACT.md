# Smart Contract Developer Assessment Task

## Your Assignment

You have been assigned the **Solana / Anchor Program** track.

## Time Estimate
2-3 hours

## Context

`program/src/lib.rs` uses the **authorized signer pattern**: every `claim_reward` instruction must include a signature from the backend. The backend is the trusted off-chain authority that verifies battle results before signing.

This means you **must have the backend running** to test your program — the `initialize` instruction requires the backend signer pubkey, and `claim_reward` will fail with `InvalidBackendSignature` unless you pass a signature obtained from the backend.

## Setup (Required)

### 1. Start the game server and backend

```bash
# Terminal 1
cd game-server
cargo run

# Terminal 2
cd backend
cp .env.example .env
cargo run --bin keygen
# Paste BACKEND_SIGNER_PRIVATE_KEY into .env
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

### 1. Fix Ed25519 Signature Verification (Priority: HIGH)

**File:** `program/src/lib.rs`
**Function:** `verify_ed25519_signature()`

The current placeholder always returns `false` — the program will reject every claim. Fix it.

**The correct approach for Anchor programs:**

Use the `Ed25519` native program via the `instructions` sysvar. This is the production-standard pattern:

```rust
// In your ClaimReward accounts struct, add:
pub sysvar_instructions: AccountInfo<'info>,

// In claim_reward, verify using the sysvar:
use solana_program::sysvar::instructions;
// The client must prepend an Ed25519 instruction to the transaction.
// The program then reads and verifies it from the sysvar.
```

Alternatively, use the `ed25519-dalek` crate for inline verification (simpler for the assessment, less gas-efficient).

Document your approach and tradeoffs in your summary.

### 2. Security Review & Fixes (Priority: HIGH)

The program has intentional security issues. Find and fix them. The `TODO` comments in `claim_reward` point you in the right direction, but there are more issues beyond those hints.

**What to look for:**
- Can the signature be replayed on a different program or cluster?
- What happens if `amount` is 0 or exceeds the vault balance?
- Is the `claim_record` PDA seeded securely? Could two different battles collide?
- Should there be a maximum reward cap per claim?
- What happens if `initialize` is called twice?

### 3. Write Anchor Tests (Priority: HIGH)

Your tests must interact with the backend to get real signatures. This mirrors how the program works in production.

```typescript
// tests/spinbattles.ts
import * as anchor from "@coral-xyz/anchor";
import axios from "axios";
import * as crypto from "crypto";

describe("spinbattles", () => {
  it("claims reward with valid backend signature", async () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    // Get signer pubkey from backend
    const { data: signerData } = await axios.get(
      "http://localhost:8080/api/rewards/signer-pubkey"
    );

    // Initialize program with backend signer
    // ... (deploy and initialize)

    // Get pending battles for the player
    const { data: pending } = await axios.get(
      `http://localhost:8080/api/rewards/pending/${provider.wallet.publicKey}`
    );
    const battle = pending.pending_rewards[0];

    // Get a backend signature
    // NOTE: /api/rewards/sign requires wallet signature verification.
    // You may need to implement verify_signature() in the backend first,
    // or temporarily bypass it — document your approach.
    const { data: auth } = await axios.post(
      "http://localhost:8080/api/rewards/sign",
      {
        address: provider.wallet.publicKey.toString(),
        wallet_signature: "<sign wallet_message with player keypair>",
        wallet_message: "Verify wallet ownership",
        battle_id: battle.battle_id,
      }
    );

    // Call claim_reward with the backend signature
    // ...

    // Verify the claim_record PDA is marked as claimed
    // ...
  });

  it("rejects claim without valid backend signature", async () => {
    // ...
  });

  it("prevents double claiming", async () => {
    // ...
  });
});
```

### 4. Gas Optimization (Priority: LOW)

Review `claim_reward` for compute unit improvements. Consider:
- Using the Ed25519 native program instead of inline verification
- Reducing account data sizes
- Removing redundant checks

---

## What We're Evaluating

- Can you identify real Solana/Anchor vulnerabilities?
- Do you understand the authorized signer pattern and its security implications?
- Can you write tests that integrate with an external service?
- Is your Rust idiomatic and your Anchor usage correct?

---

## Submission

1. Updated `program/src/lib.rs` with fixes and comments
2. Test file(s) with results (`anchor test` output)
3. Summary (5-7 sentences): vulnerabilities found, fixes applied, tradeoffs
