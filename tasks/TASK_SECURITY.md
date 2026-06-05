# Security Review Assessment Task

## Your Assignment

You have been assigned the **Security Review** track.

## Time Estimate
2-3 hours

## Context

SpinBattles is preparing to deploy a Rust-based Web3 reward system. Before launch, we need a thorough security review of both the Solana program and the Rust/Axum backend API. This is not a static code review only — you must also probe the **running backend** to observe real behavior and test your findings.

## Setup (Required)

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

You need the backend running to:
- Observe actual API responses and error messages
- Test input validation by sending malformed requests
- Probe the `/api/rewards/sign` endpoint for logic flaws
- Verify whether rate limiting exists
- Check what information is leaked in error responses

---

## Your Tasks

### 1. Solana Program Security Review (Priority: HIGH)

**File:** `program/src/lib.rs`

Review the program and document vulnerabilities. Pay particular attention to the authorized signer pattern and the Ed25519 verification — it's the core security mechanism.

**Deliver:** `SECURITY_REPORT_PROGRAM.md`

Format each issue as:
```
Issue: [Name]
Severity: Critical / High / Medium / Low
Location: lib.rs line X, function Y
Description: What is wrong
Impact: What an attacker could do
Recommendation: How to fix it
Code: Fixed snippet (Rust)
```

Minimum: 3 issues with concrete fix examples.

### 2. Backend API Security Review (Priority: HIGH)

**Files:** `backend/src/routes/`, `backend/src/services/`

You must test the running API, not just read the code. Use curl or a tool like Postman.

**Things to probe:**
```bash
# Does the sign endpoint leak information on invalid input?
curl -X POST http://localhost:8080/api/rewards/sign \
  -H "Content-Type: application/json" \
  -d '{"address":"not-a-pubkey","wallet_signature":"x","wallet_message":"x","battle_id":"x"}'

# Is there rate limiting?
for i in $(seq 1 20); do
  curl -s http://localhost:8080/api/rewards/pending/9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM
done

# What does an invalid address return?
curl http://localhost:8080/api/wallet/not-a-pubkey/balance

# Can you claim a reward without a wallet signature?
curl -X POST http://localhost:8080/api/rewards/sign \
  -H "Content-Type: application/json" \
  -d '{
    "address": "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM",
    "wallet_signature": "invalidsig",
    "wallet_message": "Verify wallet ownership",
    "battle_id": "battle_1234"
  }'

# What happens with an oversized payload?
curl -X POST http://localhost:8080/api/rewards/sign \
  -H "Content-Type: application/json" \
  -d "{\"address\":\"9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM\",\"wallet_signature\":\"$(python3 -c 'print("A"*10000)')\",\"wallet_message\":\"x\",\"battle_id\":\"x\"}"
```

**Deliver:** `SECURITY_REPORT_BACKEND.md`

### 3. Architecture & Design Review (Priority: MEDIUM)

Consider the full system design:
- What are the trust boundaries between the player client, backend, and program?
- What is the trust boundary between the backend and the game server? Is it authenticated?
- What happens if the backend private key is compromised?
- What happens if the game server is compromised or returns fabricated battle results?
- Are there replay attack vectors? (different cluster, different program ID, expired signatures)
- Is the in-memory claim history a security risk?
- What economic attacks are possible against the reward vault?
- Is the `verify_signature` implementation correct for the wallet ownership check used before signing?

Add an `## Architecture Concerns` section to either report.

### 4. Provide Fixed Code (Priority: MEDIUM)

For your top 3 issues, provide fixed code — either as snippets in your reports or as separate fixed files.

---

## What We're Evaluating

- Can you identify real vulnerabilities, not just theoretical ones?
- Did you actually probe the running API or only read the code?
- Are your severity ratings justified?
- Are your fixes correct Rust/Anchor code?
- Is your report clear enough for a developer to act on?

---

## Submission

**Choose ONE of these methods:**

### Option 1: GitHub Repository (Recommended)
1. Create a new public GitHub repository
2. Copy this project and add your security reports
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
1. `SECURITY_REPORT_PROGRAM.md`
2. `SECURITY_REPORT_BACKEND.md`
3. Fixed code for top 3 issues
4. Email summary (3-5 sentences) of the most critical findings

**Email subject:** `SpinBattles Security Assessment - [Your Name]`
