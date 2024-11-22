use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Not owner")]
    NotOwner,
    #[msg("Incorrect Opponent")]
    InvalidDefender,
    #[msg("Invalid points")]
    InvalidPoints,
}

impl From<CustomError> for ProgramError {
    fn from(e: CustomError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
