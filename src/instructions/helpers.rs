/*
 * =============================================================================
 * ファイル: src/instructions/helpers.rs
 * 説明: 全インストラクション共通のヘルパー
 * =============================================================================
 */
 use pinocchio::{
    account_info::AccountInfo, program::invoke_signed, pubkey::Pubkey, ProgramResult,
    instruction::Instruction,
};

// チュートリアルで示されたヘルパー群を実装
pub struct Transfer<'a> {
    pub from: &'a AccountInfo,
    pub to: &'a AccountInfo,
    pub authority: &'a AccountInfo,
    pub amount: u64,
}

impl<'a> Transfer<'a> {
    pub fn invoke(&self) -> ProgramResult {
        let ix = pinocchio_token::instruction::transfer(
            &pinocchio_token::ID,
            self.from.key,
            self.to.key,
            self.authority.key,
            &[],
            self.amount,
        )?;
        pinocchio::program::invoke(&ix, &[self.from.clone(), self.to.clone(), self.authority.clone()])
    }

    pub fn invoke_signed(&self, seeds: &[&[&[u8]]]) -> ProgramResult {
        let ix = pinocchio_token::instruction::transfer(
            &pinocchio_token::ID,
            self.from.key,
            self.to.key,
            self.authority.key,
            &[],
            self.amount,
        )?;
        invoke_signed(&ix, &[self.from.clone(), self.to.clone(), self.authority.clone()], seeds)
    }
}

pub struct CloseAccount<'a> {
    pub account: &'a AccountInfo,
    pub destination: &'a AccountInfo,
    pub authority: &'a AccountInfo,
}

impl<'a> CloseAccount<'a> {
    pub fn invoke_signed(&self, seeds: &[&[&[u8]]]) -> ProgramResult {
        let ix = pinocchio_token::instruction::close_account(
            &pinocchio_token::ID,
            self.account.key,
            self.destination.key,
            self.authority.key,
            &[],
        )?;
        invoke_signed(&ix, &[self.account.clone(), self.destination.clone(), self.authority.clone()], seeds)
    }
}

// チュートリアルにはないが、アカウント作成に必要なヘルパー
pub fn create_pda_account<'a>(
    from: &AccountInfo<'a>,
    to: &AccountInfo<'a>,
    space: usize,
    owner: &Pubkey,
    system_program: &AccountInfo<'a>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let rent = pinocchio::sysvar::rent::Rent::get()?;
    let ix = pinocchio::system_instruction::create_account(
        from.key,
        to.key,
        rent.minimum_balance(space),
        space as u64,
        owner,
    );
    invoke_signed(&ix, &[from.clone(), to.clone(), system_program.clone()], seeds)
}