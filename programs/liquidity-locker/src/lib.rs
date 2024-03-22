use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;

use crate::instructions::*;
declare_id!("HxXkV8UgTmgQmu3bvGUQkXM6V9JwMJeVSSB5JP2QFiRb");

#[program]
pub mod liquidity_locker {
    use super::*;

    pub fn claim_fees(ctx: Context<ClaimFees>,) -> Result<()> {
        claim_fees_handler(ctx)
    }

    pub fn lock_position(ctx: Context<LockPosition>, duration:u32, allow_fee_claim:bool) -> Result<()> {
        lock_position_handler(ctx, duration, allow_fee_claim)
    }

    pub fn unlock_position(ctx: Context<UnlockPosition>,) -> Result<()> {
        unlock_position_handler(ctx)
    }
}
