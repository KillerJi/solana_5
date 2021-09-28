use crate::error::IdoError::InvalidInstruction;
use solana_program::{program_error::ProgramError,msg};

pub enum Instruction { 
    Helloworld,
} 
impl Instruction { 
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> { 
        let (tag, _rest) = input.split_first().ok_or(InvalidInstruction)?; 
        msg!("tag {:?}",tag);
        Ok(match tag { 
            0 => Instruction::Helloworld, 
            _ => return Err(InvalidInstruction.into()),
        } )
    }
}