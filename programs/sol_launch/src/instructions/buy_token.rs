use crate::{buyer_account::BuyerAccount, pool_account::PoolAccount, ErrorMessage, POOL_SEED};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

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
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = currency,
        associated_token::authority = buyer
    )]
    pub buyer_token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = currency,
        associated_token::authority = pool_account.signer
    )]
    pub creator_token_account: Account<'info, TokenAccount>,
    /// The mint of the currency used in the pool (must match pool_account.currency)
    pub currency: Account<'info, anchor_spl::token::Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
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

    // Tính toán số lượng SPL token cần chuyển: amount * token_rate * 10^token_decimals
    let token_rate = ctx.accounts.pool_account.token_rate;
    let token_decimals = ctx.accounts.pool_account.token_decimals as u32;
    let decimals_factor = 10u64
        .checked_pow(token_decimals)
        .ok_or(ErrorMessage::MathOverflow)?;
    let spl_token_amount = amount
        .checked_mul(token_rate)
        .and_then(|v| v.checked_mul(decimals_factor))
        .ok_or(ErrorMessage::MathOverflow)?;

    // Chuyển SPL token từ buyer sang creator (creator_token_account)
    let cpi_accounts = Transfer {
        from: ctx.accounts.buyer_token_account.to_account_info(),
        to: ctx.accounts.creator_token_account.to_account_info(),
        authority: ctx.accounts.buyer.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_ctx, spl_token_amount)?;

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
