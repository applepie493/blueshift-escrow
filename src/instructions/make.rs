use crate::state::Escrow;
use pinocchio::{
    ProgramResult,
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey,
    sysvars::{Sysvar, rent::Rent},
};
use pinocchio_log::log;
use pinocchio_token::state::TokenAccount;

pub fn make_instruction(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [
        maker,
        mint_a,
        mint_b,
        maker_ata,
        vault,
        escrow,
        _system_program,
        _token_program,
    ] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let bump = unsafe { *(data.as_ptr() as *const u8) }.to_le_bytes();
    let seed = [(b"escrow"), maker.key().as_slice(), bump.as_ref()];
    let seeds = &seed[..];

    let pda = pubkey::checked_create_program_address(seeds, &crate::ID).unwrap();
    assert_eq!(&pda, escrow.key());

    assert_eq!(unsafe { mint_a.owner() }, &pinocchio_token::ID);
    assert_eq!(unsafe { mint_b.owner() }, &pinocchio_token::ID);

    assert!(unsafe {
        TokenAccount::from_account_info_unchecked(vault)
            .unwrap()
            .owner()
            == escrow.key()
    });

    if unsafe { escrow.owner() } != &crate::ID {
        log!("Creating Escrow Account");

        // Creating Escrow Account
        pinocchio_system::instructions::CreateAccount {
            from: maker,
            to: escrow,
            lamports: Rent::get()?.minimum_balance(Escrow::LEN),
            space: Escrow::LEN as u64,
            owner: &crate::ID,
        }
        .invoke()?;

        // Populate Escrow Account
        let escrow_account = Escrow::from_account_info_unchecked(&escrow);

        escrow_account.maker = *maker.key();
        escrow_account.mint_a = *mint_a.key();
        escrow_account.mint_b = *mint_b.key();
        escrow_account.amount = unsafe { *(data.as_ptr().add(1) as *const u64) };
        escrow_account.bump = unsafe { *data.as_ptr() };

        log!("Amount: {}", unsafe {
            *(data.as_ptr().add(1 + 8) as *const u64)
        });

        // Transfer mint_a (token being offered) from user ata to vault
        pinocchio_token::instructions::Transfer {
            from: maker_ata,
            to: vault,
            authority: maker,
            amount: unsafe { *(data.as_ptr().add(1 + 8) as *const u64) },
        }
        .invoke()?;
    } else {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    Ok(())
}