use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorMessage {
    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("Invalid time")]
    InvalidTime,

    #[msg("Invalid token amount")]
    InvalidTokenAmount,

    #[msg("Not enough tokens for sale")]
    NotEnoughTokensForSale,

    #[msg("Math overflow")]
    MathOverflow,
}
