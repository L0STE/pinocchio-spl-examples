use pinocchio::{
    account_info::AccountInfo, ProgramResult, program_error::ProgramError,
};

use pinocchio_token::instructions::ThawAccount;

pub fn thaw_account(accounts: &[AccountInfo]) -> ProgramResult {
    let [token, mint, freeze_authority, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    ThawAccount { 
        token, 
        mint, 
        freeze_authority  
    }.invoke()?;
   
    Ok(())
}
