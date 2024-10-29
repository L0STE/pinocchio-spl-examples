use pinocchio::{
    account_info::AccountInfo, ProgramResult, program_error::ProgramError, pubkey::Pubkey,
};

use pinocchio_token::instructions::InitilizeMint2;

pub fn initialize_mint_2(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [mint, mint_2, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let decimals = data[21];    
    let mint_authority = unsafe { *(data.as_ptr().add(1) as *mut Pubkey) };
    let freeze_authority = unsafe { *(data.as_ptr().add(33) as *mut Pubkey) };

    InitilizeMint2 { 
        mint,
        decimals,
        mint_authority,
        freeze_authority: None,
        
    }.invoke()?;

    InitilizeMint2 { 
        mint: mint_2,
        decimals,
        mint_authority,
        freeze_authority: Some(freeze_authority),
        
    }.invoke()?;
   
    Ok(())
}
