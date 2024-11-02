use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

use pinocchio_token::instructions::InitilizeAccount2;

pub fn initialize_account_2(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [token, mint, rent_sysvar, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Not sure about this one. *const or *mut ?
    let owner = unsafe { *(data.as_ptr() as *const Pubkey) };

    InitilizeAccount2 {
        token,
        mint,
        rent_sysvar,
        owner: &owner,
    }
    .invoke()?;

    Ok(())
}
