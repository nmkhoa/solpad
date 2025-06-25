use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct BuyerAccount {
    pub token_amount: u64,
    pub token_decimals: u8,
    pub token_address: Pubkey,
    pub currency: Pubkey,
}

impl BuyerAccount {
    pub const LEN: usize = 8 + BuyerAccount::INIT_SPACE;
}
