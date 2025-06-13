use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};

#[repr(C)]
pub struct Escrow {
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub amount: u64,
    pub bump: u8,
}

impl Escrow {
    pub const LEN: usize = 32 + 32 + 32 + 8 + 1;

    pub fn from_account_info_unchecked(account_info: &AccountInfo) -> &mut Self {
        unsafe { &mut *(account_info.borrow_mut_data_unchecked().as_ptr() as *mut Self) }
    }
}