import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import axios from "axios";
import * as crypto from "crypto";

/**
 * SpinBattles Program Tests
 *
 * These tests interact with the running backend to get real signatures.
 * Both services must be running before you run these tests:
 *
 *   Terminal 1: cd game-server && cargo run
 *   Terminal 2: cd backend && cargo run
 *
 * Then run tests:
 *   anchor test --skip-local-validator
 *
 * NOTE: /api/rewards/sign requires wallet signature verification (verify_signature).
 * You may need to implement that in the backend first, or temporarily bypass it
 * for testing purposes — document your approach in your submission summary.
 */

const BACKEND_URL = "http://localhost:8080";

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

    // Step 4: Sign the wallet message to prove ownership
    // TODO: sign "Verify wallet ownership" with the player keypair
    // const walletSignature = await provider.wallet.signMessage(
    //   Buffer.from("Verify wallet ownership")
    // );

    // Step 5: Get a backend claim signature
    // const { data: auth } = await axios.post(`${BACKEND_URL}/api/rewards/sign`, {
    //   address: playerPubkey,
    //   wallet_signature: bs58.encode(walletSignature),
    //   wallet_message: "Verify wallet ownership",
    //   battle_id: battle.battle_id,
    // });
    // console.log("Backend signature:", auth.signature);

    // Step 6: Call claim_reward on the program
    // TODO: build the battle_id_hash (SHA-256 of battle_id string)
    // const battleIdHash = crypto.createHash("sha256")
    //   .update(battle.battle_id)
    //   .digest();
    // const signatureBytes = bs58.decode(auth.signature);
    // await program.methods
    //   .claimReward([...battleIdHash], new anchor.BN(auth.amount_lamports), [...signatureBytes])
    //   .accounts({ ... })
    //   .rpc();

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
