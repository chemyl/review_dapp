use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::{
    handlers::dapp_instruction_handler::{add_review, update_review},
    instruction::ReviewInstruction,
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = ReviewInstruction::unpack(instruction_data)?;
    match instruction {
        ReviewInstruction::AddReview {
            tittle,
            rating,
            decription,
        } => add_review(program_id, accounts, tittle, rating, decription),
        ReviewInstruction::UpdateReview {
            tittle,
            rating,
            decription,
        } => update_review(program_id, accounts, tittle, rating, decription),
    }
}
