use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;

pub use constants::*;
pub use errors::*;
pub use instructions::{buy_token::*, create_pool::*, initialize::*};
pub use states::*;

declare_id!("ESn9HUCzhjXYqSUpMDiSs1JV4znknv4LvHQFxUL3fTBG");

#[program]
pub mod sol_launch {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, owner: Pubkey, creator: Pubkey) -> Result<()> {
        process_initialize(ctx, owner, creator)
    }

    pub fn creator_create_pool(
        ctx: Context<CreatePool>,
        start_time: u64,
        end_time: u64,
        claim_time: u64,
        tokens_for_sale: u64,
        tokens_sold: u64,
        token_pub: Pubkey,
        conversion_rate: u8,
        purchase_pub: Pubkey,
        signer: Pubkey,
    ) -> Result<()> {
        process_create_pool(
            ctx,
            start_time,
            end_time,
            claim_time,
            tokens_for_sale,
            tokens_sold,
            token_pub,
            conversion_rate,
            purchase_pub,
            signer,
        )
    }

    pub fn buy_token(ctx: Context<BuyToken>, amount: u64) -> Result<()> {
        process_buy_token(ctx, amount)
    }
}
