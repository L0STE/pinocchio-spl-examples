use pinocchio::{
    account_info::AccountInfo, ProgramResult, program_error::ProgramError,
};

use pinocchio_token::instructions::MintToChecked;

pub fn mint_to_checked(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [mint, token, mint_authority, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let amount = unsafe { *(data.as_ptr() as *const u64) };
    let decimals = unsafe { *(data.as_ptr().add(8) as *const u8) };

    MintToChecked { 
        mint, 
        token, 
        mint_authority,
        amount,
        decimals,
    }.invoke()?;
   
    Ok(())
}
