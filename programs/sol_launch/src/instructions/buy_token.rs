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
    #[account(mut)]
    pub receive_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub mint_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub token_program: InterfaceAccount<'info, TokenAccount>,
}

pub fn process_buy_token(ctx: Context<BuyToken>, amount: u64) -> Result<()> {
    let clock = Clock::get()?;
    let now = clock.unix_timestamp as u64;
    let pool_account = &mut ctx.accounts.pool_account;

    require!(
        now >= pool_account.start_time && now <= pool_account.end_time,
        ErrorMessage::InvalidTime
    );

    require!(amount > 0, ErrorMessage::InvalidTokenAmount);
    require!(
        pool_account.tokens_for_sale > pool_account.tokens_sold + amount,
        ErrorMessage::NotEnoughTokensForSale
    );

    let decimals = pool_account.purchase_token_decimal;
    let transfer_amount = amount
        .saturating_mul(decimals as u64)
        .saturating_mul(pool_account.conversion_rate as u64);
    let cpi_accounts = TransferChecked {
        mint: ctx.accounts.mint_token_account.to_account_info(),
        from: ctx.accounts.buyer_token_account.to_account_info(),
        to: ctx.accounts.receive_token_account.to_account_info(),
        authority: ctx.accounts.buyer.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    token_interface::transfer_checked(cpi_context, transfer_amount, decimals)?;

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
