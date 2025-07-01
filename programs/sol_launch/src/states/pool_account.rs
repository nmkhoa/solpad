use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PoolAccount {
    pub start_time: u64,
    pub end_time: u64,
    pub claim_time: u64,
    pub tokens_for_sale: u64,
    pub tokens_sold: u64,
    pub token_pub: Pubkey,
    pub conversion_rate: u8,
    pub purchase_token_pub: Pubkey,
    pub receiver_token_account: Pubkey,
    pub signer: Pubkey,
}

impl PoolAccount {
    pub const LEN: usize = 8 + PoolAccount::INIT_SPACE;
}
