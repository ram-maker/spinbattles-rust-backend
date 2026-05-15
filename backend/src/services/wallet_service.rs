use crate::errors::AppError;

/// Wallet Service
///
/// Handles wallet signature verification and token balance checking.
///
/// Assessment note for Backend candidates:
///   `verify_signature()` is intentionally incomplete — implement it.
///   `get_token_balance()` falls back to realistic mock data when no RPC is configured,
///   but candidates should implement the real on-chain SPL token query.

/// Verify that an Ed25519 wallet signature is valid.
///
/// TODO (Backend task): Implement this function.
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
    // INCOMPLETE — implement signature verification here
    tracing::debug!("verify_signature called for address: {}", address);

    // Suppress unused variable warnings until implemented
    let _ = (address, signature, message);

    // Placeholder — always returns false until implemented
    Ok(false)
}

/// Validate that a string is a valid base58 Solana public key.
pub fn is_valid_pubkey(address: &str) -> bool {
    use solana_sdk::pubkey::Pubkey;
    use std::str::FromStr;
    Pubkey::from_str(address).is_ok()
}

/// Get the SBR token balance for a wallet address.
///
/// TODO (Backend task): Implement the real on-chain SPL token query.
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

    if rpc_url.is_some() && mint.is_some() {
        // TODO: replace this block with a real SPL token account balance query
        // Hint:
        //   let ata = spl_associated_token_account::get_associated_token_address(&wallet_pubkey, &mint_pubkey);
        //   POST rpc_url with { "method": "getTokenAccountBalance", "params": [ata.to_string()] }
        tracing::warn!("Real on-chain balance query not yet implemented — falling back to mock");
    }

    tracing::debug!("Using mock balance for: {}", address);
    Ok(crate::mock_data::get_mock_balance(address))
}
