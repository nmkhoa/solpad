use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod states;

pub use constants::*;
pub use error::*;
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
        start_time: i64,
        end_time: i64,
        claim_time: i64,
        total_amount: u64,
        price: u64,
        max_purchase: u64,
    ) -> Result<()> {
        process_create_pool(
            ctx,
            start_time,
            end_time,
            claim_time,
            total_amount,
            price,
            max_purchase,
        )
    }
}
