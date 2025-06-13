use instructions::EscrowCheck;
use pinocchio::{
    ProgramResult, account_info::AccountInfo, entrypoint, program_error::ProgramError,
    pubkey::Pubkey,
};

mod instructions;
mod state;

use instructions::*;
use state::*;

#[cfg(not(feature = "no-entrypoint"))]

entrypoint!(process_instruction);

pinocchio_pubkey::declare_id!("C9AJuLQAM4igQxsb9wvY4mRD2wahitWXaS8zb4RKqfoG");

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    assert_eq!(program_id, &crate::ID);

    let (discriminator, data) = data.split_first().ok_or(ProgramError::InvalidAccountData)?;

    match EscrowCheck::try_from(discriminator)? {
        EscrowCheck::Make => make_instruction(accounts, data)?,
        EscrowCheck::Take => take_instruction(accounts, data)?,
        EscrowCheck::Refund => refund_instruction(accounts, data)?,
    }

    Ok(())
}