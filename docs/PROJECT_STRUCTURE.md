# SpinBattles Rust Web3 Assessment — Project Structure

## Overview

A simplified Web3 reward system for SpinBattles game battles. Built in Rust throughout — Rouille game server, Axum backend, Anchor/Solana on-chain.

## Directory Structure

```
spinbattles-rust-assessment/
├── README.md
├── QUICK_START.md
│
├── game-server/                    # Rust/Rouille game server (fully implemented — run first)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs                 # Server entrypoint
│       ├── mock_data.rs            # Deterministic battle data helpers
│       ├── models.rs               # JSON response models
│       └── routes/                 # Health and battle routes
│           ├── mod.rs
│           ├── health.rs
│           └── battles.rs
│
├── backend/                        # Rust/Axum REST API (trusted signer service)
│   ├── Cargo.toml
│   ├── .env.example
│   └── src/
│       ├── main.rs                 # Axum server setup and routing
│       ├── state.rs                # Shared AppState (signer, claim history)
│       ├── errors.rs               # AppError type with IntoResponse
│       ├── models.rs               # Request/response structs
│       ├── mock_data.rs            # Mock balance data (battle data comes from game-server)
│       ├── bin/
│       │   └── keygen.rs           # Keypair generation utility
│       ├── routes/
│       │   ├── health.rs
│       │   ├── wallet.rs           # POST /api/wallet/verify, GET /api/wallet/:addr/balance
│       │   └── rewards.rs          # GET /signer-pubkey, POST /sign, POST /claim, GET /pending, GET /history
│       └── services/
│           ├── game_client.rs      # HTTP client for game server calls (fully implemented)
│           ├── signer_service.rs   # Ed25519 claim signing (fully implemented)
│           ├── wallet_service.rs   # Signature verification + balance
│           └── reward_service.rs   # Claim authorization + history
│
├── program/                        # Solana/Anchor program
│   ├── Cargo.toml
│   ├── Anchor.toml
│   ├── package.json
│   ├── tsconfig.json
│   ├── tests/
│   │   └── spinbattles.ts          # Test skeleton — candidates complete this
│   └── src/
│       └── lib.rs                  # SpinBattles program
│
├── tasks/
│   ├── TASK_BACKEND.md
│   ├── TASK_SMART_CONTRACT.md
│   ├── TASK_SECURITY_REVIEW.md
│   ├── TASK_DEVOPS.md
│   └── TASK_FULLSTACK.md
│
└── docs/
    ├── ASSESSMENT_GUIDELINES.md
    └── PROJECT_STRUCTURE.md
```

## Startup Order (Required)

```
1. game-server  (port 8081)  — start first, always running
2. backend      (port 8080)  — start second, calls game-server
3. program                   — deployed on-chain, calls backend for signatures
```

## Data Flow

```
Player Client
     ↓
Rust/Axum Backend (port 8080)  ←── trusted signer authority
     ↓  calls for battle data
Rust/Rouille Game Server (port 8081) ←── authoritative battle results source
     ↓  (Ed25519 signature flows back up)
Solana Program
     ↓  (SPL token transfer)
Player Token Account
```

## Game Server API

```
GET /health
GET /battles/:address          — list won battles for a player
GET /battles/:battle_id/verify — verify a specific battle result
```

## Backend API

### Wallet
```
POST /api/wallet/verify
  Body: { address, signature, message }
  Returns: { verified: bool }

GET /api/wallet/:address/balance
  Returns: { balance, balance_ui }
```

### Rewards
```
GET  /api/rewards/signer-pubkey
  Returns: { signer_pubkey }

POST /api/rewards/sign
  Body: { address, wallet_signature, wallet_message, battle_id }
  Returns: { signature, amount_lamports, expires_at }

POST /api/rewards/claim
  Body: { address, battle_id, amount, tx_signature }
  Returns: { tx_signature, status }

GET  /api/rewards/pending/:address
  Returns: { pending_rewards: [...] }

GET  /api/rewards/:address/history
  Returns: { history: [...] }
```

## Program Instructions

```rust
initialize(authorized_signer: Pubkey)
claim_reward(battle_id_hash: [u8; 32], amount: u64, signature: [u8; 64])
set_authorized_signer(new_signer: Pubkey)
```

## Current Notes

- Backend performs wallet signature verification and claim-record validation.
- Program verifies backend authorization using the Ed25519 native instruction/sysvar pattern.
- Token-balance lookup still uses deterministic fallback data unless RPC integration is enabled.

## Environment Variables

### Backend (.env)
```
PORT=8080
RUST_LOG=info
GAME_SERVER_URL=http://localhost:8081
SOLANA_RPC_URL=https://api.devnet.solana.com
SBR_TOKEN_MINT=<mint_address>
BACKEND_SIGNER_PRIVATE_KEY=<base58_private_key>
```

### Game Server (optional)
```
GAME_SERVER_PORT=8081
```
