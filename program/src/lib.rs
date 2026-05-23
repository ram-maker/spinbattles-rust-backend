use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    ed25519_program,
    instruction::Instruction,
    sysvar::instructions::{load_current_index_checked, load_instruction_at_checked},
};
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
            &ctx.accounts.instructions.to_account_info(),
            &config.authorized_signer.to_bytes(),
            &message,
            &signature,
        );
        require!(valid, SpinBattlesError::InvalidBackendSignature);
        require!(amount > 0, SpinBattlesError::InvalidAmount);
        require!(
            ctx.accounts.player_token_account.owner == player,
            SpinBattlesError::InvalidPlayerTokenAccount
        );
        require!(
            ctx.accounts.player_token_account.mint == ctx.accounts.vault.mint,
            SpinBattlesError::InvalidPlayerTokenAccount
        );

        // Prevent double-claiming
        let claim_record = &mut ctx.accounts.claim_record;
        require!(!claim_record.claimed, SpinBattlesError::AlreadyClaimed);

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

    /// CHECK: constrained to the Solana instructions sysvar account.
    #[account(address = anchor_lang::solana_program::sysvar::instructions::ID)]
    pub instructions: UncheckedAccount<'info>,

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

    #[msg("Invalid claim amount")]
    InvalidAmount,

    #[msg("Invalid player token account")]
    InvalidPlayerTokenAccount,
}

// ── Crypto helpers ────────────────────────────────────────────────────────────

/// Verifies backend signature by checking a prior Ed25519 program instruction.
///
/// The transaction must prepend an Ed25519 verify instruction that validates:
/// `signature(pubkey_bytes, message)`.
fn verify_ed25519_signature(
    instructions_sysvar: &AccountInfo,
    pubkey_bytes: &[u8; 32],
    message: &[u8],
    signature_bytes: &[u8; 64],
) -> bool {
    let current_index = match load_current_index_checked(instructions_sysvar) {
        Ok(v) => v as usize,
        Err(_) => return false,
    };
    if current_index == 0 {
        return false;
    }

    for index in 0..current_index {
        let instruction = match load_instruction_at_checked(index, instructions_sysvar) {
            Ok(v) => v,
            Err(_) => continue,
        };
        if !is_matching_ed25519_ix(&instruction, pubkey_bytes, message, signature_bytes) {
            continue;
        }
        return true;
    }
    false
}

fn is_matching_ed25519_ix(
    ix: &Instruction,
    expected_pubkey: &[u8; 32],
    expected_message: &[u8],
    expected_signature: &[u8; 64],
) -> bool {
    if ix.program_id != ed25519_program::id() {
        return false;
    }
    parse_ed25519_ix_data(&ix.data)
        .map(|(sig, pubkey, message)| {
            sig == expected_signature.as_slice()
                && pubkey == expected_pubkey.as_slice()
                && message == expected_message
        })
        .unwrap_or(false)
}

fn parse_ed25519_ix_data(data: &[u8]) -> Option<(&[u8], &[u8], &[u8])> {
    const HEADER_LEN: usize = 2;
    const OFFSETS_LEN: usize = 14;
    const SELF_IX_INDEX: u16 = u16::MAX;

    if data.len() < HEADER_LEN + OFFSETS_LEN || data[0] != 1 {
        return None;
    }

    let offsets = &data[HEADER_LEN..HEADER_LEN + OFFSETS_LEN];
    let signature_offset = read_u16(offsets, 0)? as usize;
    let signature_ix_index = read_u16(offsets, 2)?;
    let pubkey_offset = read_u16(offsets, 4)? as usize;
    let pubkey_ix_index = read_u16(offsets, 6)?;
    let message_offset = read_u16(offsets, 8)? as usize;
    let message_size = read_u16(offsets, 10)? as usize;
    let message_ix_index = read_u16(offsets, 12)?;

    if signature_ix_index != SELF_IX_INDEX
        || pubkey_ix_index != SELF_IX_INDEX
        || message_ix_index != SELF_IX_INDEX
    {
        return None;
    }

    let signature = data.get(signature_offset..signature_offset + 64)?;
    let pubkey = data.get(pubkey_offset..pubkey_offset + 32)?;
    let message = data.get(message_offset..message_offset + message_size)?;
    Some((signature, pubkey, message))
}

fn read_u16(bytes: &[u8], offset: usize) -> Option<u16> {
    let slice = bytes.get(offset..offset + 2)?;
    Some(u16::from_le_bytes([slice[0], slice[1]]))
}
