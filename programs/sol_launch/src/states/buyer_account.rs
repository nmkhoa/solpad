use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct BuyerAccount {
    pub pool: Pubkey,
    pub token_amount: u64,
    pub token_pub: Pubkey,
    pub purchase_pub: Pubkey,
}

impl BuyerAccount {
    pub const LEN: usize = 8 + BuyerAccount::INIT_SPACE;
}
