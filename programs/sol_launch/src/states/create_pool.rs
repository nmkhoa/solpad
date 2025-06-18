use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub start_time: i64,
    pub end_time: i64,
    pub claim_time: i64,
    pub total_amount: u64,
    pub price: u64,
    pub currency: Pubkey,
    pub max_purchase: u64,
    pub creator: Pubkey,
    pub bump: u8,
}

impl Pool {
    pub const LEN: usize = 8 + Pool::INIT_SPACE;
}
