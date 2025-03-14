use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

use crate::state::review_payload::ReviewPayload;

pub enum ReviewInstruction {
    AddReview {
        tittle: String,
        rating: u8,
        decription: String,
    },
    UpdateReview {
        tittle: String,
        rating: u8,
        decription: String,
    },
}

impl ReviewInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        let payload = ReviewPayload::try_from_slice(rest).unwrap();
        Ok(match variant {
            0 => Self::AddReview {
                tittle: payload.tittle,
                rating: payload.rating,
                decription: payload.decription,
            },
            1 => Self::UpdateReview {
                tittle: payload.tittle,
                rating: payload.rating,
                decription: payload.decription,
            },
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
