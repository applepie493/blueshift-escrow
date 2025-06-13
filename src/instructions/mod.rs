pub mod make;
pub mod refund;
pub mod take;

pub use make::*;
pub use refund::*;
pub use take::*;

use pinocchio::program_error::ProgramError;

pub enum EscrowCheck {
    Make = 0,
    Take = 1,
    Refund = 2,
}

impl TryFrom<&u8> for EscrowCheck {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(EscrowCheck::Make),
            1 => Ok(EscrowCheck::Take),
            2 => Ok(EscrowCheck::Refund),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}