use crate::state::Escrow;
use pinocchio::{
    self, ProgramResult,
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::find_program_address,
};
use pinocchio_token::{
    instructions::{CloseAccount, Transfer},
    state::TokenAccount,
};

pub fn take_instruction(accounts: &[AccountInfo], _data: &[u8]) -> ProgramResult {
    let [
        taker,
        maker,
        mint_a,
        mint_b,
        taker_ata_a,
        taker_ata_b,
        maker_ata_b,
        vault,
        escrow,
        _token_program,
        _system_program,
    ] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let escrow_account = Escrow::from_account_info_unchecked(escrow);
    assert_eq!(escrow_account.mint_a, *mint_a.key());
    assert_eq!(escrow_account.mint_b, *mint_b.key());

    let vault_account = TokenAccount::from_account_info(vault)?;

    let seed = [(b"escrow"), maker.key().as_slice(), &[escrow_account.bump]];
    let seeds = &seed[..];
    let escrow_pda = find_program_address(seeds, &crate::ID).0;
    assert_eq!(*escrow.key(), escrow_pda);

    Transfer {
        from: taker_ata_b,
        to: maker_ata_b,
        authority: taker,
        amount: escrow_account.amount,
    }
    .invoke()?;

    let bump = [escrow_account.bump.to_le()];
    let seed = [
        Seed::from(b"escrow"),
        Seed::from(maker.key()),
        Seed::from(&bump),
    ];
    let seeds = Signer::from(&seed);

    Transfer {
        from: vault,
        to: taker_ata_a,
        authority: escrow,
        amount: vault_account.amount(),
    }
    .invoke_signed(&[seeds.clone()])?;

    CloseAccount {
        account: vault,
        destination: maker,
        authority: escrow,
    }
    .invoke_signed(&[seeds])?;

    unsafe {
        *maker.borrow_mut_lamports_unchecked() += *escrow.borrow_lamports_unchecked();
        *escrow.borrow_mut_lamports_unchecked() = 0;
    }

    Ok(())
}