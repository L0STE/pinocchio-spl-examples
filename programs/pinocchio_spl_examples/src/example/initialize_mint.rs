use pinocchio::{
    account_info::AccountInfo, ProgramResult, program_error::ProgramError,
};

use pinocchio_token::instructions::InitilizeMint;

pub fn initialize_mint(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [mint, rent_sysvar, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let decimals = data[0];    
    let mint_authority = unsafe { *(data.as_ptr().add(1) as *mut [u8; 32]) };

    InitilizeMint { 
        mint,
        rent_sysvar,
        decimals,
        mint_authority: &mint_authority,
        freeze_authority: Some(&mint_authority),
        
    }.invoke()?;
   
    Ok(())
}
