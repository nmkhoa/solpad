use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCodeApp {
    #[msg("Invalid time range")]
    InvalidTimeRange,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Invalid price")]
    InvalidPrice,
    #[msg("Invalid max purchase amount")]
    InvalidMaxPurchase,
    #[msg("Pool not active")]
    PoolNotActive,
    #[msg("Pool already ended")]
    PoolEnded,
    #[msg("Not enough tokens")]
    InsufficientTokens,
    #[msg("Exceeded max purchase limit")]
    ExceededMaxPurchase,
    #[msg("Not authorized")]
    NotAuthorized,
    #[msg("Claim not available yet")]
    ClaimNotAvailable,
}
