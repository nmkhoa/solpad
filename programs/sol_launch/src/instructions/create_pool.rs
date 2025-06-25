use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::clock;

use crate::{
    config_account::ConfigAccount, pool_account::PoolAccount, ErrorMessage, CONFIG_SEED, POOL_SEED,
};

#[derive(Accounts)]
#[instruction(
    start_time: u64,
    end_time: u64,
    claim_time: u64,
    tokens_for_sale: u64,
    token_decimals: u8,
    token_rate: u64,
    decimals: u8,
    currency: Pubkey,
    token: Pubkey,
    signer: Pubkey
)]
pub struct CreatePool<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = PoolAccount::LEN,
        seeds = [POOL_SEED, token.key().as_ref()],
        bump
    )]
    pub pool_account: Account<'info, PoolAccount>,
    #[account(
        seeds = [CONFIG_SEED],
        constraint = config_account.creator.key() == signer.key() @ErrorMessage::Unauthorized ,
        bump
    )]
    pub config_account: Account<'info, ConfigAccount>,
    // System program
    pub system_program: Program<'info, System>,
}

pub fn process_create_pool(
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
    let clock = Clock::get()?;
    let now = clock.unix_timestamp as u64;
    require!(start_time > now, ErrorMessage::InvalidTime);
    require!(start_time < end_time, ErrorMessage::InvalidTime);
    require!(end_time < claim_time, ErrorMessage::InvalidTime);

    let pool_account = &mut ctx.accounts.pool_account;
    pool_account.start_time = start_time;
    pool_account.end_time = end_time;
    pool_account.claim_time = claim_time;
    pool_account.tokens_for_sale = tokens_for_sale;
    pool_account.token_decimals = token_decimals;
    pool_account.token_rate = token_rate;
    pool_account.decimals = decimals;
    pool_account.currency = currency.key();
    pool_account.currency_decimal = currency_decimal;
    pool_account.token = token.key();
    pool_account.signer = signer.key();
    Ok(())
}
