use crate::errors::AppError;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use spl_associated_token_account::get_associated_token_address;
use std::str::FromStr;

/// Wallet Service
///
/// Handles wallet signature verification and token balance checking.

/// Verify that an Ed25519 wallet signature is valid.
///
/// A Solana wallet signs arbitrary messages using Ed25519. To verify:
///   1. Decode the base58 public key into a `Pubkey`
///   2. Decode the base58 signature into a `Signature`
///   3. Call `signature.verify(pubkey.as_ref(), message.as_bytes())`
///
/// Useful types:
///   - `solana_sdk::pubkey::Pubkey`
///   - `solana_sdk::signature::Signature`
///
/// # Arguments
/// * `address`   - Base58-encoded Solana public key (the claimed wallet)
/// * `signature` - Base58-encoded Ed25519 signature
/// * `message`   - The original plaintext message that was signed
///
/// Returns `Ok(true)` if valid, `Ok(false)` if the signature does not match.
pub fn verify_signature(address: &str, signature: &str, message: &str) -> Result<bool, AppError> {
    tracing::debug!("verify_signature called for address: {}", address);

    let pubkey = match Pubkey::from_str(address) {
        Ok(v) => v,
        Err(_) => {
            tracing::warn!("Invalid pubkey in verify_signature");
            return Ok(false);
        }
    };

    let sig = match Signature::from_str(signature) {
        Ok(v) => v,
        Err(_) => {
            tracing::warn!("Invalid base58 signature in verify_signature");
            return Ok(false);
        }
    };

    Ok(sig.verify(pubkey.as_ref(), message.as_bytes()))
}

/// Validate that a string is a valid base58 Solana public key.
pub fn is_valid_pubkey(address: &str) -> bool {
    use solana_sdk::pubkey::Pubkey;
    use std::str::FromStr;
    Pubkey::from_str(address).is_ok()
}

/// Get the SBR token balance for a wallet address.
///
/// When `SOLANA_RPC_URL` and `SBR_TOKEN_MINT` are set, query the real token account:
///   1. Derive the associated token account address for (wallet, mint)
///   2. Call `getTokenAccountBalance` via the Solana JSON-RPC
///   3. Return the `amount` field (as a string, to avoid u64 precision loss in JSON)
///
/// Useful crates already in Cargo.toml:
///   - `solana_sdk` for pubkey/address derivation
///   - `reqwest` (add if needed) for the RPC call, or use `solana_client`
///
/// When no RPC is configured, fall back to `mock_data::get_mock_balance()`.
///
/// Returns `(lamports_string, ui_string)` e.g. ("1500000000000", "1500.00 SBR")
pub async fn get_token_balance(address: &str) -> Result<(String, String), AppError> {
    let rpc_url = std::env::var("SOLANA_RPC_URL").ok();
    let mint = std::env::var("SBR_TOKEN_MINT").ok();

    if let (Some(rpc_url), Some(mint_address)) = (rpc_url, mint) {
        let wallet_pubkey = Pubkey::from_str(address).map_err(|e| {
            tracing::error!("Invalid Solana public key: {}", e);
            AppError::Internal(e.into())
        })?;
        let mint_pubkey = Pubkey::from_str(&mint_address).map_err(|e| {
            tracing::error!("Invalid SBR mint address: {}", e);
            AppError::Internal(e.into())
        })?;
        let ata_address = get_associated_token_address(&wallet_pubkey, &mint_pubkey);

        // 3. Create the RPC client and query the node
        // Note: RpcClient calls are blocking by default; wrap in spawn_blocking for async hygiene
        let client = RpcClient::new(rpc_url);
        let balance = client
            .get_token_account_balance(&ata_address)
            .map_err(|e| AppError::Internal(e.into()))?;
        let lamports_string = balance.amount;
        let ui_string = format!("{} SBR", balance.ui_amount_string);

        return Ok((lamports_string, ui_string));
    }
    tracing::debug!("Using mock balance for: {}", address);
    Ok(crate::mock_data::get_mock_balance(address))
}
