use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

/// SpinBattles Battle Rewards Program
///
/// Architecture: Authorized Signer Pattern
/// ----------------------------------------
/// This program does NOT trust the caller to self-report their reward.
/// Every `claim_reward` instruction must include a signature from the backend
/// (the "authorized signer"). The backend verifies the battle result off-chain
/// and signs the claim parameters before the player can submit on-chain.
///
/// This means:
///   - The backend MUST be running to generate claim signatures
///   - GET /api/rewards/signer-pubkey  → get the signer pubkey for initialization
///   - POST /api/rewards/sign          → get a signature before calling claim_reward
///
/// NOTE: This program has intentional security issues for assessment purposes.
///       Smart contract candidates: find and fix them.
///       Security review candidates: document them.
#[program]
pub mod spinbattles_program {
    use super::*;

    /// Initialize the reward vault and set the authorized backend signer.
    ///
    /// Must be called once by the program authority before any claims can be made.
    /// The `authorized_signer` must match `BACKEND_SIGNER_PRIVATE_KEY` in backend/.env.
    ///
    /// Retrieve the correct pubkey from the running backend:
    ///   GET http://localhost:8080/api/rewards/signer-pubkey
    pub fn initialize(
        ctx: Context<Initialize>,
        authorized_signer: Pubkey,
    ) -> Result<()> {
        require!(
            authorized_signer != Pubkey::default(),
            SpinBattlesError::InvalidSigner
        );

        let config = &mut ctx.accounts.config;
        config.authority = ctx.accounts.authority.key();
        config.authorized_signer = authorized_signer;
        config.vault = ctx.accounts.vault.key();
        config.bump = ctx.bumps.config;

        msg!("SpinBattles program initialized. Authorized signer: {}", authorized_signer);
        Ok(())
    }

    /// Claim a battle reward.
    ///
    /// Requires a valid Ed25519 signature from the backend authorized signer.
    /// Without the backend running and a valid signature, this instruction fails.
    ///
    /// To obtain a signature:
    ///   POST http://localhost:8080/api/rewards/sign
    ///   Body: { address, wallet_signature, wallet_message, battle_id }
    ///
    /// TODO (Smart Contract task): This function still has security issues.
    ///   - What happens if amount is 0?
    ///   - What happens if amount exceeds the vault balance?
    ///   - Is there a maximum reward cap per battle?
    ///   - Can the signature be replayed on a different program or cluster?
    ///   - Is the claim_record PDA seeded securely enough?
    ///
    /// # Arguments
    /// * `battle_id_hash` - SHA-256 hash of the battle_id string (32 bytes)
    /// * `amount`         - Token amount in lamports — must match what the backend signed
    /// * `signature`      - Base58-decoded backend Ed25519 signature (64 bytes)
    pub fn claim_reward(
        ctx: Context<ClaimReward>,
        battle_id_hash: [u8; 32],
        amount: u64,
        signature: [u8; 64],
    ) -> Result<()> {
        let config = &ctx.accounts.config;
        let player = ctx.accounts.player.key();

        // Verify the backend authorized this exact claim
        // Message layout: player_pubkey (32) || battle_id_hash (32) || amount LE (8)
        let mut message = Vec::with_capacity(72);
        message.extend_from_slice(player.as_ref());
        message.extend_from_slice(&battle_id_hash);
        message.extend_from_slice(&amount.to_le_bytes());

        let valid = verify_ed25519_signature(
            &config.authorized_signer.to_bytes(),
            &message,
            &signature,
        );
        require!(valid, SpinBattlesError::InvalidBackendSignature);

        // Prevent double-claiming
        let claim_record = &mut ctx.accounts.claim_record;
        require!(!claim_record.claimed, SpinBattlesError::AlreadyClaimed);

        // TODO: Add more validation here (Smart Contract task)

        // Mark as claimed before transfer (checks-effects-interactions)
        claim_record.claimed = true;
        claim_record.player = player;
        claim_record.battle_id_hash = battle_id_hash;
        claim_record.amount = amount;
        claim_record.claimed_at = Clock::get()?.unix_timestamp;

        // Transfer tokens from vault to player
        let seeds = &[b"config".as_ref(), &[config.bump]];
        let signer_seeds = &[&seeds[..]];

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vault.to_account_info(),
                    to: ctx.accounts.player_token_account.to_account_info(),
                    authority: ctx.accounts.config.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
        )?;

        emit!(RewardClaimed {
            player,
            battle_id_hash,
            amount,
        });

        Ok(())
    }

    /// Update the authorized signer (authority only).
    pub fn set_authorized_signer(
        ctx: Context<SetAuthorizedSigner>,
        new_signer: Pubkey,
    ) -> Result<()> {
        require!(new_signer != Pubkey::default(), SpinBattlesError::InvalidSigner);
        let old = ctx.accounts.config.authorized_signer;
        ctx.accounts.config.authorized_signer = new_signer;
        msg!("Authorized signer updated: {} -> {}", old, new_signer);
        Ok(())
    }
}

// ── Account structs ───────────────────────────────────────────────────────────

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + ProgramConfig::LEN,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, ProgramConfig>,

    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(battle_id_hash: [u8; 32])]
pub struct ClaimReward<'info> {
    #[account(seeds = [b"config"], bump = config.bump)]
    pub config: Account<'info, ProgramConfig>,

    #[account(
        init_if_needed,
        payer = player,
        space = 8 + ClaimRecord::LEN,
        seeds = [b"claim", player.key().as_ref(), &battle_id_hash],
        bump
    )]
    pub claim_record: Account<'info, ClaimRecord>,

    #[account(
        mut,
        constraint = vault.key() == config.vault @ SpinBattlesError::InvalidVault
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub player_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub player: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetAuthorizedSigner<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump,
        has_one = authority @ SpinBattlesError::Unauthorized
    )]
    pub config: Account<'info, ProgramConfig>,

    pub authority: Signer<'info>,
}

// ── State accounts ────────────────────────────────────────────────────────────

#[account]
pub struct ProgramConfig {
    pub authority: Pubkey,
    pub authorized_signer: Pubkey,
    pub vault: Pubkey,
    pub bump: u8,
}

impl ProgramConfig {
    pub const LEN: usize = 32 + 32 + 32 + 1;
}

#[account]
pub struct ClaimRecord {
    pub claimed: bool,
    pub player: Pubkey,
    pub battle_id_hash: [u8; 32],
    pub amount: u64,
    pub claimed_at: i64,
}

impl ClaimRecord {
    pub const LEN: usize = 1 + 32 + 32 + 8 + 8;
}

// ── Events ────────────────────────────────────────────────────────────────────

#[event]
pub struct RewardClaimed {
    pub player: Pubkey,
    pub battle_id_hash: [u8; 32],
    pub amount: u64,
}

// ── Errors ────────────────────────────────────────────────────────────────────

#[error_code]
pub enum SpinBattlesError {
    #[msg("Invalid backend signature")]
    InvalidBackendSignature,

    #[msg("Reward already claimed")]
    AlreadyClaimed,

    #[msg("Invalid authorized signer pubkey")]
    InvalidSigner,

    #[msg("Vault account does not match config")]
    InvalidVault,

    #[msg("Unauthorized")]
    Unauthorized,
}

// ── Crypto helpers ────────────────────────────────────────────────────────────

/// Verify an Ed25519 signature against a message and public key.
///
/// TODO (Smart Contract task): This function is a placeholder — it always returns false.
/// Fix it so `claim_reward` can actually verify backend signatures.
///
/// There are two valid approaches:
///
/// Option A — Ed25519 native program via instructions sysvar (recommended for production):
///   The client prepends a `solana_program::ed25519_program` instruction to the transaction.
///   The program reads and verifies it from `sysvar::instructions`. This is the most
///   compute-efficient approach and is used by production Solana programs.
///
/// Option B — Inline verification using `ed25519-dalek` (simpler, acceptable for assessment):
///   Add `ed25519-dalek = "2"` to Cargo.toml, then:
///   ```rust
///   use ed25519_dalek::{Signature, VerifyingKey};
///   let vk = VerifyingKey::from_bytes(pubkey_bytes).map_err(|_| false)?;
///   let sig = Signature::from_bytes(signature_bytes);
///   vk.verify_strict(message, &sig).is_ok()
///   ```
///
/// Document your chosen approach and its tradeoffs in your submission summary.
fn verify_ed25519_signature(pubkey_bytes: &[u8; 32], message: &[u8], signature_bytes: &[u8; 64]) -> bool {
    // INCOMPLETE — implement Ed25519 verification here (Smart Contract task)
    let _ = (pubkey_bytes, message, signature_bytes);
    false
}
