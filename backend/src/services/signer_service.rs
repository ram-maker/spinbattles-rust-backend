use anyhow::Result;
use solana_sdk::{signature::Keypair, signer::Signer};

/// Signer Service
///
/// The backend acts as the trusted off-chain authority for reward claims.
/// Before a player can call `claim_reward` on the Solana program, they must
/// obtain a signature from this service proving the backend authorised the claim.
///
/// This is the standard "authorized signer" pattern used in production Solana games:
///   1. Player requests a claim signature from the backend
///   2. Backend verifies the battle result and signs (player_pubkey || battle_id || amount)
///   3. Player submits the signature to the on-chain program
///   4. Program verifies the signature came from the known backend signer pubkey
///
/// Without a valid backend signature the program will reject the transaction.

/// Signs a reward claim authorisation.
///
/// The message layout matches exactly what the Solana program verifies:
///   player_pubkey (32 bytes) || battle_id_hash (32 bytes) || amount_lamports (8 bytes LE)
///
/// # Arguments
/// * `keypair`        - The backend signer keypair
/// * `player_pubkey`  - The claimant's base58 public key
/// * `battle_id`      - The battle identifier string
/// * `amount_lamports`- Token amount in lamports
///
/// Returns the base58-encoded Ed25519 signature and expiry timestamp.
pub fn sign_claim_authorisation(
    keypair: &Keypair,
    player_pubkey: &str,
    battle_id: &str,
    amount_lamports: u64,
) -> Result<(String, i64)> {
    use sha2::{Digest, Sha256};
    use solana_sdk::pubkey::Pubkey;
    use std::str::FromStr;

    let player_key = Pubkey::from_str(player_pubkey)
        .map_err(|_| anyhow::anyhow!("Invalid player pubkey"))?;

    // Hash the battle_id to a fixed 32-byte value
    let battle_id_hash: [u8; 32] = Sha256::digest(battle_id.as_bytes()).into();

    // Build the message: pubkey (32) || battle_id_hash (32) || amount LE (8)
    let mut message = Vec::with_capacity(72);
    message.extend_from_slice(player_key.as_ref());
    message.extend_from_slice(&battle_id_hash);
    message.extend_from_slice(&amount_lamports.to_le_bytes());

    let signature = keypair.sign_message(&message);
    let sig_b58 = bs58::encode(signature.as_ref()).into_string();

    // Signatures expire after 10 minutes to limit replay window
    let expires_at = chrono::Utc::now().timestamp() + 600;

    Ok((sig_b58, expires_at))
}
