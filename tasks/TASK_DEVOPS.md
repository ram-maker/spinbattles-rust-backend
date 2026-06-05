# DevOps Engineer Assessment Task

## Your Assignment

You have been assigned the **DevOps** track.

## Time Estimate
2-3 hours

> Focus on HIGH priority items first. Completing all four tasks is not expected — we'd rather see two things done well than four things done hastily.

## Context

SpinBattles is preparing to move the battle rewards system from local development to a containerized, production-ready deployment. Currently, developers run the game server and backend manually with `cargo run`. Your job is to containerize both services, wire them together, and set up a basic CI pipeline — so the team can ship reliably.

The system has two Rust services that must run in a specific order:
- `game-server` on port `8081` — must start first
- `backend` on port `8080` — depends on game-server being healthy

## Setup (Required)

Verify the services work locally before containerizing:

```bash
# Terminal 1 — Backend
cd backend
cp .env.example .env
cargo run

# Terminal 2 — Game Server
cd game-server
cargo run
```

Confirm both are healthy:
```bash
curl http://localhost:8080/health
curl http://localhost:8081/health
curl http://localhost:8080/api/rewards/signer-pubkey
```

---

## Your Tasks

### 1. Dockerize Both Services (Priority: HIGH)

Create a `Dockerfile` for each service. Both are standard Rust/Cargo projects.

**Requirements:**
- Use multi-stage builds — builder stage with full Rust toolchain, final stage with minimal base image
- Final image should be as small as practical (`debian:bookworm-slim` or `gcr.io/distroless/cc` are reasonable choices)
- The backend image must support running both `keygen` and the main server binary
- Environment variables must be configurable at runtime (not baked into the image)
- Images must build cleanly with `docker build`

**Expected files:**
```
game-server/Dockerfile
backend/Dockerfile
```

### 2. Docker Compose Orchestration (Priority: HIGH)

Create a `docker-compose.yml` at the repo root that brings up the full stack with a single command.

**Requirements:**
- Services start in the correct order (`game-server` before `backend`)
- Use a health check on `game-server` so `backend` only starts once it's ready
- `backend` reads its config from an `.env` file or environment block — do not hardcode secrets
- Expose ports `8080` and `8081` to the host
- Include a named network so services communicate by service name

**Test it:**
```bash
docker-compose up --build
curl http://localhost:8081/health
curl http://localhost:8080/health
curl http://localhost:8080/api/rewards/signer-pubkey
```

### 3. CI Pipeline (Priority: MEDIUM)

Add a GitHub Actions workflow that runs on every push and pull request to `main`.

**File:** `.github/workflows/ci.yml`

**Requirements:**
- Cache Cargo registry and build artifacts to keep runs fast
- Run `cargo build` for both `game-server` and `backend`
- Run `cargo clippy` with `-- -D warnings` (fail on warnings)
- Run `cargo fmt --check` (fail if formatting is off)
- Build both Docker images to verify they build cleanly (no push required)

### 4. Environment & Secrets Handling (Priority: MEDIUM)

Review how the backend handles its `BACKEND_SIGNER_PRIVATE_KEY` and document how this should be managed in a real deployment.

**Deliver:** A short `## Secrets Management` section added to `docs/PROJECT_STRUCTURE.md` or as a standalone `docs/DEPLOYMENT.md`.

Cover:
- Why the private key must never be baked into a Docker image or committed to git
- How you would inject it in a real environment (Docker secrets, environment injection at runtime, a secrets manager)
- What the blast radius is if the key leaks and how to rotate it

---

## What We're Evaluating

- Are your Dockerfiles production-minded — small images, no unnecessary layers, correct binary entrypoints?
- Does your Compose file handle startup ordering and health checks correctly?
- Is your CI pipeline practical and fast (caching, fail-fast)?
- Do you understand the security implications of secret management in containerized environments?

---

## Submission

**Choose ONE of these methods:**

### Option 1: GitHub Repository (Recommended)
1. Create a new public GitHub repository
2. Copy this project and add your DevOps files
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
1. `game-server/Dockerfile`
2. `backend/Dockerfile`
3. `docker-compose.yml`
4. `.github/workflows/ci.yml`
5. Secrets management documentation
6. Brief summary (3-5 sentences): decisions made, tradeoffs, what you would add with more time

**Email subject:** `SpinBattles DevOps Assessment - [Your Name]`
