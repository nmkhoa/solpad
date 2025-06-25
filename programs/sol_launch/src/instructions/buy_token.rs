use crate::{buyer_account::BuyerAccount, pool_account::PoolAccount, ErrorMessage, POOL_SEED};
use anchor_lang::prelude::*;

#[event]
pub struct BuyTokenEvent {
    pub buyer: Pubkey,
    pub pool: Pubkey,
    pub currency: Pubkey,
    pub amount: u64,
}

#[derive(Accounts)]
pub struct BuyToken<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut, seeds = [POOL_SEED, pool_account.token.as_ref()], bump)]
    pub pool_account: Account<'info, PoolAccount>,
    #[account(mut)]
    pub buyer_account: Account<'info, BuyerAccount>,
}

pub fn process_buy_token(ctx: Context<BuyToken>, amount: u64) -> Result<()> {
    let clock = Clock::get()?;
    let now = clock.unix_timestamp as u64;

    require!(
        now >= ctx.accounts.pool_account.start_time && now <= ctx.accounts.pool_account.end_time,
        ErrorMessage::InvalidTime
    );

    require!(amount > 0, ErrorMessage::InvalidTokenAmount);
    require!(
        ctx.accounts.pool_account.tokens_for_sale >= amount,
        ErrorMessage::NotEnoughTokensForSale
    );

    ctx.accounts.pool_account.tokens_for_sale -= amount;

    let buyer_account: &mut Account<'_, BuyerAccount> = &mut ctx.accounts.buyer_account;
    buyer_account.pool = ctx.accounts.pool_account.key();
    buyer_account.token_amount += amount;
    buyer_account.token_decimals = ctx.accounts.pool_account.token_decimals;
    buyer_account.currency = ctx.accounts.pool_account.currency;
    buyer_account.token_address = ctx.accounts.pool_account.token;

    emit!(BuyTokenEvent {
        buyer: ctx.accounts.buyer.key(),
        pool: ctx.accounts.pool_account.key(),
        currency: ctx.accounts.pool_account.currency,
        amount,
    });

    Ok(())
}
