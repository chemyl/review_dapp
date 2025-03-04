pub mod instruction;
pub mod state;

use crate::instruction::ReviewInstruction;
use crate::state::AccountState;
use crate::state::ReviewError;
use borsh::BorshSerialize;
// use borsh::from_slice;
use solana_program::borsh1::try_from_slice_unchecked;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    // msg,
    program::invoke_signed,
    program_error::ProgramError,
    // program_pack::IsInitialized,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{Sysvar, rent::Rent},
};
// use std::clone;
use std::convert::TryInto;

entrypoint!(process_instruction);
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

pub fn add_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    rating: u8,
    description: String,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let system_program_account = next_account_info(account_info_iter)?;

    // check if initializer match Sign
    if !initializer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // make review unique
    let (pda, bump_seed) = Pubkey::find_program_address(
        &[initializer.key.as_ref(), title.as_bytes().as_ref()],
        program_id,
    );

    if pda != *pda_account.key {
        return Err(ProgramError::InvalidArgument);
    }

    if rating > 10 || rating < 1 {
        return Err(ReviewError::InvalidRating.into());
    }

    let account_len: usize = 1000;
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_account.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        &[
            initializer.clone(),
            pda_account.clone(),
            system_program_account.clone(),
        ],
        &[&[
            initializer.key.as_ref(),
            title.as_bytes().as_ref(),
            &[bump_seed],
        ]],
    )?;

    let mut account_data =
        try_from_slice_unchecked::<AccountState>(&pda_account.data.borrow()).unwrap();

    if account_data.is_initialized {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    account_data.title = title;
    account_data.description = description;
    account_data.rating = rating;
    account_data.is_initialized = true;

    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;

    Ok(())
}

pub fn update_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _title: String,
    rating: u8,
    description: String,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;

    if pda_account.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }

    if !initializer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut account_data =
        try_from_slice_unchecked::<AccountState>(&pda_account.data.borrow_mut()).unwrap();

    let (pda, _bump_seed) = Pubkey::find_program_address(
        &[
            initializer.key.as_ref(),
            account_data.title.as_bytes().as_ref(),
        ],
        program_id,
    );

    if pda != *pda_account.key {
        return Err(ReviewError::InvalidPDA.into());
    }

    if !account_data.is_initialized {
        return Err(ReviewError::UninitializedAccount.into());
    }

    if rating > 10 || rating < 1 {
        return Err(ReviewError::InvalidRating.into());
    }

    account_data.description = description;
    account_data.rating = rating;

    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;

    Ok(())
}
