use crate::models::ClaimRecord;
use anyhow::{Context, Result};
use solana_sdk::{signature::Keypair, signer::Signer};
use std::collections::HashMap;
use std::sync::Mutex;

/// Shared application state, held behind Arc<AppState>.
pub struct AppState {
    pub signer: Option<Keypair>,
    /// In-memory claim history — a real system would use a database
    pub claim_history: Mutex<HashMap<String, ClaimRecord>>,
}

impl AppState {
    pub fn new() -> Result<Self> {
        let signer = match std::env::var("BACKEND_SIGNER_PRIVATE_KEY") {
            Ok(key) => {
                let bytes = bs58::decode(&key)
                    .into_vec()
                    .context("BACKEND_SIGNER_PRIVATE_KEY is not valid base58")?;
                let keypair = Keypair::from_bytes(&bytes)
                    .context("BACKEND_SIGNER_PRIVATE_KEY has invalid length (expected 64 bytes)")?;
                tracing::info!("Authorized signer pubkey: {}", keypair.pubkey());
                Some(keypair)
            }
            Err(_) => {
                tracing::warn!(
                    "BACKEND_SIGNER_PRIVATE_KEY not set. \
                     Claim signing will not work. \
                     Generate a key with: cargo run --bin keygen"
                );
                None
            }
        };

        Ok(Self {
            signer,
            claim_history: Mutex::new(HashMap::new()),
        })
    }

    /// Returns the signer's public key, or an error if not configured.
    pub fn signer_pubkey(&self) -> Result<String, crate::errors::AppError> {
        self.signer
            .as_ref()
            .map(|kp| kp.pubkey().to_string())
            .ok_or(crate::errors::AppError::SignerNotConfigured)
    }
}
