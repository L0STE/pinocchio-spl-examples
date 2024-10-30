use pinocchio::{
    account_info::AccountInfo, ProgramResult, program_error::ProgramError,
};

use pinocchio_token::instructions::InitilizeMint2;

pub fn initialize_mint_2(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [mint, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let decimals = data[0];    
    let mint_authority = unsafe { *(data.as_ptr().add(1) as *mut [u8; 32]) };

    InitilizeMint2 { 
        mint,
        decimals,
        mint_authority,
        freeze_authority: Some(mint_authority),
    }.invoke()?;
   
    Ok(())
}
