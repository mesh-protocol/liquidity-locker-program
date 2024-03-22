use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Mint, Token, TokenAccount, Transfer };
use anchor_spl::associated_token::AssociatedToken;

use crate::state::Locker;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct UnlockPosition<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(address = locker.nft_mint)]
    pub nft_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [
            user.key().as_ref(), 
            nft_mint.key().as_ref(),
            Locker::SEEDS
        ],
        bump,
        close = user,
    )]
    pub locker: Account<'info, Locker>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = locker,
        constraint = nft_vault.amount == 1 @ ErrorCode::TokenAccountEmpty,
    )]
    pub nft_vault: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = nft_mint,
        associated_token::authority = user,
    )]
    pub nft_token_account: Account<'info, TokenAccount>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn unlock_position_handler(ctx: Context<UnlockPosition>) -> Result<()> {
    let current_time:u64 = Clock::get()?.unix_timestamp.try_into().unwrap();
    
    require!(
        ctx.accounts.locker.unlock_time < current_time,
        ErrorCode::NotUnlocked
    );

    let user_key = ctx.accounts.user.key();
    let nft_mint_key = ctx.accounts.nft_mint.key();
    
    let signer : &[&[&[u8]]]= &[&[
        user_key.as_ref(), 
        nft_mint_key.as_ref(), 
        Locker::SEEDS,
        &[ctx.bumps.locker]
    ]];

    let transfer_cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.nft_vault.to_account_info(),
            to: ctx.accounts.nft_token_account.to_account_info(),
            authority: ctx.accounts.locker.to_account_info(),
        }
    ).with_signer(signer);

    token::transfer(transfer_cpi_ctx, 1)?;

    let close_account_cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.nft_vault.to_account_info(),
            authority: ctx.accounts.locker.to_account_info(),
            destination: ctx.accounts.user.to_account_info()
        }
    ).with_signer(signer);

    token::close_account(close_account_cpi_ctx)?;

    Ok(())
}