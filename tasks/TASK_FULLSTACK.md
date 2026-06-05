# Fullstack Developer Assessment Task

## Your Assignment

You have been assigned the **Fullstack** track.

## Time Estimate
2-3 hours

## Context

SpinBattles needs a minimal web UI so players can connect their Solana wallet, check their SBR token balance, view pending battle rewards, and initiate a reward claim. The backend API is already built and includes working wallet signature verification — your job is to build the frontend that talks to it.

You are free to use any frontend framework you are comfortable with (React, Next.js, SvelteKit, Vue). The UI does not need to be polished — functionality and correctness matter more than styling.

## Setup (Required)

Both backend services must be running before you start the frontend:

```bash
# Terminal 1 — Backend
cd backend
cp .env.example .env
cargo run

# Terminal 2 — Game Server
cd game-server
cargo run
```

Verify both services are running:
```bash
curl http://localhost:8080/health
curl http://localhost:8081/health
curl http://localhost:8080/api/rewards/signer-pubkey
```

The backend API reference is in `docs/PROJECT_STRUCTURE.md` under **Backend API**.

---

## Your Tasks

### 1. Wallet Connection (Priority: HIGH)

Implement wallet connection using the Solana wallet adapter.

**Requirements:**
- Support at minimum Phantom wallet
- Display the connected wallet address once connected
- Handle the disconnected state gracefully
- Use `@solana/wallet-adapter-react` or equivalent for your framework

```bash
# Recommended packages
npm install @solana/wallet-adapter-react @solana/wallet-adapter-phantom @solana/web3.js
```

### 2. Wallet Balance Display (Priority: HIGH)

Once a wallet is connected, fetch and display the player's SBR token balance.

**Endpoint:**
```
GET http://localhost:8080/api/wallet/:address/balance
```

**Requirements:**
- Fetch balance automatically on wallet connect
- Display the balance clearly (the response includes `balance` and `balance_ui` fields)
- Show a loading state while fetching
- Handle errors — if the backend is unreachable, show a clear message rather than a blank screen

### 3. Pending Rewards List (Priority: HIGH)

Fetch and display the player's pending battle rewards.

**Endpoint:**
```
GET http://localhost:8080/api/rewards/pending/:address
```

**Requirements:**
- List each pending reward with `battle_id` and `amount`
- Each item should have a "Claim" button (wire up in task 4)
- Show an empty state if there are no pending rewards
- Refresh the list after a successful claim

### 4. Reward Claim Flow (Priority: MEDIUM)

Implement the claim flow when a player clicks "Claim" on a pending reward.

**Flow:**
1. Sign the exact message `"Verify wallet ownership"` with the connected wallet
2. POST to `/api/rewards/sign` with `address`, `wallet_signature`, `wallet_message`, `battle_id`
3. On success, display the returned `signature`, `amount_lamports`, and `expires_at`
4. POST to `/api/rewards/claim` with `address`, `battle_id`, `amount`, and `tx_signature`
5. Show success or error feedback and refresh the pending list

**Requirements:**
- The backend verifies the wallet Ed25519 signature — use the exact message string above
- Handle `SignatureVerificationFailed` and other API errors with clear user-visible messages
- CORS is already enabled on the backend for local frontend development

**Test the sign step manually (optional):**
```bash
curl -X POST http://localhost:8080/api/rewards/sign \
  -H "Content-Type: application/json" \
  -d '{
    "address": "<your_pubkey>",
    "wallet_signature": "<signature_of_Verify_wallet_ownership>",
    "wallet_message": "Verify wallet ownership",
    "battle_id": "<battle_id_from_pending_list>"
  }'
```

### 5. Reward History (Priority: LOW)

Add a history tab or section showing past claims for the connected wallet.

**Endpoint:**
```
GET http://localhost:8080/api/rewards/:address/history
```

---

## What We're Evaluating

- Can you integrate Solana wallet adapter correctly?
- Is your API integration clean — proper loading/error states, no silent failures?
- Is the claim flow implemented in the correct order?
- Is your component structure reasonable and readable?
- Do you understand the wallet signing step and why it exists?

---

## Submission

**Choose ONE of these methods:**

### Option 1: GitHub Repository (Recommended)
1. Create a new public GitHub repository
2. Copy this project and add your frontend
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
1. Frontend source code in a `frontend/` directory at the repo root
2. `frontend/README.md` with setup and run instructions
3. Brief summary (3-5 sentences): framework chosen, decisions made, what you would improve with more time

**Email subject:** `SpinBattles Fullstack Assessment - [Your Name]`
