use anchor_lang::prelude::*;

#[account]
pub struct Locker {
    pub user: Pubkey,
    pub nft_mint: Pubkey,
    pub unlock_time: u64,
    pub allow_fee_claim: bool,
}

impl Locker {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 1;
    pub const SEEDS: &'static [u8] = b"locker";

    pub fn init(
        user: Pubkey,
        nft_mint: Pubkey,
        unlock_time: u64,
        allow_fee_claim: bool,
    ) -> Self {
        Self {
            user,
            nft_mint,
            unlock_time,
            allow_fee_claim,
        }
    }
}
