use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum IdoError {
    #[error("Invalid Instruction")]
    InvalidInstruction,

}

impl From<IdoError> for ProgramError {
    fn from(e: IdoError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
