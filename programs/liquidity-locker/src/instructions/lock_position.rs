use std::ops::Add;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer };
use anchor_spl::associated_token::AssociatedToken;

use crate::state::Locker;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct LockPosition<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mint::decimals = 0,
        constraint = nft_mint.supply == 1 @ErrorCode::MintNotNFT 
    )]
    pub nft_mint: Account<'info, Mint>,

    #[account(
        init,
        seeds = [
            user.key().as_ref(), 
            nft_mint.key().as_ref(),
            Locker::SEEDS
        ],
        bump,
        payer = user,
        space = Locker::LEN,
    )]
    pub locker: Account<'info, Locker>,

    #[account(
        init,
        payer = user,
        associated_token::mint = nft_mint,
        associated_token::authority = locker,
    )]
    pub nft_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user,
        constraint = nft_token_account.amount == 1 @ErrorCode::TokenAccountEmpty
    )]
    pub nft_token_account: Account<'info, TokenAccount>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn lock_position_handler(ctx: Context<LockPosition>, _duration:u32, _allow_fee_claim:bool) -> Result<()> {
    require!(_duration > 0, ErrorCode::InvalidDuration);

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer{
            from: ctx.accounts.nft_token_account.to_account_info(),
            to: ctx.accounts.nft_vault.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        }
    );

    token::transfer(cpi_ctx, 1)?;

    let current_time:u64 = Clock::get()?.unix_timestamp.try_into().unwrap();
    let unlock_time = current_time.add(u64::from(_duration));

    let locker = &mut ctx.accounts.locker;
    **locker = Locker::init(
        ctx.accounts.user.key(),
        ctx.accounts.nft_mint.key(),
        unlock_time,
        _allow_fee_claim
    );

    Ok(())
}