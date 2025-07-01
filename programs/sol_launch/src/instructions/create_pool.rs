use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{self, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{
    config_account::ConfigAccount, pool_account::PoolAccount, ErrorMessage, CONFIG_SEED, POOL_SEED,
};

#[derive(Accounts)]
#[instruction(
    start_time: u64,
    end_time: u64,
    claim_time: u64,
    tokens_for_sale: u64,
    tokens_sold: u64,
    token_pub: Pubkey,
    conversion_rate: u8,
    purchase_pub: Pubkey,
    signer: Pubkey,
)]
pub struct CreatePool<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = PoolAccount::LEN,
        seeds = [POOL_SEED, purchase_pub.key().as_ref()],
        bump
    )]
    pub pool_account: Account<'info, PoolAccount>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    pub receiver_token_account: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(
        seeds = [CONFIG_SEED],
        constraint = config_account.creator.key() == signer.key() @ErrorMessage::Unauthorized ,
        bump
    )]
    pub config_account: Account<'info, ConfigAccount>,
    pub system_program: Program<'info, System>,
}

pub fn process_create_pool(
    ctx: Context<CreatePool>,
    start_time: u64,
    end_time: u64,
    claim_time: u64,
    tokens_for_sale: u64,
    tokens_sold: u64,
    token_pub: Pubkey,
    conversion_rate: u8,
    purchase_token_pub: Pubkey,
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
    pool_account.tokens_sold = tokens_sold;
    pool_account.token_pub = token_pub;
    pool_account.conversion_rate = conversion_rate;
    pool_account.purchase_token_pub = purchase_token_pub.key();
    pool_account.receiver_token_account = ctx.accounts.receiver_token_account.key();
    pool_account.signer = signer.key();
    Ok(())
}
