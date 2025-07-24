use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgBfv3ho9SMz");

#[program]
pub mod crypto_card_gateway {
    use super::*;

    pub fn initialize_wallet(ctx: Context<InitializeWallet>) -> Result<()> {
        let user_wallet = &mut ctx.accounts.user_wallet;
        user_wallet.owner = *ctx.accounts.owner.key;
        user_wallet.card_number = random_u64();
        user_wallet.cvc = random_u8();
        user_wallet.expiry = get_expiry_timestamp();
        user_wallet.approval_required = true;
        Ok(())
    }

    pub fn toggle_approval(ctx: Context<UpdateWallet>, approve: bool) -> Result<()> {
        let user_wallet = &mut ctx.accounts.user_wallet;
        require!(user_wallet.owner == *ctx.accounts.owner.key, CustomError::Unauthorized);
        user_wallet.approval_required = approve;
        Ok(())
    }

    pub fn regenerate_card(ctx: Context<UpdateWallet>) -> Result<()> {
        let user_wallet = &mut ctx.accounts.user_wallet;
        require!(user_wallet.owner == *ctx.accounts.owner.key, CustomError::Unauthorized);
        user_wallet.cvc = random_u8();
        user_wallet.expiry = get_expiry_timestamp();
        Ok(())
    }

    pub fn authorize_payment(ctx: Context<AuthorizePayment>, device_hash: u64) -> Result<()> {
        let user_wallet = &ctx.accounts.user_wallet;
        let merchant = &ctx.accounts.merchant;

        require!(user_wallet.device_hash == device_hash, CustomError::DeviceMismatch);

        if user_wallet.approval_required {
            require!(merchant.approved == true, CustomError::ApprovalRequired);
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeWallet<'info> {
    #[account(init, payer = owner, space = 8 + 64)]
    pub user_wallet: Account<'info, Wallet>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateWallet<'info> {
    #[account(mut)]
    pub user_wallet: Account<'info, Wallet>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct AuthorizePayment<'info> {
    pub user_wallet: Account<'info, Wallet>,
    pub merchant: Account<'info, Merchant>,
}

#[account]
pub struct Wallet {
    pub owner: Pubkey,
    pub card_number: u64,
    pub cvc: u8,
    pub expiry: i64,
    pub approval_required: bool,
    pub device_hash: u64,
}

#[account]
pub struct Merchant {
    pub approved: bool,
}

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized access.")]
    Unauthorized,
    #[msg("Approval required for this transaction.")]
    ApprovalRequired,
    #[msg("Device mismatch detected.")]
    DeviceMismatch,
}

// Dummy generators for demo (you'd replace these with real secure randomness in production)
fn random_u64() -> u64 {
    1234567890123456
}

fn random_u8() -> u8 {
    123
}

fn get_expiry_timestamp() -> i64 {
    Clock::get().unwrap().unix_timestamp + 31536000  // 1 year from now
}
