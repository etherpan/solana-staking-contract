use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Owner mismatch")]
    OwnerMismatch,

    #[msg("Token A mint mismatch")]
    TokenAMintMismatch,

    #[msg("Token B mint mismatch")]
    TokenBMintMismatch
}