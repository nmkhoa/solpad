use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;

pub use constants::*;
pub use errors::*;
pub use instructions::{create_pool::*, initialize::*};
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
        token_decimals: u8,
        token_rate: u64,
        decimals: u8,
        currency: Pubkey,
        currency_decimal: u8,
        token: Pubkey,
        signer: Pubkey,
    ) -> Result<()> {
        process_create_pool(
            ctx,
            start_time,
            end_time,
            claim_time,
            tokens_for_sale,
            token_decimals,
            token_rate,
            decimals,
            currency,
            currency_decimal,
            token,
            signer,
        )
    }
}
