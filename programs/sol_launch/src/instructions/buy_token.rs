use crate::{buyer_account::BuyerAccount, pool_account::PoolAccount, ErrorMessage, POOL_SEED};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, TokenAccount, TokenInterface, TransferChecked};

#[event]
pub struct BuyTokenEvent {
    pub buyer: Pubkey,
    pub pool: Pubkey,
    pub amount: u64,
}

#[derive(Accounts)]
pub struct BuyToken<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut, seeds = [POOL_SEED, pool_account.token_pub.as_ref()], bump)]
    pub pool_account: Account<'info, PoolAccount>,
    #[account(mut)]
    pub buyer_account: Account<'info, BuyerAccount>,
    #[account(mut)]
    pub buyer_token_account: InterfaceAccount<'info, TokenAccount>,
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
        ctx.accounts.pool_account.tokens_for_sale > ctx.accounts.pool_account.tokens_sold + amount,
        ErrorMessage::NotEnoughTokensForSale
    );

    ctx.accounts.pool_account.tokens_sold += amount;

    let buyer_account: &mut Account<'_, BuyerAccount> = &mut ctx.accounts.buyer_account;
    buyer_account.pool = ctx.accounts.pool_account.key();
    buyer_account.token_amount += amount;
    buyer_account.token_pub = ctx.accounts.pool_account.token_pub.key();
    buyer_account.purchase_pub = ctx.accounts.pool_account.purchase_token_pub.key();

    emit!(BuyTokenEvent {
        buyer: ctx.accounts.buyer.key(),
        pool: ctx.accounts.pool_account.key(),
        amount,
    });

    Ok(())
}
