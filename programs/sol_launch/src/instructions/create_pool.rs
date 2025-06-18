use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::constants::POOL_SEED;
use crate::error::ErrorCodeApp;
use crate::states::config_account::ConfigAccount;
use crate::states::create_pool::Pool;

#[derive(Accounts)]
#[instruction(
    start_time: i64,
    end_time: i64,
    claim_time: i64,
    total_amount: u64,
    price: u64,
    max_purchase: u64
)]
pub struct CreatePool<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        has_one = creator,
        seeds = [b"config"],
        bump
    )]
    pub config_account: Account<'info, ConfigAccount>,

    #[account(
        init,
        payer = creator,
        space = Pool::LEN,
        seeds = [POOL_SEED, currency.key().as_ref()],
        bump
    )]
    pub pool: Account<'info, Pool>,

    pub currency: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn process_create_pool(
    ctx: Context<CreatePool>,
    start_time: i64,
    end_time: i64,
    claim_time: i64,
    total_amount: u64,
    price: u64,
    max_purchase: u64,
) -> Result<()> {
    require!(start_time < end_time, ErrorCodeApp::InvalidTimeRange);
    require!(end_time < claim_time, ErrorCodeApp::InvalidTimeRange);
    require!(total_amount > 0, ErrorCodeApp::InvalidAmount);
    require!(price > 0, ErrorCodeApp::InvalidPrice);
    require!(
        max_purchase > 0 && max_purchase <= total_amount,
        ErrorCodeApp::InvalidMaxPurchase
    );

    let pool = &mut ctx.accounts.pool;
    pool.start_time = start_time;
    pool.end_time = end_time;
    pool.claim_time = claim_time;
    pool.total_amount = total_amount;
    pool.price = price;
    pool.currency = ctx.accounts.currency.key();
    pool.max_purchase = max_purchase;
    pool.creator = ctx.accounts.creator.key();

    Ok(())
}
