use solana_program::program_error::ProgramError;
use thiserror::{self, Error};

#[derive(Debug, Error)]
pub enum ReviewError {
    #[error("Account not initialized yet")]
    UninitializedAccount,

    #[error("Rating is invalid")]
    InvalidRating,

    #[error("PDA Error")]
    InvalidPDA,
}

// Wraping ReviewError to ProgramError
impl From<ReviewError> for ProgramError {
    fn from(e: ReviewError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
