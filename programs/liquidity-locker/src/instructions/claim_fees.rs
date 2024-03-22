use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, Mint, TokenAccount };
use anchor_spl::token_interface::Token2022;

use raydium_amm_v3::states::{PoolState, PersonalPositionState, ProtocolPositionState, TickArrayState};
use raydium_amm_v3::program::AmmV3;
use raydium_amm_v3;

use crate::state::Locker;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct ClaimFees<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(address = locker.nft_mint)]
    pub nft_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [
            user.key().as_ref(), 
            locker.nft_mint.key().as_ref(),
            Locker::SEEDS
        ],
        bump,
    )]
    pub locker: Account<'info, Locker>,

    #[account(mut)]
    pub pool_state: AccountLoader<'info, PoolState>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = locker,
        constraint = nft_vault.amount == 1 @ ErrorCode::TokenAccountEmpty,
    )]
    pub nft_vault: Account<'info, TokenAccount>,

    #[account(
        mut, 
        constraint = personal_position.pool_id == pool_state.key(),
        constraint = personal_position.nft_mint == nft_mint.key()
    )]
    pub personal_position: Box<Account<'info, PersonalPositionState>>,

    #[account(
        mut, 
        constraint = protocol_position.pool_id == pool_state.key(),
    )]
    pub protocol_position: Box<Account<'info, ProtocolPositionState>>,

    #[account(
        mut,
        constraint = token_vault_0.key() == pool_state.load()?.token_vault_0
    )]
    pub token_vault_0: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = token_vault_1.key() == pool_state.load()?.token_vault_1
    )]
    pub token_vault_1: Box<Account<'info, TokenAccount>>,

    #[account(mut, constraint = tick_array_lower.load()?.pool_id == pool_state.key())]
    pub tick_array_lower: AccountLoader<'info, TickArrayState>,

    #[account(mut, constraint = tick_array_upper.load()?.pool_id == pool_state.key())]
    pub tick_array_upper: AccountLoader<'info, TickArrayState>,

    #[account(
        mut,
        token::mint = token_vault_0.mint,
    )]
    pub recipient_token_account_0: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        token::mint = token_vault_1.mint,
    )]
    pub recipient_token_account_1: Box<Account<'info, TokenAccount>>,

    #[account(
        address = token_vault_0.mint
    )]
    pub vault_0_mint: Box<Account<'info, Mint>>,

    #[account(
        address = token_vault_1.mint
    )]
    pub vault_1_mint: Box<Account<'info, Mint>>,

    /// CHECK:
    #[account(
        address = spl_memo::id()
    )]
    pub memo_program: UncheckedAccount<'info>,

    pub raydium_clmm_program: Program<'info, AmmV3>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
    pub token_program_2022: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

pub fn claim_fees_handler(ctx: Context<ClaimFees>) -> Result<()> {
    let locker = &ctx.accounts.locker;

    require!(locker.allow_fee_claim, ErrorCode::ClaimNotAllowed);

    let user_key = ctx.accounts.user.key();
    let nft_mint_key = ctx.accounts.nft_mint.key();

    let signer : &[&[&[u8]]]= &[&[
        user_key.as_ref(), 
        nft_mint_key.as_ref(), 
        Locker::SEEDS,
        &[ctx.bumps.locker]
    ]];

    let cpi_accounts = raydium_amm_v3::cpi::accounts::DecreaseLiquidityV2 {
        nft_owner:ctx.accounts.locker.to_account_info(),
        nft_account:ctx.accounts.nft_vault.to_account_info(),
        personal_position:ctx.accounts.personal_position.to_account_info(),
        pool_state:ctx.accounts.pool_state.to_account_info(),
        protocol_position:ctx.accounts.protocol_position.to_account_info(),
        token_vault_0:ctx.accounts.token_vault_0.to_account_info(),
        token_vault_1:ctx.accounts.token_vault_1.to_account_info(),
        tick_array_lower:ctx.accounts.tick_array_lower.to_account_info(),
        tick_array_upper:ctx.accounts.tick_array_upper.to_account_info(),
        recipient_token_account_0:ctx.accounts.recipient_token_account_0.to_account_info(),
        recipient_token_account_1:ctx.accounts.recipient_token_account_1.to_account_info(),
        token_program:ctx.accounts.token_program.to_account_info(),
        token_program_2022:ctx.accounts.token_program_2022.to_account_info(),
        memo_program:ctx.accounts.memo_program.to_account_info(),
        vault_0_mint:ctx.accounts.vault_0_mint.to_account_info(),
        vault_1_mint:ctx.accounts.vault_1_mint.to_account_info()
    };

    let cpi_ctx = CpiContext::new(ctx.accounts.raydium_clmm_program.to_account_info(),cpi_accounts).with_signer(signer);
    raydium_amm_v3::cpi::decrease_liquidity_v2(cpi_ctx, 0, 0, 0)?;

    Ok(())
}