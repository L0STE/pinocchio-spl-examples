use pinocchio::{
    account_info::AccountInfo, ProgramResult, program_error::ProgramError, pubkey::Pubkey,
};

use pinocchio_token::instructions::InitilizeMint;

pub fn initialize_mint(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [mint, mint_2, rent_sysvar, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let decimals = data[0];    
    let mint_authority = unsafe { *(data.as_ptr().add(1) as *mut Pubkey) };
    let freeze_authority = unsafe { *(data.as_ptr().add(33) as *mut Pubkey) };

    InitilizeMint { 
        mint,
        rent_sysvar,
        decimals,
        mint_authority,
        freeze_authority: None,
        
    }.invoke()?;

    InitilizeMint { 
        mint: mint_2,
        rent_sysvar,
        decimals,
        mint_authority,
        freeze_authority: Some(freeze_authority),
        
    }.invoke()?;
   
    Ok(())
}
