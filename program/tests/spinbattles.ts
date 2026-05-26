import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import axios from "axios";
import * as crypto from "crypto";

/**
 * SpinBattles Program Tests
 *
 * These tests interact with the running game server and backend to obtain
 * real claim authorizations. Both services must be running before you run tests:
 *
 *   Terminal 1: cd game-server && cargo run
 *   Terminal 2: cd backend && cargo run
 *
 * Then run tests:
 *   anchor test --skip-local-validator
 *
 * Backend wallet verification is implemented. Sign the exact message
 * "Verify wallet ownership" before calling POST /api/rewards/sign.
 *
 * claim_reward also requires a prior Ed25519 verify instruction in the same
 * transaction (instructions sysvar pattern). See TASK_SMART_CONTRACT.md.
 */

const BACKEND_URL = "http://localhost:8080";
const WALLET_MESSAGE = "Verify wallet ownership";

describe("spinbattles", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  it("claims reward with valid backend signature", async () => {
    // Step 1: Get the backend signer pubkey
    const { data: signerData } = await axios.get(
      `${BACKEND_URL}/api/rewards/signer-pubkey`
    );
    console.log("Backend signer pubkey:", signerData.signer_pubkey);

    // Step 2: Deploy and initialize the program with the backend signer
    // TODO: deploy program and call initialize(signerData.signer_pubkey)

    // Step 3: Get pending battles for the player
    const playerPubkey = provider.wallet.publicKey.toString();
    const { data: pending } = await axios.get(
      `${BACKEND_URL}/api/rewards/pending/${playerPubkey}`
    );
    const battle = pending.pending_rewards[0];
    console.log("Battle to claim:", battle.battle_id);

    // Step 4: Sign the wallet ownership message (backend verifies this)
    // TODO: sign WALLET_MESSAGE with the player keypair and base58-encode it
    // const walletSignature = await provider.wallet.signMessage(
    //   Buffer.from(WALLET_MESSAGE)
    // );

    // Step 5: Get a backend claim authorization
    // const { data: auth } = await axios.post(`${BACKEND_URL}/api/rewards/sign`, {
    //   address: playerPubkey,
    //   wallet_signature: bs58.encode(walletSignature),
    //   wallet_message: WALLET_MESSAGE,
    //   battle_id: battle.battle_id,
    // });
    // console.log("Backend signature:", auth.signature);

    // Step 6: Build battle_id_hash and prepend Ed25519 verify ix, then claim
    // TODO: battle_id_hash = SHA-256(battle.battle_id)
    // TODO: prepend Ed25519 instruction for backend signer + message layout
    // TODO: call program.methods.claimReward(...).accounts({ ... }).rpc()

    // Step 7: Verify the claim_record PDA is marked as claimed
    // const claimRecord = await program.account.claimRecord.fetch(claimRecordPda);
    // assert.isTrue(claimRecord.claimed);
  });

  it("rejects claim without valid backend signature", async () => {
    // TODO: attempt claimReward with a random/invalid signature
    // expect it to fail with InvalidBackendSignature
  });

  it("prevents double claiming", async () => {
    // TODO: claim once successfully, then attempt to claim again
    // expect the second attempt to fail with AlreadyClaimed
  });
});
