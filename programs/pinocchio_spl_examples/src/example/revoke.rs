use pinocchio::{
    account_info::AccountInfo, ProgramResult, program_error::ProgramError,
};

use pinocchio_token::instructions::Revoke;

pub fn revoke(accounts: &[AccountInfo]) -> ProgramResult {
    let [token, authority, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    Revoke { 
        token, 
        authority
    }.invoke()?;
   
    Ok(())
}
